use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock, RwLockReadGuard},
};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Schema {
    pub(crate) schema_version: u32,
    pub(crate) fallback: String,
    pub(crate) formats: Vec<FormatSchema>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct FormatSchema {
    pub(crate) id: String,
    pub(crate) record_kind: String,
    pub(crate) detect: Detect,
    pub(crate) parser: ParserSpec,
    pub(crate) validation: Validation,
    #[serde(default)]
    pub(crate) application: Option<String>,
    #[serde(default)]
    pub(crate) catalog_rank: Option<usize>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Detect {
    pub(crate) all: Vec<Condition>,
    pub(crate) any: Vec<Condition>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Condition {
    pub(crate) kind: String,
    pub(crate) value: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Validation {
    pub(crate) require_events: bool,
    pub(crate) require_content: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum ParserSpec {
    TextBlocks {
        start_regex: String,
        severity: String,
        preamble_severity: String,
        include_preamble: bool,
    },
    LineBlocks {
        start_regex: String,
        severity_capture: String,
        default_severity: String,
    },
    WholeText {
        severity: String,
    },
    Buildkit,
    DelimitedLines {
        delimiter: String,
        fields: Vec<String>,
        severity_rules: Vec<FieldValueRule>,
        default_severity: String,
    },
    JsonFields {
        source_suffix: String,
        timestamp_paths: Vec<String>,
        severity_paths: Vec<String>,
        default_severity: String,
        status_paths: Vec<String>,
        nonzero_status_is_error: bool,
        status_from_text: bool,
        fields: Vec<FieldSpec>,
        fingerprint_paths: Vec<String>,
        normalize_fingerprint_digits: bool,
    },
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct FieldSpec {
    pub(crate) label: String,
    pub(crate) paths: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct FieldValueRule {
    pub(crate) field: String,
    pub(crate) values: Vec<String>,
    pub(crate) severity: String,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct RegistryStatus {
    pub(crate) built_in: usize,
    pub(crate) user: usize,
    pub(crate) rejected: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Coverage {
    pub(crate) covered: usize,
    pub(crate) partial: usize,
    pub(crate) raw: usize,
    pub(crate) total: usize,
}

pub(crate) fn schema() -> RwLockReadGuard<'static, Schema> {
    registry().read().unwrap_or_else(|error| error.into_inner())
}

pub(crate) fn reload_user_schemas() -> RegistryStatus {
    reload_from_dir(&crate::storage::app_dir().join("schemas"))
}

pub(crate) fn coverage() -> Coverage {
    let mut result = Coverage::default();
    for line in include_str!("../parser-catalog/application-log-top100.csv")
        .lines()
        .skip(1)
    {
        let Some(status) = line.split(',').nth(3) else {
            continue;
        };
        result.total += 1;
        match status.trim() {
            "covered" => result.covered += 1,
            "partial" => result.partial += 1,
            "raw" => result.raw += 1,
            _ => {}
        }
    }
    result
}

pub(crate) fn validate_draft(
    text: &str,
    sample: &str,
) -> Result<(String, Vec<String>, usize), String> {
    let schema: Schema =
        serde_json::from_str(text).map_err(|error| format!("Schema JSON: {error}"))?;
    validate_schema(&schema)?;
    let existing: HashSet<String> = built_in().formats.into_iter().map(|item| item.id).collect();
    if let Some(duplicate) = schema
        .formats
        .iter()
        .find(|item| existing.contains(&item.id))
    {
        return Err(format!(
            "built-in format id already exists: {}",
            duplicate.id
        ));
    }
    let parsed = crate::parser_schema::parse_with_schema(sample, &schema);
    if !parsed.supported || parsed.events.is_empty() {
        return Err("schema does not parse the supplied sample".into());
    }
    let json = serde_json::to_string_pretty(&schema).map_err(|error| error.to_string())?;
    Ok((json, parsed.format_ids, parsed.events.len()))
}

pub(crate) fn install_draft(text: &str) -> Result<PathBuf, String> {
    let schema: Schema = serde_json::from_str(text).map_err(|error| error.to_string())?;
    validate_schema(&schema)?;
    let id = schema
        .formats
        .first()
        .ok_or("schema contains no formats")?
        .id
        .clone();
    let dir = crate::storage::app_dir().join("schemas");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let path = dir.join(format!("{id}.json"));
    if path.exists() {
        return Err(format!("schema already exists: {id}"));
    }
    fs::write(&path, text).map_err(|error| error.to_string())?;
    Ok(path)
}

fn registry() -> &'static RwLock<Schema> {
    static REGISTRY: OnceLock<RwLock<Schema>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(built_in()))
}

fn built_in() -> Schema {
    let schema: Schema =
        serde_json::from_str(include_str!("../parser-catalog/executable-parser-v3.json"))
            .expect("embedded parser schema v3 must be valid");
    validate_schema(&schema).expect("embedded parser schema v3 must pass validation");
    schema
}

fn reload_from_dir(dir: &Path) -> RegistryStatus {
    let mut merged = built_in();
    let mut ids: HashSet<String> = merged.formats.iter().map(|item| item.id.clone()).collect();
    let mut status = RegistryStatus {
        built_in: merged.formats.len(),
        ..Default::default()
    };
    let Ok(entries) = fs::read_dir(dir) else {
        replace_registry(merged);
        return status;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
    {
        match load_candidate(&path, &ids) {
            Ok(candidate) => {
                status.user += candidate.formats.len();
                ids.extend(candidate.formats.iter().map(|item| item.id.clone()));
                merged.formats.extend(candidate.formats);
            }
            Err(error) => status
                .rejected
                .push(format!("{}: {error}", file_name(&path))),
        }
    }
    replace_registry(merged);
    status
}

fn load_candidate(path: &Path, existing: &HashSet<String>) -> Result<Schema, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let schema: Schema = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    validate_schema(&schema)?;
    if let Some(duplicate) = schema
        .formats
        .iter()
        .find(|item| existing.contains(&item.id))
    {
        return Err(format!("format id already exists: {}", duplicate.id));
    }
    Ok(schema)
}

fn validate_schema(schema: &Schema) -> Result<(), String> {
    if schema.schema_version != 3 || schema.fallback != "raw" {
        return Err("expected schema_version 3 and raw fallback".into());
    }
    if schema.formats.is_empty() || schema.formats.len() > 100 {
        return Err("formats must contain 1..100 items".into());
    }
    let mut ids = HashSet::new();
    for format in &schema.formats {
        if format.id.is_empty()
            || format.id.len() > 80
            || !format
                .id
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(format!("invalid format id: {}", format.id));
        }
        if !ids.insert(&format.id) {
            return Err(format!("duplicate format id: {}", format.id));
        }
        if !matches!(format.record_kind.as_str(), "text" | "json") {
            return Err(format!("invalid record kind: {}", format.id));
        }
        if format.detect.all.is_empty() && format.detect.any.is_empty() {
            return Err(format!("empty detector: {}", format.id));
        }
        for condition in format.detect.all.iter().chain(&format.detect.any) {
            match condition.kind.as_str() {
                "path_exists" if condition.value.starts_with("$.") => {}
                "regex" => validate_regex(&condition.value)?,
                _ => return Err(format!("invalid detector in {}", format.id)),
            }
        }
        match &format.parser {
            ParserSpec::TextBlocks { start_regex, .. }
            | ParserSpec::LineBlocks { start_regex, .. } => validate_regex(start_regex)?,
            ParserSpec::JsonFields { .. } if format.record_kind != "json" => {
                return Err(format!("json parser on text format: {}", format.id))
            }
            ParserSpec::DelimitedLines {
                delimiter,
                fields,
                severity_rules,
                ..
            } => {
                if !matches!(delimiter.as_str(), "whitespace" | "tab" | "comma" | "pipe") {
                    return Err(format!("unsupported delimiter: {}", format.id));
                }
                if fields.is_empty() || fields.len() > 64 {
                    return Err(format!(
                        "delimited fields must contain 1..64 items: {}",
                        format.id
                    ));
                }
                if severity_rules.iter().any(|rule| {
                    !fields.contains(&rule.field)
                        || rule.values.is_empty()
                        || rule.severity.trim().is_empty()
                }) {
                    return Err(format!("invalid severity rule: {}", format.id));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_regex(pattern: &str) -> Result<(), String> {
    if pattern.len() > 2_000 {
        return Err("regex exceeds 2000 bytes".into());
    }
    RegexBuilder::new(pattern)
        .multi_line(true)
        .build()
        .map(|_| ())
        .map_err(|error| format!("invalid regex: {error}"))
}

fn replace_registry(schema: Schema) {
    *registry()
        .write()
        .unwrap_or_else(|error| error.into_inner()) = schema;
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("schema.json")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_catalog_coverage_without_inflation() {
        let coverage = coverage();
        assert_eq!(coverage.total, 100);
        assert_eq!(coverage.covered + coverage.partial + coverage.raw, 100);
    }

    #[test]
    fn rejects_an_unbounded_or_invalid_candidate() {
        let schema = Schema {
            schema_version: 3,
            fallback: "raw".into(),
            formats: Vec::new(),
        };
        assert!(validate_schema(&schema).is_err());
    }
}
