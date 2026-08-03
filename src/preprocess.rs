use crate::parser_schema::{LogEvent, ParseOutcome, Severity};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct PreparedLog {
    pub(crate) name: String,
    pub(crate) detected_format: String,
    pub(crate) original_chars: usize,
    pub(crate) normalized_chars: usize,
    pub(crate) event_count: usize,
    pub(crate) important_events: usize,
    pub(crate) duplicate_count: usize,
    pub(crate) estimated_tokens: usize,
    pub(crate) selected_variant: String,
    pub(crate) batches: Vec<String>,
    pub(crate) raw_preview: String,
    pub(crate) parsed_preview: String,
    pub(crate) normalized: bool,
}

pub(crate) fn prepare(name: String, input: &str, batch_chars: usize) -> PreparedLog {
    let clean = sanitized_raw(input);
    let ParseOutcome {
        format_ids,
        events,
        supported,
    } = crate::parser_schema::parse(&clean);
    if !supported {
        return prepare_fallback(name, input, batch_chars, format_ids);
    }
    let event_count = events.len();
    let important_events = events
        .iter()
        .filter(|event| {
            matches!(
                event.severity,
                Severity::Critical | Severity::Error | Severity::Warning
            )
        })
        .count();
    let (events, duplicate_count) = deduplicate(events);
    let normalized = render_events(&events);
    let use_raw = clean.chars().count() < normalized.chars().count();
    let selected_chars = if use_raw {
        clean.chars().count()
    } else {
        normalized.chars().count()
    };
    let batches = if use_raw {
        crate::preprocess_raw::batches(&clean, batch_chars.max(3_000))
    } else {
        make_batches(&events, batch_chars.max(3_000))
    };
    PreparedLog {
        name,
        detected_format: format_ids.join(" + "),
        original_chars: input.chars().count(),
        normalized_chars: normalized.chars().count(),
        event_count,
        important_events,
        duplicate_count,
        estimated_tokens: selected_chars.div_ceil(3),
        selected_variant: if use_raw { "Raw" } else { "Parsed" }.to_owned(),
        batches,
        raw_preview: input.to_owned(),
        parsed_preview: normalized,
        normalized: true,
    }
}

pub(crate) fn prepare_raw(
    name: String,
    input: String,
    character_budget: usize,
) -> Result<PreparedLog, String> {
    let size = input.chars().count();
    if size > character_budget {
        return Err(format!(
            "Raw log has {size} characters but the current limit is {character_budget}. Raise the limit or enable normalization."
        ));
    }
    let clean = sanitized_raw(&input);
    let parsed = crate::parser_schema::parse(&clean);
    let parsed_preview = if parsed.supported {
        render_events(&deduplicate(parsed.events).0)
    } else {
        input.clone()
    };
    Ok(PreparedLog {
        name,
        detected_format: "raw (preparation disabled)".to_owned(),
        original_chars: size,
        normalized_chars: size,
        event_count: 1,
        important_events: 0,
        duplicate_count: 0,
        estimated_tokens: size.div_ceil(3),
        selected_variant: "Raw".to_owned(),
        batches: vec![input.clone()],
        raw_preview: input,
        parsed_preview,
        normalized: false,
    })
}

fn prepare_fallback(
    name: String,
    input: &str,
    batch_chars: usize,
    formats: Vec<String>,
) -> PreparedLog {
    let size = input.chars().count();
    PreparedLog {
        name,
        detected_format: if formats.is_empty() {
            "unknown -> raw".to_owned()
        } else {
            format!("{} -> raw", formats.join(" + "))
        },
        original_chars: size,
        normalized_chars: size,
        event_count: 1,
        important_events: 0,
        duplicate_count: 0,
        estimated_tokens: size.div_ceil(3),
        selected_variant: "Raw".to_owned(),
        batches: crate::preprocess_raw::batches(input, batch_chars.max(3_000)),
        raw_preview: input.to_owned(),
        parsed_preview: input.to_owned(),
        normalized: false,
    }
}

fn normalize_newlines(input: &str) -> String {
    input
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(|line| redact_line(&line.trim_end().replace('\t', "    ")))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn sanitized_raw(input: &str) -> String {
    normalize_newlines(input)
}

fn redact_line(line: &str) -> String {
    let mut result = line.to_owned();
    for marker in ["Bearer ", "api_key=", "apikey=", "password=", "token="] {
        if let Some(start) = result
            .to_ascii_lowercase()
            .find(&marker.to_ascii_lowercase())
        {
            let value_start = start + marker.len();
            let value_end = result[value_start..]
                .find(char::is_whitespace)
                .map_or(result.len(), |offset| value_start + offset);
            result.replace_range(value_start..value_end, "[REDACTED]");
        }
    }
    result
}

fn deduplicate(events: Vec<LogEvent>) -> (Vec<LogEvent>, usize) {
    let mut positions = HashMap::<String, usize>::new();
    let mut kept: Vec<LogEvent> = Vec::new();
    let mut duplicates = 0;
    for event in events {
        if let Some(index) = positions.get(&event.fingerprint).copied() {
            kept[index].repeats += 1;
            duplicates += 1;
        } else {
            positions.insert(event.fingerprint.clone(), kept.len());
            kept.push(event);
        }
    }
    (kept, duplicates)
}

fn render_event(event: &LogEvent) -> String {
    let repeats = if event.repeats > 1 {
        format!(" · repeated {} times", event.repeats)
    } else {
        String::new()
    };
    format!(
        "[{} · {}{}]\n{}",
        event.source,
        event.severity.label(),
        repeats,
        event.content
    )
}

fn render_events(events: &[LogEvent]) -> String {
    events
        .iter()
        .map(render_event)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn make_batches(events: &[LogEvent], budget: usize) -> Vec<String> {
    let mut batches = Vec::new();
    let mut current = String::new();
    for event in events {
        let rendered = render_event(event);
        if !current.is_empty() && current.chars().count() + rendered.chars().count() + 2 > budget {
            batches.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push_str("\n\n");
        }
        current.push_str(&rendered);
    }
    if !current.is_empty() {
        batches.push(current);
    }
    batches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_rust_diagnostic_whole() {
        let log =
            "Checking x\nerror[E0001]: bad\n --> x:1:1\nhelp: fix\nerror[E0002]: worse\n --> x:2:1";
        let result = prepare("rust".into(), log, 3_000);
        assert_eq!(result.event_count, 3);
        assert!(result.parsed_preview.contains("help: fix"));
    }

    #[test]
    fn deduplicates_json_requests() {
        let log = r#"[{"insertId":"1","severity":"INFO","httpRequest":{"requestUrl":"/health","status":200}},{"insertId":"2","severity":"INFO","httpRequest":{"requestUrl":"/health","status":200}}]"#;
        let result = prepare("json".into(), log, 3_000);
        assert_eq!(result.duplicate_count, 1);
        assert!(result.parsed_preview.contains("repeated 2 times"));
    }

    #[test]
    fn classifies_cloud_logging_payload_variants() {
        let log = r#"[
          {"insertId":"1","timestamp":"t1","severity":"INFO","httpRequest":{"requestMethod":"GET","requestUrl":"/health","status":200}},
          {"insertId":"2","timestamp":"t2","severity":"INFO","protoPayload":{"serviceName":"firestore.googleapis.com","methodName":"RunQuery","resourceName":"db"}},
          {"insertId":"3","timestamp":"t3","textPayload":"127.0.0.1 - \"GET / HTTP/1.1\" 200 OK"}
        ]"#;
        let result = prepare("cloud.json".into(), log, 3_000);
        assert_eq!(result.important_events, 0);
        assert!(!result.parsed_preview.contains("UNKNOWN"));
        assert!(result.parsed_preview.contains("protoPayload"));
        assert!(result.parsed_preview.contains("textPayload"));
    }

    #[test]
    fn sends_raw_when_parsed_is_larger() {
        let log = "fatal: not a git repository (or any parent): .git";
        let result = prepare("git.log".into(), log, 3_000);
        assert_eq!(result.selected_variant, "Raw");
        assert_eq!(result.batches, [log]);
    }
}
