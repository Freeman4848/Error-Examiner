use serde_json::Value;
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
    pub(crate) batches: Vec<String>,
    pub(crate) raw_preview: String,
    pub(crate) parsed_preview: String,
    pub(crate) normalized: bool,
}

#[derive(Clone)]
struct LogEvent {
    source: String,
    severity: Severity,
    content: String,
    fingerprint: String,
    repeats: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Severity {
    Critical,
    Error,
    Warning,
    Info,
    Unknown,
}

impl Severity {
    fn label(self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
            Self::Unknown => "UNKNOWN",
        }
    }
}

pub(crate) fn prepare(name: String, input: &str, batch_chars: usize) -> PreparedLog {
    let clean = normalize_newlines(input);
    let detection = crate::parser_catalog::detect(&clean);
    if !detection.supported {
        return prepare_fallback(name, input, batch_chars, detection.format_ids);
    }
    let events = parse_json(&clean).unwrap_or_else(|| parse_text(&clean));
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
    let batches = make_batches(&events, batch_chars.max(3_000));
    PreparedLog {
        name,
        detected_format: detection.format_ids.join(" + "),
        original_chars: input.chars().count(),
        normalized_chars: normalized.chars().count(),
        event_count,
        important_events,
        duplicate_count,
        estimated_tokens: normalized.chars().count().div_ceil(3),
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
    let clean = normalize_newlines(&input);
    let events = parse_json(&clean).unwrap_or_else(|| parse_text(&clean));
    let (events, _) = deduplicate(events);
    let parsed_preview = render_events(&events);
    Ok(PreparedLog {
        name,
        detected_format: "raw (preparation disabled)".to_owned(),
        original_chars: size,
        normalized_chars: size,
        event_count: 1,
        important_events: 0,
        duplicate_count: 0,
        estimated_tokens: size.div_ceil(3),
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

fn parse_json(input: &str) -> Option<Vec<LogEvent>> {
    let values = serde_json::from_str::<Value>(input)
        .ok()?
        .as_array()?
        .clone();
    Some(
        values
            .iter()
            .enumerate()
            .map(|(index, value)| json_event(index + 1, value))
            .collect(),
    )
}

fn json_event(index: usize, value: &Value) -> LogEvent {
    if value["httpRequest"].is_object() {
        return json_http_event(index, value);
    }
    if value["protoPayload"].is_object() {
        return json_audit_event(index, value);
    }
    if let Some(payload) = value["textPayload"].as_str() {
        return json_text_event(index, value, payload);
    }
    json_payload_event(index, value)
}

fn json_http_event(index: usize, value: &Value) -> LogEvent {
    let severity_text = value["severity"].as_str().unwrap_or("UNKNOWN");
    let status = value["httpRequest"]["status"].as_u64();
    let severity = severity_from(severity_text, status);
    let timestamp = value["timestamp"].as_str().unwrap_or("no timestamp");
    let request = &value["httpRequest"];
    let method = request["requestMethod"].as_str().unwrap_or("");
    let url = request["requestUrl"].as_str().unwrap_or("");
    let latency = request["latency"].as_str().unwrap_or("");
    let payload = value["textPayload"]
        .as_str()
        .or_else(|| value["jsonPayload"]["message"].as_str())
        .unwrap_or("");
    let content = format!(
        "{timestamp} {severity_text} {method} {url} status={} latency={latency} {payload}",
        status.map_or_else(|| "-".to_owned(), |value| value.to_string())
    )
    .trim()
    .to_owned();
    let fingerprint = format!(
        "{severity_text}|{method}|{url}|{}|{payload}",
        status.unwrap_or(0)
    );
    LogEvent {
        source: format!("JSON item {index}"),
        severity,
        content,
        fingerprint,
        repeats: 1,
    }
}

fn json_audit_event(index: usize, value: &Value) -> LogEvent {
    let payload = &value["protoPayload"];
    let severity_text = value["severity"].as_str().unwrap_or("INFO");
    let status = payload["status"]["code"].as_u64();
    let severity = if status.is_some_and(|code| code != 0) {
        Severity::Error
    } else {
        severity_from(severity_text, None)
    };
    let timestamp = value["timestamp"].as_str().unwrap_or("no timestamp");
    let service = payload["serviceName"].as_str().unwrap_or("");
    let method = payload["methodName"].as_str().unwrap_or("");
    let resource = payload["resourceName"].as_str().unwrap_or("");
    let content = format!("{timestamp} {severity_text} {service} {method} {resource}")
        .trim()
        .to_owned();
    LogEvent {
        source: format!("JSON item {index} / protoPayload"),
        severity,
        fingerprint: format!("{severity_text}|{service}|{method}|{resource}"),
        content,
        repeats: 1,
    }
}

fn json_text_event(index: usize, value: &Value, payload: &str) -> LogEvent {
    let status = payload
        .split_whitespace()
        .filter_map(|token| token.parse::<u64>().ok())
        .find(|status| (100..=599).contains(status));
    let inferred = text_event(index, index, payload.to_owned()).severity;
    let severity = match severity_from(value["severity"].as_str().unwrap_or(""), status) {
        Severity::Unknown => inferred,
        known => known,
    };
    let timestamp = value["timestamp"].as_str().unwrap_or("no timestamp");
    let has_level_prefix = [
        "TRACE", "DEBUG", "INFO", "WARN", "WARNING", "ERROR", "FATAL",
    ]
    .iter()
    .any(|level| payload.trim_start().starts_with(level));
    let content = if has_level_prefix {
        format!("{timestamp} {payload}")
    } else {
        format!("{timestamp} {} {payload}", severity.label())
    };
    LogEvent {
        source: format!("JSON item {index} / textPayload"),
        severity,
        fingerprint: fingerprint(payload),
        content,
        repeats: 1,
    }
}

fn json_payload_event(index: usize, value: &Value) -> LogEvent {
    let payload = &value["jsonPayload"];
    let text = payload["message"]
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| payload.to_string());
    let severity_text = value["severity"].as_str().unwrap_or("UNKNOWN");
    let severity = severity_from(severity_text, None);
    let timestamp = value["timestamp"].as_str().unwrap_or("no timestamp");
    LogEvent {
        source: format!("JSON item {index} / jsonPayload"),
        severity,
        fingerprint: fingerprint(&text),
        content: format!("{timestamp} {severity_text} {text}"),
        repeats: 1,
    }
}

fn parse_text(input: &str) -> Vec<LogEvent> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.iter().any(|line| line.starts_with("error[E")) {
        return parse_rust_diagnostics(&lines);
    }
    lines
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| text_event(index + 1, index + 1, line.to_string()))
        .collect()
}

fn parse_rust_diagnostics(lines: &[&str]) -> Vec<LogEvent> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("error[E") && index > start {
            ranges.push((start, index));
            start = index;
        }
    }
    ranges.push((start, lines.len()));
    ranges
        .into_iter()
        .filter(|(start, end)| start < end)
        .map(|(start, end)| text_event(start + 1, end, lines[start..end].join("\n")))
        .collect()
}

fn text_event(start: usize, end: usize, content: String) -> LogEvent {
    let lower = content.to_ascii_lowercase();
    let severity = if lower.contains("fatal") || lower.contains("panic") {
        Severity::Critical
    } else if lower.contains("error") || lower.contains("failed") {
        Severity::Error
    } else if lower.contains("warning") || lower.contains("warn") {
        Severity::Warning
    } else if lower.contains("info") || lower.contains("checking ") {
        Severity::Info
    } else {
        Severity::Unknown
    };
    LogEvent {
        source: if start == end {
            format!("line {start}")
        } else {
            format!("lines {start}-{end}")
        },
        severity,
        fingerprint: fingerprint(&content),
        content,
        repeats: 1,
    }
}

fn severity_from(text: &str, status: Option<u64>) -> Severity {
    if status.is_some_and(|value| value >= 500) || text.eq_ignore_ascii_case("CRITICAL") {
        Severity::Critical
    } else if status.is_some_and(|value| value >= 400) || text.eq_ignore_ascii_case("ERROR") {
        Severity::Error
    } else if text.eq_ignore_ascii_case("WARNING") || text.eq_ignore_ascii_case("WARN") {
        Severity::Warning
    } else if status.is_some() || text.eq_ignore_ascii_case("INFO") {
        Severity::Info
    } else {
        Severity::Unknown
    }
}

fn fingerprint(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_digits = false;
    for character in text.chars() {
        if character.is_ascii_digit() {
            if !in_digits {
                result.push('#');
            }
            in_digits = true;
        } else {
            in_digits = false;
            result.push(character.to_ascii_lowercase());
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
        let log = "Checking x\nerror[E1]: bad\n --> x:1\nhelp: fix\nerror[E2]: worse\n --> x:2";
        let result = prepare("rust".into(), log, 3_000);
        assert_eq!(result.event_count, 3);
        assert!(result.parsed_preview.contains("help: fix"));
    }

    #[test]
    fn deduplicates_json_requests() {
        let log = r#"[{"severity":"INFO","httpRequest":{"requestUrl":"/health","status":200}},{"severity":"INFO","httpRequest":{"requestUrl":"/health","status":200}}]"#;
        let result = prepare("json".into(), log, 3_000);
        assert_eq!(result.duplicate_count, 1);
        assert!(result.parsed_preview.contains("repeated 2 times"));
    }

    #[test]
    fn classifies_cloud_logging_payload_variants() {
        let log = r#"[
          {"timestamp":"t1","severity":"INFO","httpRequest":{"requestMethod":"GET","requestUrl":"/health","status":200}},
          {"timestamp":"t2","severity":"INFO","protoPayload":{"serviceName":"firestore.googleapis.com","methodName":"RunQuery","resourceName":"db"}},
          {"timestamp":"t3","textPayload":"127.0.0.1 - \"GET / HTTP/1.1\" 200 OK"}
        ]"#;
        let result = prepare("cloud.json".into(), log, 3_000);
        assert_eq!(result.important_events, 0);
        assert!(!result.parsed_preview.contains("UNKNOWN"));
        assert!(result.parsed_preview.contains("protoPayload"));
        assert!(result.parsed_preview.contains("textPayload"));
    }
}
