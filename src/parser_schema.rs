use crate::parser_registry::{Condition, Detect, FormatSchema, ParserSpec, Schema, Validation};
use regex::{Regex, RegexBuilder};
use serde_json::Value;

#[derive(Clone)]
pub(crate) struct LogEvent {
    pub(crate) source: String,
    pub(crate) severity: Severity,
    pub(crate) content: String,
    pub(crate) fingerprint: String,
    pub(crate) repeats: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Severity {
    Critical,
    Error,
    Warning,
    Info,
    Unknown,
}

impl Severity {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
            Self::Unknown => "UNKNOWN",
        }
    }

    fn from_text(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "0" | "1" | "2" | "F" | "ALERT" | "CRIT" | "FATAL" | "CRITICAL" | "PANIC" => {
                Self::Critical
            }
            "3" | "E" | "ERROR" | "ERR" | "SEVERE" => Self::Error,
            "4" | "W" | "WARNING" | "WARN" => Self::Warning,
            "5" | "6" | "7" | "I" | "D" | "INFO" | "NOTICE" | "DEBUG" | "TRACE" => Self::Info,
            _ => Self::Unknown,
        }
    }
}

pub(crate) struct ParseOutcome {
    pub(crate) format_ids: Vec<String>,
    pub(crate) events: Vec<LogEvent>,
    pub(crate) supported: bool,
}

pub(crate) fn parse(input: &str) -> ParseOutcome {
    let schema = crate::parser_registry::schema();
    parse_with_schema(input, &schema)
}

pub(crate) fn parse_with_schema(input: &str, schema: &Schema) -> ParseOutcome {
    let trimmed = input.trim();
    if let Some(records) = json_records(trimmed) {
        return parse_json_records(schema, &records);
    }
    let Some(format) = best_format(schema, "text", None, trimmed) else {
        return fallback(Vec::new());
    };
    let events = parse_text(format, trimmed);
    outcome(vec![format.id.clone()], events, &format.validation)
}

fn parse_json_records(schema: &Schema, records: &[Value]) -> ParseOutcome {
    let mut ids = Vec::new();
    let mut events = Vec::new();
    for (index, record) in records.iter().enumerate() {
        let text = record.to_string();
        let Some(format) = best_format(schema, "json", Some(record), &text) else {
            return fallback(ids);
        };
        let Some(event) = parse_json_event(format, index + 1, record) else {
            return fallback(ids);
        };
        if !valid_events(std::slice::from_ref(&event), &format.validation) {
            return fallback(ids);
        }
        if !ids.contains(&format.id) {
            ids.push(format.id.clone());
        }
        events.push(event);
    }
    ParseOutcome {
        format_ids: ids,
        supported: !events.is_empty(),
        events,
    }
}

fn best_format<'a>(
    schema: &'a Schema,
    kind: &str,
    value: Option<&Value>,
    text: &str,
) -> Option<&'a FormatSchema> {
    schema
        .formats
        .iter()
        .filter(|format| format.record_kind == kind)
        .filter(|format| detector_matches(&format.detect, value, text))
        .max_by_key(|format| format.detect.all.len() * 3 + format.detect.any.len())
}

fn detector_matches(detect: &Detect, value: Option<&Value>, text: &str) -> bool {
    detect
        .all
        .iter()
        .all(|rule| condition_matches(rule, value, text))
        && (detect.any.is_empty()
            || detect
                .any
                .iter()
                .any(|rule| condition_matches(rule, value, text)))
}

fn condition_matches(rule: &Condition, value: Option<&Value>, text: &str) -> bool {
    match rule.kind.as_str() {
        "path_exists" => value
            .and_then(|value| at_path(value, &rule.value))
            .is_some(),
        "regex" => regex(&rule.value).is_some_and(|regex| regex.is_match(text)),
        _ => false,
    }
}

fn parse_text(format: &FormatSchema, input: &str) -> Vec<LogEvent> {
    match &format.parser {
        ParserSpec::TextBlocks {
            start_regex,
            severity,
            preamble_severity,
            include_preamble,
        } => crate::parser_text::fixed_blocks(
            input,
            start_regex,
            severity,
            preamble_severity,
            *include_preamble,
        ),
        ParserSpec::LineBlocks {
            start_regex,
            severity_capture,
            default_severity,
        } => {
            crate::parser_text::line_blocks(input, start_regex, severity_capture, default_severity)
        }
        ParserSpec::WholeText { severity } => crate::parser_text::whole(input, severity),
        ParserSpec::Buildkit => crate::parser_text::buildkit(input),
        ParserSpec::DelimitedLines {
            delimiter,
            fields,
            severity_rules,
            default_severity,
        } => crate::parser_text::delimited_lines(
            input,
            delimiter,
            fields,
            severity_rules,
            default_severity,
        ),
        ParserSpec::JsonFields { .. } => Vec::new(),
    }
}

fn parse_json_event(format: &FormatSchema, index: usize, value: &Value) -> Option<LogEvent> {
    let ParserSpec::JsonFields {
        source_suffix,
        timestamp_paths,
        severity_paths,
        default_severity,
        status_paths,
        nonzero_status_is_error,
        status_from_text,
        fields,
        fingerprint_paths,
        normalize_fingerprint_digits,
    } = &format.parser
    else {
        return None;
    };
    let timestamp = first_value(value, timestamp_paths).unwrap_or_else(|| "no timestamp".into());
    let field_values: Vec<(String, String)> = fields
        .iter()
        .filter_map(|field| {
            first_value(value, &field.paths).map(|content| (field.label.clone(), content))
        })
        .collect();
    let message = field_values
        .iter()
        .map(|(label, value)| {
            if label.is_empty() {
                value.clone()
            } else {
                format!("{label}={value}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    let status = first_value(value, status_paths)
        .and_then(|value| value.parse::<u64>().ok())
        .or_else(|| status_from_text.then(|| http_status(&message)).flatten());
    let level_text = first_value(value, severity_paths).unwrap_or_else(|| default_severity.clone());
    let severity = resolve_severity(&level_text, status, *nonzero_status_is_error, &message);
    let level = if starts_with_level(&message) {
        String::new()
    } else {
        severity.label().to_owned()
    };
    let content = format!("{timestamp} {level} {message}")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut fingerprint = fingerprint_paths
        .iter()
        .filter_map(|path| at_path(value, path).map(value_text))
        .collect::<Vec<_>>()
        .join("|");
    if fingerprint.is_empty() {
        fingerprint = message.clone();
    }
    if *normalize_fingerprint_digits {
        fingerprint = normalize_digits(&fingerprint);
    }
    Some(LogEvent {
        source: format!("JSON item {index}{source_suffix}"),
        severity,
        content,
        fingerprint,
        repeats: 1,
    })
}

fn json_records(input: &str) -> Option<Vec<Value>> {
    match serde_json::from_str::<Value>(input).ok() {
        Some(Value::Array(values)) => Some(values),
        Some(value @ Value::Object(_)) => Some(vec![value]),
        _ => {
            let values: Option<Vec<Value>> = input
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| serde_json::from_str(line).ok())
                .collect();
            values.filter(|values| values.len() > 1)
        }
    }
}

fn at_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let path = path.strip_prefix("$.")?;
    value
        .get(path)
        .or_else(|| path.split('.').try_fold(value, |item, key| item.get(key)))
}

fn first_value(value: &Value, paths: &[String]) -> Option<String> {
    paths
        .iter()
        .filter_map(|path| at_path(value, path).map(value_text))
        .find(|value| !value.is_empty() && value != "null" && value != "{}")
}

fn value_text(value: &Value) -> String {
    value
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| value.to_string())
}

fn resolve_severity(text: &str, status: Option<u64>, nonzero: bool, message: &str) -> Severity {
    if nonzero && status.is_some_and(|status| status != 0) {
        return Severity::Error;
    }
    if status.is_some_and(|status| status >= 500) {
        return Severity::Critical;
    }
    if status.is_some_and(|status| status >= 400) {
        return Severity::Error;
    }
    let severity = Severity::from_text(text);
    if severity != Severity::Unknown {
        return severity;
    }
    let lower = message.to_ascii_lowercase();
    if lower.contains("panic") || lower.contains("fatal") {
        Severity::Critical
    } else if lower.contains("error") || lower.contains("failed") {
        Severity::Error
    } else if lower.contains("warn") {
        Severity::Warning
    } else if status.is_some() || lower.contains("info") {
        Severity::Info
    } else {
        Severity::Unknown
    }
}

fn valid_events(events: &[LogEvent], validation: &Validation) -> bool {
    (!validation.require_events || !events.is_empty())
        && (!validation.require_content
            || events.iter().all(|event| !event.content.trim().is_empty()))
}

fn outcome(ids: Vec<String>, events: Vec<LogEvent>, validation: &Validation) -> ParseOutcome {
    let supported = valid_events(&events, validation);
    ParseOutcome {
        format_ids: ids,
        events: if supported { events } else { Vec::new() },
        supported,
    }
}

fn fallback(ids: Vec<String>) -> ParseOutcome {
    ParseOutcome {
        format_ids: ids,
        events: Vec::new(),
        supported: false,
    }
}

fn regex(pattern: &str) -> Option<Regex> {
    RegexBuilder::new(pattern).multi_line(true).build().ok()
}

fn starts_with_level(text: &str) -> bool {
    [
        "TRACE", "DEBUG", "INFO", "WARN", "WARNING", "ERROR", "FATAL",
    ]
    .iter()
    .any(|level| text.trim_start().starts_with(level))
}

fn http_status(text: &str) -> Option<u64> {
    text.split_whitespace()
        .filter_map(|token| token.parse().ok())
        .find(|status| (100..=599).contains(status))
}

fn normalize_digits(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut digits = false;
    for character in text.chars() {
        if character.is_ascii_digit() {
            if !digits {
                result.push('#');
            }
            digits = true;
        } else {
            digits = false;
            result.push(character.to_ascii_lowercase());
        }
    }
    result
}
