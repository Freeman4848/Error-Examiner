use regex::RegexBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashSet, sync::OnceLock};

#[derive(Deserialize)]
struct Pipeline {
    classifier: Classifier,
}

#[derive(Deserialize)]
struct Classifier {
    formats: Vec<Format>,
}

#[derive(Deserialize)]
struct Format {
    id: String,
    record_kind: String,
    parser_id: String,
    match_groups: Vec<MatchGroup>,
}

#[derive(Deserialize)]
struct MatchGroup {
    detect: DetectionRules,
}

#[derive(Deserialize)]
struct DetectionRules {
    required: Vec<Predicate>,
    any: Vec<Predicate>,
    forbidden: Vec<Predicate>,
}

#[derive(Deserialize)]
struct Predicate {
    #[serde(rename = "type")]
    kind: String,
    value: String,
}

#[derive(Debug)]
pub(crate) struct Detection {
    pub(crate) format_ids: Vec<String>,
    pub(crate) supported: bool,
}

pub(crate) fn detect(input: &str) -> Detection {
    let catalog = catalog();
    let trimmed = input.trim();
    let json = serde_json::from_str::<Value>(trimmed).ok();
    let records: Vec<&Value> = match json.as_ref() {
        Some(Value::Array(values)) => values.iter().collect(),
        Some(value @ Value::Object(_)) => vec![value],
        _ => Vec::new(),
    };
    let mut matches = Vec::new();
    if records.is_empty() {
        if let Some(values) = parse_ndjson(trimmed) {
            for value in &values {
                classify(catalog, "json-record", Some(value), trimmed, &mut matches);
            }
        } else {
            classify(catalog, "text-record", None, trimmed, &mut matches);
        }
    } else {
        for record in records {
            classify(
                catalog,
                "json-record",
                Some(record),
                &record.to_string(),
                &mut matches,
            );
        }
    }
    let mut seen = HashSet::new();
    matches.retain(|item: &(String, String, usize)| seen.insert(item.0.clone()));
    let supported = !matches.is_empty()
        && matches
            .iter()
            .all(|(_, parser, _)| supported_parser(parser));
    Detection {
        format_ids: matches.into_iter().map(|(id, _, _)| id).collect(),
        supported,
    }
}

fn classify(
    catalog: &Pipeline,
    record_kind: &str,
    value: Option<&Value>,
    text: &str,
    output: &mut Vec<(String, String, usize)>,
) {
    let best = catalog
        .classifier
        .formats
        .iter()
        .filter(|format| format.record_kind == record_kind)
        .filter_map(|format| match_score(format, value, text).map(|score| (format, score)))
        .max_by_key(|(_, score)| *score);
    if let Some((format, score)) = best {
        output.push((format.id.clone(), format.parser_id.clone(), score));
    }
}

fn match_score(format: &Format, value: Option<&Value>, text: &str) -> Option<usize> {
    if format.match_groups.is_empty() {
        return Some(0);
    }
    let mut score = 0;
    for group in &format.match_groups {
        if group
            .detect
            .required
            .iter()
            .any(|rule| !predicate_matches(rule, value, text))
            || group
                .detect
                .forbidden
                .iter()
                .any(|rule| predicate_matches(rule, value, text))
        {
            return None;
        }
        let any_matches = group
            .detect
            .any
            .iter()
            .filter(|rule| predicate_matches(rule, value, text))
            .count();
        if group.detect.required.is_empty() && !group.detect.any.is_empty() && any_matches == 0 {
            return None;
        }
        score += group.detect.required.len() * 3 + any_matches + 1;
    }
    Some(score)
}

fn predicate_matches(rule: &Predicate, value: Option<&Value>, text: &str) -> bool {
    match rule.kind.as_str() {
        "json_path_exists" => value.is_some_and(|value| json_path_exists(value, &rule.value)),
        "regex" => RegexBuilder::new(&rule.value)
            .multi_line(true)
            .build()
            .is_ok_and(|regex| regex.is_match(text)),
        _ => false,
    }
}

fn json_path_exists(value: &Value, path: &str) -> bool {
    path.strip_prefix("$.").is_some_and(|path| {
        path.split('.')
            .try_fold(value, |item, key| item.get(key))
            .is_some()
    })
}

fn parse_ndjson(input: &str) -> Option<Vec<Value>> {
    let values: Option<Vec<Value>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).ok())
        .collect();
    values.filter(|values| values.len() > 1)
}

fn supported_parser(parser: &str) -> bool {
    matches!(
        parser,
        "cargo-rustc-parser"
            | "gcp-http-request-parser"
            | "gcp-json-payload-parser"
            | "gcp-text-payload-parser"
            | "gcp-proto-audit-parser"
            | "gcp-ndjson-parser"
            | "generic-json-parser"
            | "generic-json-array-parser"
    )
}

fn catalog() -> &'static Pipeline {
    static CATALOG: OnceLock<Pipeline> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../parser-catalog/parser-pipeline-v2.json"))
            .expect("embedded parser pipeline must be valid")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_rust_and_mixed_gcp_logs() {
        let rust = detect(include_str!("../fixtures/broken-rust/broken-rust.log"));
        assert!(rust.supported, "{rust:?}");
        assert!(rust.format_ids.contains(&"cargo-rustc-error".to_owned()));

        let cloud = r#"[
          {"insertId":"1","logName":"x","httpRequest":{"status":200}},
          {"insertId":"2","logName":"x","protoPayload":{"@type":"type.googleapis.com/google.cloud.audit.AuditLog"}},
          {"insertId":"3","logName":"x","textPayload":"INFO request complete"}
        ]"#;
        let cloud = detect(cloud);
        assert!(cloud.supported);
        assert!(cloud.format_ids.contains(&"gcp-http-request".to_owned()));
        assert!(cloud
            .format_ids
            .contains(&"gcp-proto-payload-audit".to_owned()));
        assert!(cloud.format_ids.contains(&"gcp-text-payload".to_owned()));
    }
}
