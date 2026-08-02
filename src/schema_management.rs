use crate::parser_registry::{FormatSchema, RegistryStatus, Schema};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};

const DEFAULT_PACK: &str = include_str!("../parser-catalog/executable-parser-v3.json");
pub(crate) const REPOSITORY_HINT: &str =
    "Download parser-catalog/executable-parser-v3.json from the project repository.";

#[derive(Default, Deserialize, Serialize)]
struct SchemaState {
    disabled: Vec<String>,
}

pub(crate) fn ensure_backup() -> Result<PathBuf, String> {
    let dir = crate::storage::app_dir().join("schema-backup");
    let path = dir.join("executable-parser-v3.json");
    if path.exists() {
        make_readonly(&path)?;
        make_readonly(&dir)?;
        return Ok(path);
    }
    if dir.exists() {
        return Err(format!(
            "Default schema backup is missing. {REPOSITORY_HINT}"
        ));
    }
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    fs::write(&path, DEFAULT_PACK).map_err(|error| error.to_string())?;
    make_readonly(&path)?;
    make_readonly(&dir)?;
    Ok(path)
}

pub(crate) fn disabled_ids() -> HashSet<String> {
    load_state().disabled.into_iter().collect()
}

pub(crate) fn set_disabled(id: &str, disabled: bool) -> Result<(), String> {
    let mut ids = disabled_ids();
    if disabled {
        ids.insert(id.to_owned());
    } else {
        ids.remove(id);
    }
    save_state(ids)
}

pub(crate) fn apply(merged: &mut Schema, status: &mut RegistryStatus) {
    for path in json_paths(crate::storage::app_dir().join("schema-overrides")) {
        match load_override(&path, merged) {
            Ok(format) => {
                if let Some(existing) = merged.formats.iter_mut().find(|item| item.id == format.id)
                {
                    *existing = format.clone();
                }
                if let Some(profile) = status.profiles.iter_mut().find(|item| item.id == format.id)
                {
                    profile.application = format.application.clone().unwrap_or(format.id.clone());
                    profile.origin = "override".into();
                }
            }
            Err(error) => status.rejected.push(format!("override: {error}")),
        }
    }
    let disabled = disabled_ids();
    for profile in &mut status.profiles {
        profile.active = !disabled.contains(&profile.id);
    }
    merged
        .formats
        .retain(|format| !disabled.contains(&format.id));
    status.active = merged.formats.len();
}

pub(crate) fn install_override(schema: &Schema) -> Result<PathBuf, String> {
    let format = schema.formats.first().ok_or("schema contains no formats")?;
    let dir = crate::storage::app_dir().join("schema-overrides");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let path = dir.join(format!("{}.json", format.id));
    let text = serde_json::to_string_pretty(schema).map_err(|error| error.to_string())?;
    fs::write(&path, text).map_err(|error| error.to_string())?;
    Ok(path)
}

pub(crate) fn stored_format(id: &str) -> Option<FormatSchema> {
    let dirs = [
        crate::storage::app_dir().join("schema-overrides"),
        crate::storage::app_dir().join("schemas"),
    ];
    dirs.into_iter()
        .flat_map(json_paths)
        .filter_map(|path| fs::read_to_string(path).ok())
        .filter_map(|text| serde_json::from_str::<Schema>(&text).ok())
        .flat_map(|schema| schema.formats)
        .find(|format| format.id == id)
}

pub(crate) fn restore_defaults() -> Result<String, String> {
    let backup = crate::storage::app_dir()
        .join("schema-backup")
        .join("executable-parser-v3.json");
    let text = fs::read_to_string(&backup)
        .map_err(|_| format!("Default schema backup is missing. {REPOSITORY_HINT}"))?;
    let schema: Schema = serde_json::from_str(&text)
        .map_err(|error| format!("Default schema backup is invalid: {error}"))?;
    crate::parser_registry::validate_schema(&schema)?;
    save_state(HashSet::new())?;
    let overrides = crate::storage::app_dir().join("schema-overrides");
    if overrides.exists() {
        let trash = crate::storage::app_dir().join("schema-trash");
        fs::create_dir_all(&trash).map_err(|error| error.to_string())?;
        let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
        fs::rename(&overrides, trash.join(format!("{stamp}-overrides")))
            .map_err(|error| error.to_string())?;
    }
    Ok("Defaults restored; overrides moved to schema-trash.".into())
}

fn load_override(path: &PathBuf, merged: &Schema) -> Result<FormatSchema, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let schema: Schema = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    crate::parser_registry::validate_schema(&schema)?;
    if schema.formats.len() != 1 {
        return Err("override must contain exactly one format".into());
    }
    let format = schema.formats.into_iter().next().unwrap();
    if !merged.formats.iter().any(|item| item.id == format.id) {
        return Err(format!("unknown profile: {}", format.id));
    }
    Ok(format)
}

fn state_path() -> PathBuf {
    crate::storage::app_dir().join("schema-state.json")
}

fn load_state() -> SchemaState {
    fs::read_to_string(state_path())
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_state(ids: HashSet<String>) -> Result<(), String> {
    let mut disabled: Vec<String> = ids.into_iter().collect();
    disabled.sort();
    let text = serde_json::to_string_pretty(&SchemaState { disabled })
        .map_err(|error| error.to_string())?;
    fs::write(state_path(), text).map_err(|error| error.to_string())
}

fn json_paths(dir: PathBuf) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<_> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn make_readonly(path: &PathBuf) -> Result<(), String> {
    let mut permissions = fs::metadata(path)
        .map_err(|error| error.to_string())?
        .permissions();
    permissions.set_readonly(true);
    fs::set_permissions(path, permissions).map_err(|error| error.to_string())
}
