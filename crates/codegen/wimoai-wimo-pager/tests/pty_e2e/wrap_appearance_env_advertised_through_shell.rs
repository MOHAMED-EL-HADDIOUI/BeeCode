// Per-test-case module for the `pty_e2e` integration test crate.
#[allow(unused_imports)]
use super::common::*;

/// A single argv containing whitespace routes through `$SHELL -i -c`, the same hop OSC 52 takes.
const PRINT_APPEARANCE: &str =
    "printf 'wimo=%s lc=%s\\n' \"$wimo_APPEARANCE\" \"$LC_wimo_APPEARANCE\"";

fn parse_printed_appearance(raw: &str) -> Option<(String, String)> {
    let line = raw.lines().find(|l| l.starts_with("wimo="))?;
    let rest = line.strip_prefix("wimo=")?;
    let (wimo, lc) = rest.split_once(" lc=")?;
    Some((wimo.to_owned(), lc.to_owned()))
}

/// End-to-end check that the appearance stamp survives the interactive shell hop.
///
/// The parent pins wimo_APPEARANCE and LC_wimo_APPEARANCE empty.
/// `COLORFGBG` is a dark hint `detect()` would honor, so a wrap that invented polarity from it would stamp `dark`.
/// Do not call `detect_desktop()` here: two live portal probes can disagree.
#[test]
#[ignore = "PTY e2e; run the owning pty_e2e_* Cargo test with --ignored (see Cargo.toml)"]
#[cfg(unix)]
fn wrap_appearance_env_advertised_through_shell() {
    let (code, raw) = run_wrap(
        &[PRINT_APPEARANCE],
        &[
            ("SHELL", "/bin/sh"),
            ("COLORFGBG", "15;0"),
            ("wimo_APPEARANCE", ""),
            ("LC_wimo_APPEARANCE", ""),
        ],
    );
    let (wimo, lc) = parse_printed_appearance(&raw)
        .unwrap_or_else(|| panic!("missing wimo=/lc= line\nraw:\n{raw}"));
    match (wimo.as_str(), lc.as_str()) {
        ("", "") => {}
        ("dark", "dark") | ("light", "light") => {}
        _ => panic!(
            "wimo and LC must agree and not invent from COLORFGBG; wimo={wimo:?} lc={lc:?}\nraw:\n{raw}"
        ),
    }
    assert_eq!(
        code,
        Some(0),
        "shell-routed printf must exit 0\nraw:\n{raw}"
    );
}

/// The parent sets `wimo_APPEARANCE=light` and pins LC empty.
/// A desktop probe that answers overrides both names to the same polarity; one that answers `None` inherits wimo and must not invent LC.
/// The test itself never probes the desktop; a second live probe could disagree.
#[test]
#[ignore = "PTY e2e; run the owning pty_e2e_* Cargo test with --ignored (see Cargo.toml)"]
#[cfg(unix)]
fn wrap_appearance_env_desktop_none_does_not_restamp_parent_wimo() {
    let (code, raw) = run_wrap(
        &[PRINT_APPEARANCE],
        &[
            ("SHELL", "/bin/sh"),
            ("wimo_APPEARANCE", "light"),
            ("LC_wimo_APPEARANCE", ""),
        ],
    );
    let (wimo, lc) = parse_printed_appearance(&raw)
        .unwrap_or_else(|| panic!("missing wimo=/lc= line\nraw:\n{raw}"));
    match (wimo.as_str(), lc.as_str()) {
        ("light", "") => {}
        ("dark", "dark") | ("light", "light") => {}
        _ => panic!(
            "expected inherit wimo=light with empty lc, or a matching desktop stamp; wimo={wimo:?} lc={lc:?}\nraw:\n{raw}"
        ),
    }
    assert_eq!(
        code,
        Some(0),
        "shell-routed printf must exit 0\nraw:\n{raw}"
    );
}
