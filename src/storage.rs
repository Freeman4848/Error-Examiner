use serde::{de::DeserializeOwned, Serialize};
use std::{env, fs, path::PathBuf};

pub fn app_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let base = env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(not(target_os = "windows"))]
    let base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")));

    let base = base.unwrap_or_else(|| PathBuf::from("."));
    let current = base.join("error-examiner");
    // Keep the former directory name so existing users retain their settings.
    let legacy = base.join("error-explainer");
    if !current.exists() && legacy.exists() && fs::rename(&legacy, &current).is_err() {
        return legacy;
    }
    current
}

pub fn load_json<T: DeserializeOwned + Default>(name: &str) -> T {
    fs::read_to_string(app_dir().join(name))
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save_json<T: Serialize>(name: &str, value: &T) -> Result<(), String> {
    let dir = app_dir();
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let data = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    fs::write(dir.join(name), data).map_err(|error| error.to_string())
}
