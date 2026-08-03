use crate::mcp_audit::AuditEvent;
use rmcp::{
    ServerHandler,
    handler::server::wrapper::Parameters,
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

const DEFAULT_INPUT_BYTES: usize = 10 * 1024 * 1024;
const DEFAULT_BATCH_CHARS: usize = 12_000;
const MIN_BATCH_CHARS: usize = 3_000;
const MAX_BATCH_CHARS: usize = 50_000;

#[derive(Debug, Clone)]
pub(crate) struct ErrorExaminerMcp;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PrepareRequest {
    #[schemars(description = "Local log path. Exactly one of path or text is required.")]
    path: Option<String>,
    #[schemars(description = "Inline log text. Exactly one of path or text is required.")]
    text: Option<String>,
    #[schemars(description = "Display name for inline text.")]
    name: Option<String>,
    #[schemars(description = "auto, raw, parsed, or compare. Default: auto.")]
    mode: Option<String>,
    #[schemars(description = "Zero-based output batch. Default: 0.")]
    batch_index: Option<usize>,
    #[schemars(description = "Characters per batch, clamped to 3000..50000.")]
    batch_chars: Option<usize>,
    #[schemars(description = "Save Raw and Parsed artifacts locally. Default: false.")]
    save_artifacts: Option<bool>,
    #[schemars(description = "Client label stored in the metadata-only audit.")]
    client: Option<String>,
}

#[derive(Serialize)]
struct ToolOutput {
    contract_version: u32,
    status: String,
    request_id: String,
    input_name: String,
    detected_format: String,
    requested_mode: String,
    sent_variant: String,
    raw_chars: usize,
    parsed_chars: usize,
    raw_tokens_estimate: usize,
    parsed_tokens_estimate: usize,
    reduction_percent: f64,
    event_count: usize,
    important_events: usize,
    duplicates_removed: usize,
    batch_index: usize,
    batch_count: usize,
    batch_chars: usize,
    batch_tokens_estimate: usize,
    content: Option<String>,
    raw_content: Option<String>,
    parsed_content: Option<String>,
    warnings: Vec<String>,
    error_code: Option<String>,
}

struct LoadedInput {
    name: String,
    text: String,
}

#[derive(Clone, Copy)]
enum Mode {
    Auto,
    Raw,
    Parsed,
    Compare,
}

impl Mode {
    fn parse(value: Option<&str>) -> Result<Self, String> {
        match value.unwrap_or("auto").to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "raw" => Ok(Self::Raw),
            "parsed" => Ok(Self::Parsed),
            "compare" => Ok(Self::Compare),
            _ => Err("mode must be auto, raw, parsed, or compare".into()),
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Raw => "raw",
            Self::Parsed => "parsed",
            Self::Compare => "compare",
        }
    }
}

#[tool_router]
impl ErrorExaminerMcp {
    #[tool(
        description = "Compress and batch a local or inline error log while preserving diagnostic events. Prefer path so the agent does not read the raw log first."
    )]
    fn prepare_error_log(&self, Parameters(request): Parameters<PrepareRequest>) -> String {
        let started = Instant::now();
        let request_id = request_id();
        let result = panic::catch_unwind(AssertUnwindSafe(|| process(&request, &request_id)));
        match result {
            Ok(Ok(output)) => serialize(&output),
            Ok(Err((code, message))) => error_response(
                &request,
                &request_id,
                started.elapsed().as_millis(),
                &code,
                &message,
            ),
            Err(_) => error_response(
                &request,
                &request_id,
                started.elapsed().as_millis(),
                "internal_panic",
                "parser panic contained; no content returned",
            ),
        }
    }
}

#[tool_handler(
    name = "error-examiner-mcp",
    version = "0.1.0",
    instructions = "Use prepare_error_log with a path before reading a large log. Start with mode=auto; request compare or raw only when evidence is insufficient."
)]
impl ServerHandler for ErrorExaminerMcp {}

fn process(
    request: &PrepareRequest,
    request_id: &str,
) -> Result<ToolOutput, (String, String)> {
    let started = Instant::now();
    let loaded = load_input(request).map_err(|message| ("input_rejected".into(), message))?;
    let mode = Mode::parse(request.mode.as_deref())
        .map_err(|message| ("invalid_mode".into(), message))?;
    let budget = request
        .batch_chars
        .unwrap_or(DEFAULT_BATCH_CHARS)
        .clamp(MIN_BATCH_CHARS, MAX_BATCH_CHARS);
    let prepared = crate::preprocess::prepare(loaded.name.clone(), &loaded.text, budget);
    let raw = crate::preprocess::sanitized_raw(&loaded.text);
    let parsed = prepared.parsed_preview.clone();
    let raw_batches = crate::preprocess_raw::batches(&raw, budget);
    let parsed_batches = crate::preprocess_raw::batches(&parsed, budget);
    let index = request.batch_index.unwrap_or(0);
    let mut warnings = Vec::new();
    let (sent_variant, batches, raw_content, parsed_content) = match mode {
        Mode::Auto => (
            prepared.selected_variant.clone(),
            prepared.batches.clone(),
            None,
            None,
        ),
        Mode::Raw => ("Raw".into(), raw_batches.clone(), None, None),
        Mode::Parsed if prepared.normalized => {
            ("Parsed".into(), parsed_batches.clone(), None, None)
        }
        Mode::Parsed => {
            warnings.push("Unknown format: returned Raw fallback.".into());
            ("Raw fallback".into(), raw_batches.clone(), None, None)
        }
        Mode::Compare => {
            let count = raw_batches.len().max(parsed_batches.len());
            if index >= count {
                return Err((
                    "batch_out_of_range".into(),
                    format!("batch_index {index} is outside 0..{count}"),
                ));
            }
            (
                "Compare".into(),
                Vec::new(),
                raw_batches.get(index).cloned(),
                parsed_batches.get(index).cloned(),
            )
        }
    };
    let batch_count = if matches!(mode, Mode::Compare) {
        raw_batches.len().max(parsed_batches.len())
    } else {
        batches.len()
    };
    let content = if matches!(mode, Mode::Compare) {
        None
    } else {
        Some(batches.get(index).cloned().ok_or_else(|| {
            (
                "batch_out_of_range".into(),
                format!("batch_index {index} is outside 0..{batch_count}"),
            )
        })?)
    };
    let batch_size = content.as_deref().map(char_count).unwrap_or_else(|| {
        raw_content.as_deref().map(char_count).unwrap_or(0)
            + parsed_content.as_deref().map(char_count).unwrap_or(0)
    });
    let raw_chars = char_count(&raw);
    let parsed_chars = char_count(&parsed);
    let reduction = if raw_chars == 0 {
        0.0
    } else {
        (1.0 - parsed_chars as f64 / raw_chars as f64) * 100.0
    };
    let status = if prepared.normalized { "ok" } else { "raw_fallback" };
    let timestamp = chrono::Local::now().to_rfc3339();
    let client = request.client.as_deref().unwrap_or("mcp-client");
    let audit = AuditEvent {
        timestamp: &timestamp,
        request_id,
        client,
        tool: "prepare_error_log",
        input_name: &loaded.name,
        detected_format: &prepared.detected_format,
        status,
        raw_chars,
        parsed_chars,
        raw_tokens: token_estimate(raw_chars),
        parsed_tokens: token_estimate(parsed_chars),
        sent_variant: &sent_variant,
        events: prepared.event_count,
        important_events: prepared.important_events,
        duplicates: prepared.duplicate_count,
        batches: batch_count,
        duration_ms: started.elapsed().as_millis(),
        error_code: None,
    };
    let artifacts = request.save_artifacts.unwrap_or(false);
    crate::mcp_audit::persist(
        &audit,
        artifacts.then_some(loaded.text.as_str()),
        artifacts.then_some(parsed.as_str()),
    )
    .map_err(|message| ("audit_failed".into(), message))?;
    Ok(ToolOutput {
        contract_version: 1,
        status: status.into(),
        request_id: request_id.into(),
        input_name: loaded.name,
        detected_format: prepared.detected_format,
        requested_mode: mode.label().into(),
        sent_variant,
        raw_chars,
        parsed_chars,
        raw_tokens_estimate: token_estimate(raw_chars),
        parsed_tokens_estimate: token_estimate(parsed_chars),
        reduction_percent: reduction,
        event_count: prepared.event_count,
        important_events: prepared.important_events,
        duplicates_removed: prepared.duplicate_count,
        batch_index: index,
        batch_count,
        batch_chars: batch_size,
        batch_tokens_estimate: token_estimate(batch_size),
        content,
        raw_content,
        parsed_content,
        warnings,
        error_code: None,
    })
}

fn load_input(request: &PrepareRequest) -> Result<LoadedInput, String> {
    match (&request.path, &request.text) {
        (Some(_), Some(_)) | (None, None) => {
            Err("provide exactly one of path or text".into())
        }
        (None, Some(text)) => {
            if text.is_empty() {
                return Err("input is empty".into());
            }
            if text.len() > input_limit() {
                return Err(format!("inline text exceeds {} bytes", input_limit()));
            }
            Ok(LoadedInput {
                name: request.name.clone().unwrap_or_else(|| "inline.log".into()),
                text: text.clone(),
            })
        }
        (Some(path), None) => load_path(path),
    }
}

fn load_path(value: &str) -> Result<LoadedInput, String> {
    let path = PathBuf::from(value)
        .canonicalize()
        .map_err(|error| format!("cannot resolve path: {error}"))?;
    if !allowed_roots().iter().any(|root| path.starts_with(root)) {
        return Err("path is outside EE_MCP_ROOTS".into());
    }
    let metadata = path
        .metadata()
        .map_err(|error| format!("cannot inspect path: {error}"))?;
    if !metadata.is_file() {
        return Err("path is not a regular file".into());
    }
    let limit = input_limit();
    if metadata.len() > limit as u64 {
        return Err(format!("file exceeds {limit} bytes"));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    File::open(&path)
        .map_err(|error| format!("cannot open file: {error}"))?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read file: {error}"))?;
    if bytes.len() > limit {
        return Err(format!("file grew beyond {limit} bytes while reading"));
    }
    let text = String::from_utf8(bytes).map_err(|_| "file is not valid UTF-8".to_owned())?;
    Ok(LoadedInput {
        name: file_name(&path),
        text,
    })
}

fn allowed_roots() -> Vec<PathBuf> {
    let configured = std::env::var_os("EE_MCP_ROOTS")
        .map(|value| std::env::split_paths(&value).collect::<Vec<_>>())
        .unwrap_or_default();
    let roots = if configured.is_empty() {
        std::env::current_dir().into_iter().collect()
    } else {
        configured
    };
    roots
        .into_iter()
        .filter_map(|root| root.canonicalize().ok())
        .collect()
}

fn input_limit() -> usize {
    std::env::var("EE_MCP_MAX_INPUT_BYTES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_INPUT_BYTES)
        .clamp(1_024, 100 * 1024 * 1024)
}

fn error_response(
    request: &PrepareRequest,
    request_id: &str,
    duration_ms: u128,
    code: &str,
    message: &str,
) -> String {
    let timestamp = chrono::Local::now().to_rfc3339();
    let name = request
        .name
        .as_deref()
        .or_else(|| request.path.as_deref().and_then(|path| Path::new(path).file_name()?.to_str()))
        .unwrap_or("input");
    let client = request.client.as_deref().unwrap_or("mcp-client");
    let audit = AuditEvent {
        timestamp: &timestamp,
        request_id,
        client,
        tool: "prepare_error_log",
        input_name: name,
        detected_format: "unknown",
        status: "error",
        raw_chars: 0,
        parsed_chars: 0,
        raw_tokens: 0,
        parsed_tokens: 0,
        sent_variant: "none",
        events: 0,
        important_events: 0,
        duplicates: 0,
        batches: 0,
        duration_ms,
        error_code: Some(code),
    };
    let audit_error = crate::mcp_audit::persist(&audit, None, None).err();
    serialize(&serde_json::json!({
        "contract_version": 1,
        "status": "error",
        "request_id": request_id,
        "error_code": code,
        "message": message,
        "audit_error": audit_error,
    }))
}

fn request_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    format!(
        "{}-{:06}",
        chrono::Local::now().format("%Y%m%dT%H%M%S%.3f"),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

fn serialize<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| {
        r#"{"contract_version":1,"status":"error","error_code":"serialization_failed"}"#
            .into()
    })
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("input.log")
        .to_owned()
}

fn char_count(value: &str) -> usize {
    value.chars().count()
}

fn token_estimate(characters: usize) -> usize {
    characters.div_ceil(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_are_explicit() {
        assert_eq!(Mode::parse(None).unwrap().label(), "auto");
        assert_eq!(Mode::parse(Some("RAW")).unwrap().label(), "raw");
        assert!(Mode::parse(Some("guess")).is_err());
    }

    #[test]
    fn token_estimate_rounds_up() {
        assert_eq!(token_estimate(4), 2);
    }
}
