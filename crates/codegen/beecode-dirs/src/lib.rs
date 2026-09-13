//! Home-directory resolution generally: USERPROFILE-first `home_dir`, plus
//! the BeeCode home (`$BEECODE_HOME` or `<home>/.beecode`). Shared by
//! `beecode-config` and `beecode-fast-worktree`.
//!
//! Which function to call:
//! - [`beecode_home`]: the usual choice, a cached, created path to build on.
//! - [`user_beecode_home`]: `None` instead of a cwd fallback when no home resolves.
//! - [`default_beecode_home`]: the preferred `<home>/.beecode` default.
//! - [`resolve_beecode_home`]: a fresh, uncached resolve.
//! - [`resolve_beecode_home_with_source`]: [`resolve_beecode_home`] plus where the path came from.
//! - [`home_dir`]: the home directory itself, for sibling dot dirs (`~/.claude`, `~/.agents`, ...).
//!
//! TODO: collapse these getters by threading the path through config as an
//! explicit value.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Where a resolved BeeCode home came from, so "why did BeeCode pick this
/// directory?" is answerable in diagnostics without re-reading the
/// environment at the asking site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeecodeHomeSource {
    /// A non-empty `$BEECODE_HOME` override.
    EnvOverride,
    /// `<home>/.beecode` derived from the home directory.
    HomeDefault,
}

/// The user's home directory via [`std::env::home_dir`]: `HOME` on Unix (with
/// a passwd fallback), `USERPROFILE` on Windows.
///
/// Deliberately not `dirs::home_dir()`: on Windows `dirs` asks the
/// known-folder API and ignores a redirected `USERPROFILE`, while this crate
/// resolves `~/.beecode` from the profile variable — mixing the two sources puts
/// the BeeCode directory and other home-anchored dot directories in different
/// trees. Every home-anchored path must come from this one function.
#[allow(deprecated, clippy::disallowed_methods)] // the one sanctioned std::env::home_dir call
pub fn home_dir() -> Option<PathBuf> {
    std::env::home_dir()
}

/// `<home>/.beecode`, canonicalized via `dunce` (not `std::fs::canonicalize`,
/// which yields Windows `\\?\` verbatim paths).
fn beecode_home_in(home: &Path) -> PathBuf {
    dunce::canonicalize(home)
        .unwrap_or_else(|_| home.to_path_buf())
        .join(".beecode")
}

/// `$BEECODE_HOME` verbatim when non-empty, else `<home>/.beecode`.
/// Env values are used as-is (not canonicalized) so
/// they stay stable and comparable: callers do literal prefix checks against
/// them, and downstream symlink guards must still see their original
/// components.
fn resolve_beecode_home_from(
    beecode_home_env: Option<&OsStr>,
    os_home: Option<&Path>,
) -> Option<(PathBuf, BeecodeHomeSource)> {
    if let Some(env) = beecode_home_env.filter(|env| !env.is_empty()) {
        return Some((PathBuf::from(env), BeecodeHomeSource::EnvOverride));
    }
    let home = os_home?;
    let preferred = beecode_home_in(home);
    Some((preferred, BeecodeHomeSource::HomeDefault))
}

/// Resolve the BeeCode home from the environment (fresh, no cache); `None` if neither resolves.
pub fn resolve_beecode_home() -> Option<PathBuf> {
    resolve_beecode_home_with_source().map(|(home, _)| home)
}

/// [`resolve_beecode_home`] plus the [`BeecodeHomeSource`] the path came from.
pub fn resolve_beecode_home_with_source() -> Option<(PathBuf, BeecodeHomeSource)> {
    resolve_beecode_home_from(
        std::env::var_os("BEECODE_HOME").as_deref(),
        home_dir().as_deref(),
    )
}

/// The preferred `<home>/.beecode` default.
pub fn default_beecode_home() -> PathBuf {
    beecode_home_in(&home_dir().unwrap_or_else(|| PathBuf::from(".")))
}

/// The BeeCode home, created if missing and cached for the process; falls back
/// to [`default_beecode_home`] when nothing resolves.
pub fn beecode_home() -> PathBuf {
    static BEECODE_HOME: OnceLock<PathBuf> = OnceLock::new();
    BEECODE_HOME
        .get_or_init(|| {
            let home = resolve_beecode_home().unwrap_or_else(default_beecode_home);
            if let Err(err) = std::fs::create_dir_all(&home) {
                tracing::warn!(path = %home.display(), %err, "failed to create BeeCode home");
            }
            home
        })
        .clone()
}

/// Like [`beecode_home`], but `None` when no home resolves (no cwd fallback).
pub fn user_beecode_home() -> Option<PathBuf> {
    resolve_beecode_home().is_some().then(beecode_home)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::ffi::OsString;

    #[test]
    fn beecode_env_wins_over_os_home() {
        let resolved = resolve_beecode_home_from(
            Some(OsStr::new("/custom/beecode")),
            Some(Path::new("/home/u")),
        );
        assert_eq!(
            resolved,
            Some((
                PathBuf::from("/custom/beecode"),
                BeecodeHomeSource::EnvOverride
            ))
        );
    }

    #[test]
    fn env_used_verbatim_even_when_it_exists() {
        // A real, existing dir whose canonical form differs (macOS symlinks
        // `/var` -> `/private/var`): the env value must come back unchanged.
        let tmp = tempfile::tempdir().unwrap();
        let resolved = resolve_beecode_home_from(Some(tmp.path().as_os_str()), None);
        assert_eq!(
            resolved,
            Some((tmp.path().to_path_buf(), BeecodeHomeSource::EnvOverride))
        );
    }

    #[test]
    fn empty_env_falls_through_to_os_home() {
        let tmp = tempfile::tempdir().unwrap();
        let resolved = resolve_beecode_home_from(Some(&OsString::new()), Some(tmp.path()));
        assert_eq!(
            resolved,
            Some((
                dunce::canonicalize(tmp.path()).unwrap().join(".beecode"),
                BeecodeHomeSource::HomeDefault
            ))
        );
    }

    #[test]
    fn existing_beecode_dir_resolves() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".beecode")).unwrap();
        let resolved = resolve_beecode_home_from(None, Some(tmp.path()));
        assert_eq!(
            resolved,
            Some((
                dunce::canonicalize(tmp.path()).unwrap().join(".beecode"),
                BeecodeHomeSource::HomeDefault
            ))
        );
    }

    #[test]
    fn fresh_home_defaults_to_beecode() {
        let tmp = tempfile::tempdir().unwrap();
        let resolved = resolve_beecode_home_from(None, Some(tmp.path()));
        assert_eq!(
            resolved,
            Some((
                dunce::canonicalize(tmp.path()).unwrap().join(".beecode"),
                BeecodeHomeSource::HomeDefault
            ))
        );
    }

    #[test]
    fn default_beecode_home_has_no_verbatim_prefix() {
        // The reason we canonicalize via dunce: std::fs::canonicalize yields
        // `\\?\` verbatim paths on Windows that break git and byte-exact
        // comparisons. No-op assertion on Unix.
        let home = default_beecode_home();
        assert!(!home.to_string_lossy().starts_with(r"\\?\"));
        assert!(home.ends_with(".beecode"));
    }

    #[test]
    fn none_when_nothing_resolves() {
        assert_eq!(
            resolve_beecode_home_from(/* beecode_home_env */ None, /* os_home */ None),
            None
        );
    }
}
