//! Project-local settings: `<project>/.beecode/settings.json`.
//!
//! Discovered by walking up from the process working directory (nearest
//! `.beecode/settings.json` wins), so running from a subdirectory of a project
//! still picks up the project settings. This is the home for per-project LLM
//! provider configuration — OpenAI / Anthropic endpoints and keys:
//!
//! ```json
//! {
//!   "models": { "default": "gpt-5" },
//!   "model": {
//!     "gpt-5": {
//!       "model": "gpt-5",
//!       "base_url": "https://api.openai.com/v1",
//!       "api_backend": "responses",
//!       "env_key": "OPENAI_API_KEY"
//!     },
//!     "claude": {
//!       "model": "anthropic/claude-opus-4-6",
//!       "base_url": "https://api.anthropic.com/v1",
//!       "api_backend": "messages",
//!       "env_key": "ANTHROPIC_API_KEY"
//!     }
//!   }
//! }
//! ```
//!
//! The layer merges into the **user tier above the BeeCode home config** and
//! below the `BEECODE_CONFIG` overlay and `requirements.toml`, so enterprise pins
//! still win. Unlike the confined `BEECODE_CONFIG` overlay (which drops the
//! per-model `[model.*]` table), this layer is full-power like `config.toml`.
//!
//! Trust note: a cloned repo can plant this file, and `[model.*]` entries
//! choose where prompts are sent (`base_url`) and with what credential.
//! Only run BeeCode in projects you trust, and never commit real API keys —
//! prefer `env_key` pointing at environment variables over inline `api_key`.

use std::path::{Path, PathBuf};

use crate::loader::{apply_version_overrides_with_registered, expand_env_vars_in_toml};

/// Preferred project settings directory name, resolved against the project root.
pub const PROJECT_SETTINGS_DIRNAME: &str = ".beecode";

/// Historical project directory accepted during the Wimo → BeeCode migration.
pub const LEGACY_PROJECT_SETTINGS_DIRNAME: &str = ".wimo";

/// Project settings filename inside [`PROJECT_SETTINGS_DIRNAME`].
pub const PROJECT_SETTINGS_FILENAME: &str = "settings.json";

/// Hard cap on a project settings read (matches the overlay cap).
/// A huge file, or a special node like `/dev/zero`, must never stall or OOM
/// the agent, so the read is bounded. Over-cap or non-regular files are
/// ignored, the same as a missing file.
const MAX_SETTINGS_BYTES: u64 = 4 * 1024 * 1024;

/// Bound on the upward directory walk (filesystem root always terminates it).
const MAX_WALK_DEPTH: usize = 64;

/// Find the nearest BeeCode project settings file at or above `start`.
/// At a given directory `.beecode/settings.json` wins over the legacy
/// `.wimo/settings.json`; the legacy path is read only as a compatibility fallback.
pub fn find_project_settings_file(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    for _ in 0..MAX_WALK_DEPTH {
        for dirname in [PROJECT_SETTINGS_DIRNAME, LEGACY_PROJECT_SETTINGS_DIRNAME] {
            let candidate = dir.join(dirname).join(PROJECT_SETTINGS_FILENAME);
            match std::fs::symlink_metadata(&candidate) {
                Ok(meta) if meta.file_type().is_file() => return Some(candidate),
                _ => {}
            }
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

/// Load and process the project settings layer found from `start`.
/// Runs the `config.toml`-equivalent pipeline minus normalization (the merged
/// user layer is normalized once in [`crate::ConfigLayers::load`]):
/// `$VAR` expansion, then this layer's `version_overrides`.
/// Returns `None` when no file is found, it is unreadable/invalid, or it
/// carries no settings.
pub fn load_project_settings_from(start: &Path) -> Option<toml::Value> {
    let path = find_project_settings_file(start)?;
    let raw = read_capped_settings_file(&path)?;
    let mut value = crate::env_overlay::parse_json_layer(&raw)?;
    expand_env_vars_in_toml(&mut value);
    if let Err(e) = apply_version_overrides_with_registered(&mut value) {
        tracing::warn!(path = %path.display(), error = %e, "project settings.json `version_overrides` failed to apply; ignoring the layer");
        return None;
    }
    if value.as_table().is_some_and(|t| t.is_empty()) {
        return None;
    }
    if path
        .components()
        .any(|component| {
            component.as_os_str()
                == std::ffi::OsStr::new(LEGACY_PROJECT_SETTINGS_DIRNAME)
        })
    {
        tracing::warn!(path = %path.display(), "loaded deprecated .wimo/settings.json; move it to .beecode/settings.json");
    } else {
        tracing::debug!(path = %path.display(), "loaded project .beecode/settings.json");
    }
    Some(value)
}

/// Load the project settings layer for this process (walk-up from the
/// current working directory). `None` when the CWD is unavailable or no
/// layer resolves — callers treat that as "no project settings".
pub fn load_project_settings() -> Option<toml::Value> {
    let cwd = std::env::current_dir().ok()?;
    load_project_settings_from(&cwd)
}

/// Bounded read of a project settings file. `None` when missing, unreadable,
/// not a regular file, or over the size cap. Never logs content.
fn read_capped_settings_file(path: &Path) -> Option<String> {
    use std::io::Read;

    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            tracing::warn!(path = %path.display(), error = %e, "project settings.json is unreadable; ignoring the layer");
            return None;
        }
    };
    match file.metadata() {
        Ok(meta) if !meta.file_type().is_file() => {
            tracing::warn!(path = %path.display(), "project settings.json is not a regular file; ignoring the layer");
            return None;
        }
        Ok(_) => {}
        Err(e) => {
            tracing::warn!(path = %path.display(), error = %e, "project settings.json is unreadable; ignoring the layer");
            return None;
        }
    }
    let mut raw = String::new();
    if let Err(e) = file.take(MAX_SETTINGS_BYTES + 1).read_to_string(&mut raw) {
        tracing::warn!(path = %path.display(), error = %e, "project settings.json is unreadable; ignoring the layer");
        return None;
    }
    if raw.len() as u64 > MAX_SETTINGS_BYTES {
        tracing::warn!(path = %path.display(), max = MAX_SETTINGS_BYTES, "project settings.json exceeds the max size; ignoring the layer");
        return None;
    }
    Some(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_settings(dir: &Path, body: &str) -> PathBuf {
        let wimo_dir = dir.join(PROJECT_SETTINGS_DIRNAME);
        std::fs::create_dir_all(&wimo_dir).unwrap();
        let path = wimo_dir.join(PROJECT_SETTINGS_FILENAME);
        std::fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn missing_file_resolves_to_no_layer() {
        let dir = tempfile::TempDir::new().unwrap();
        assert_eq!(find_project_settings_file(dir.path()), None);
        assert!(load_project_settings_from(dir.path()).is_none());
    }

    #[test]
    fn nearest_settings_wins_walking_up() {
        let dir = tempfile::TempDir::new().unwrap();
        write_settings(dir.path(), r#"{"models": {"default": "outer"}}"#);
        let inner = dir.path().join("a").join("b");
        std::fs::create_dir_all(&inner).unwrap();
        write_settings(&inner, r#"{"models": {"default": "inner"}}"#);

        assert_eq!(
            find_project_settings_file(&inner),
            Some(
                inner
                    .join(PROJECT_SETTINGS_DIRNAME)
                    .join(PROJECT_SETTINGS_FILENAME)
            )
        );
        let v = load_project_settings_from(&inner).unwrap();
        assert_eq!(
            v.get("models")
                .and_then(|m| m.get("default"))
                .and_then(|d| d.as_str()),
            Some("inner")
        );

        // From the outer dir (child removed from the picture) the outer file wins.
        let v = load_project_settings_from(dir.path()).unwrap();
        assert_eq!(
            v.get("models")
                .and_then(|m| m.get("default"))
                .and_then(|d| d.as_str()),
            Some("outer")
        );
    }

    #[test]
    fn beecode_settings_win_over_legacy_wimo_settings() {
        let dir = tempfile::TempDir::new().unwrap();
        let legacy = dir.path().join(LEGACY_PROJECT_SETTINGS_DIRNAME);
        std::fs::create_dir_all(&legacy).unwrap();
        std::fs::write(legacy.join(PROJECT_SETTINGS_FILENAME), r#"{"models":{"default":"legacy"}}"#).unwrap();
        write_settings(dir.path(), r#"{"models":{"default":"beecode"}}"#);

        let value = load_project_settings_from(dir.path()).unwrap();
        assert_eq!(value["models"]["default"].as_str(), Some("beecode"));
    }

    #[test]
    fn legacy_wimo_settings_remain_a_compatible_fallback() {
        let dir = tempfile::TempDir::new().unwrap();
        let legacy = dir.path().join(LEGACY_PROJECT_SETTINGS_DIRNAME);
        std::fs::create_dir_all(&legacy).unwrap();
        let expected = legacy.join(PROJECT_SETTINGS_FILENAME);
        std::fs::write(&expected, r#"{"models":{"default":"legacy"}}"#).unwrap();

        assert_eq!(find_project_settings_file(dir.path()), Some(expected));
    }

    #[test]
    fn invalid_json_and_non_tables_are_ignored() {
        let dir = tempfile::TempDir::new().unwrap();
        write_settings(dir.path(), r#"{"models": {"#);
        assert!(load_project_settings_from(dir.path()).is_none());

        write_settings(dir.path(), r#"[1, 2, 3]"#);
        assert!(load_project_settings_from(dir.path()).is_none());

        write_settings(dir.path(), r#"{}"#);
        assert!(load_project_settings_from(dir.path()).is_none());
    }

    #[test]
    fn model_endpoints_and_env_expansion_survive() {
        let dir = tempfile::TempDir::new().unwrap();
        write_settings(
            dir.path(),
            r#"{
                "models": {"default": "gpt-5"},
                "model": {
                    "gpt-5": {
                        "model": "gpt-5",
                        "base_url": "https://api.openai.com/v1",
                        "api_backend": "responses",
                        "env_key": "OPENAI_API_KEY",
                        "extra_headers": {"X-From": "$wimo_TEST_MARKER"}
                    }
                }
            }"#,
        );
        // `std::env::set_var` is `unsafe` in edition 2024; this test mutates
        // process env, so it must not run in parallel with env readers.
        // `wimo_TEST_MARKER` is unique to this test.
        unsafe { std::env::set_var("wimo_TEST_MARKER", "expanded") };
        let v = load_project_settings_from(dir.path()).unwrap();
        unsafe { std::env::remove_var("wimo_TEST_MARKER") };
        let entry = &v["model"]["gpt-5"];
        assert_eq!(
            entry["base_url"].as_str(),
            Some("https://api.openai.com/v1")
        );
        assert_eq!(entry["api_backend"].as_str(), Some("responses"));
        assert_eq!(entry["extra_headers"]["X-From"].as_str(), Some("expanded"));
    }
}
