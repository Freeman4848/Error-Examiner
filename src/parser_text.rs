use crate::parser_schema::{LogEvent, Severity};
use regex::{Regex, RegexBuilder};

pub(crate) fn whole(input: &str, level: &str) -> Vec<LogEvent> {
    vec![event(
        0,
        input.lines().count(),
        level,
        input.trim().to_owned(),
    )]
}

pub(crate) fn buildkit(input: &str) -> Vec<LogEvent> {
    let lines: Vec<&str> = input.lines().collect();
    let marker = Regex::new(
        "(?i)error|failed|failure|exit code|no such file|not found|denied|timeout|canceled",
    )
    .unwrap();
    let mut keep = vec![false; lines.len()];
    for (index, line) in lines.iter().enumerate() {
        if marker.is_match(line) {
            let start = index.saturating_sub(2);
            let end = (index + 3).min(lines.len());
            keep[start..end].fill(true);
        }
    }
    let mut selected = Vec::new();
    let mut omitted = false;
    for (line, keep) in lines.iter().zip(keep) {
        if keep {
            if omitted {
                selected.push("… omitted successful BuildKit steps …");
            }
            selected.push(line);
            omitted = false;
        } else {
            omitted = true;
        }
    }
    let content = selected.join("\n");
    (!content.is_empty())
        .then(|| event(0, lines.len(), "ERROR", content))
        .into_iter()
        .collect()
}

pub(crate) fn fixed_blocks(
    input: &str,
    pattern: &str,
    level: &str,
    preamble_level: &str,
    include_preamble: bool,
) -> Vec<LogEvent> {
    blocks(
        input,
        pattern,
        "",
        level,
        include_preamble.then_some(preamble_level),
    )
}

pub(crate) fn line_blocks(
    input: &str,
    pattern: &str,
    capture: &str,
    default_level: &str,
) -> Vec<LogEvent> {
    blocks(input, pattern, capture, default_level, None)
}

fn blocks(
    input: &str,
    pattern: &str,
    capture: &str,
    default_level: &str,
    preamble_level: Option<&str>,
) -> Vec<LogEvent> {
    let Some(regex) = RegexBuilder::new(pattern).multi_line(true).build().ok() else {
        return Vec::new();
    };
    let lines: Vec<&str> = input.lines().collect();
    let starts: Vec<(usize, String)> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            let captures = regex.captures(line)?;
            let level = captures
                .name(capture)
                .map(|value| value.as_str())
                .unwrap_or(default_level);
            Some((index, level.to_owned()))
        })
        .collect();
    if starts.is_empty() {
        return Vec::new();
    }
    let mut events = Vec::new();
    if starts[0].0 > 0 && preamble_level.is_some() {
        events.push(event(
            0,
            starts[0].0,
            preamble_level.unwrap_or("INFO"),
            lines[..starts[0].0].join("\n"),
        ));
    }
    for (position, (start, level)) in starts.iter().enumerate() {
        let end = starts
            .get(position + 1)
            .map(|item| item.0)
            .unwrap_or(lines.len());
        events.push(event(*start, end, level, lines[*start..end].join("\n")));
    }
    events
}

fn event(start: usize, end: usize, level: &str, content: String) -> LogEvent {
    LogEvent {
        source: if start + 1 == end {
            format!("line {end}")
        } else {
            format!("lines {}-{end}", start + 1)
        },
        severity: level_severity(level, &content),
        fingerprint: normalize_digits(&content),
        content,
        repeats: 1,
    }
}

fn level_severity(level: &str, content: &str) -> Severity {
    match level.to_ascii_uppercase().as_str() {
        "F" | "ALERT" | "CRIT" | "FATAL" | "CRITICAL" | "PANIC" => Severity::Critical,
        "E" | "ERROR" | "ERR" | "SEVERE" => Severity::Error,
        "W" | "WARN" | "WARNING" => Severity::Warning,
        "I" | "D" | "V" | "INFO" | "NOTICE" | "DEBUG" | "TRACE" => Severity::Info,
        _ if Regex::new("(?i)panic|fatal").unwrap().is_match(content) => Severity::Critical,
        _ if Regex::new("(?i)error|failed|failure|refused|not found")
            .unwrap()
            .is_match(content) =>
        {
            Severity::Error
        }
        _ if Regex::new("(?i)warn").unwrap().is_match(content) => Severity::Warning,
        _ if Regex::new(r#"\"\s+[45]\d{2}(?:\s|$)"#)
            .unwrap()
            .is_match(content) =>
        {
            Severity::Error
        }
        _ if Regex::new("(?i)(?:^|\\s)(?:info|notice|debug|trace)(?:$|\\s)")
            .unwrap()
            .is_match(content) =>
        {
            Severity::Info
        }
        _ => Severity::Unknown,
    }
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
