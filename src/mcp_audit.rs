use serde::Serialize;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

#[derive(Serialize)]
pub(crate) struct AuditEvent<'a> {
    pub(crate) timestamp: &'a str,
    pub(crate) request_id: &'a str,
    pub(crate) client: &'a str,
    pub(crate) tool: &'a str,
    pub(crate) input_name: &'a str,
    pub(crate) detected_format: &'a str,
    pub(crate) status: &'a str,
    pub(crate) raw_chars: usize,
    pub(crate) parsed_chars: usize,
    pub(crate) raw_tokens: usize,
    pub(crate) parsed_tokens: usize,
    pub(crate) sent_variant: &'a str,
    pub(crate) events: usize,
    pub(crate) important_events: usize,
    pub(crate) duplicates: usize,
    pub(crate) batches: usize,
    pub(crate) duration_ms: u128,
    pub(crate) error_code: Option<&'a str>,
}

pub(crate) fn persist(
    event: &AuditEvent<'_>,
    raw: Option<&str>,
    parsed: Option<&str>,
) -> Result<PathBuf, String> {
    let _guard = audit_lock()
        .lock()
        .map_err(|_| "audit lock poisoned".to_owned())?;
    let root = data_root();
    fs::create_dir_all(&root).map_err(|error| format!("audit directory: {error}"))?;
    let audit_path = root.join("events.jsonl");
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&audit_path)
        .map_err(|error| format!("audit open: {error}"))?;
    let mut line = serde_json::to_vec(event).map_err(|error| format!("audit JSON: {error}"))?;
    line.push(b'\n');
    file.write_all(&line)
        .and_then(|_| file.flush())
        .map_err(|error| format!("audit write: {error}"))?;
    if let (Some(raw), Some(parsed)) = (raw, parsed) {
        save_artifacts(&root, event.request_id, event.input_name, raw, parsed)?;
    }
    Ok(audit_path)
}

fn save_artifacts(
    root: &Path,
    request_id: &str,
    input_name: &str,
    raw: &str,
    parsed: &str,
) -> Result<(), String> {
    let dir = root.join("artifacts").join(sanitize(request_id));
    fs::create_dir_all(&dir).map_err(|error| format!("artifact directory: {error}"))?;
    let name = sanitize(input_name);
    atomic_write(&dir.join(format!("{name}.raw.log")), raw)?;
    atomic_write(&dir.join(format!("{name}.parsed.log")), parsed)
}

fn atomic_write(path: &Path, text: &str) -> Result<(), String> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, text).map_err(|error| format!("artifact write: {error}"))?;
    fs::rename(&temp, path).map_err(|error| format!("artifact commit: {error}"))
}

fn data_root() -> PathBuf {
    std::env::var_os("EE_MCP_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::storage::app_dir().join("mcp"))
}

fn sanitize(value: &str) -> String {
    let value: String = value
        .chars()
        .take(80)
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if value.is_empty() {
        "input".into()
    } else {
        value
    }
}

fn audit_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_names_cannot_escape_the_directory() {
        assert_eq!(sanitize("../../secret log"), ".._.._secret_log");
    }
}
