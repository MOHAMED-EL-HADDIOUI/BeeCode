//! `BEECODE_HOME` override tests in an isolated binary so `beecode_home()`'s process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
#[serial_test::serial(BEECODE_HOME)]
fn beecode_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let beecode_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("BEECODE_HOME", &beecode_home);
    }

    assert_eq!(
        beecode_pager::util::pager_toml_path(),
        beecode_home.join("pager.toml")
    );
    assert_eq!(
        beecode_pager::util::display_beecode_home_prefix(),
        "$BEECODE_HOME"
    );
    assert_eq!(
        beecode_pager::util::display_user_beecode_path("config.toml"),
        "$BEECODE_HOME/config.toml"
    );

    let memory_path = beecode_home.join("memory/MEMORY.md");
    assert_eq!(
        beecode_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$BEECODE_HOME/memory/MEMORY.md"
    );

    // The copy toast abbreviates paths the same way, so a custom $BEECODE_HOME outside $HOME still shows the short form
    assert_eq!(
        beecode_pager::clipboard::display_copy_path(&beecode_home.join("last-copy.txt")),
        "$BEECODE_HOME/last-copy.txt"
    );

    assert!(beecode_pager::util::is_under_user_beecode_home(
        &memory_path
    ));
    assert!(!beecode_pager::util::is_under_user_beecode_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}

/// Isolated because `beecode_home()`'s `OnceLock` is already initialized by the time the shared lib-test binary reaches a case like this.
#[test]
#[serial_test::serial(BEECODE_HOME)]
fn disk_usage_run_creates_no_beecode_home() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ghost = tmp.path().join("ghost-home");
    unsafe {
        std::env::set_var("BEECODE_HOME", &ghost);
    }

    for json in [false, true] {
        beecode_pager::disk_usage_cmd::run(beecode_pager::disk_usage_cmd::DiskUsageArgs { json })
            .expect("a missing home is not an error");
        assert!(
            !ghost.exists(),
            "beecode du must not create the home it reports on (json={json})"
        );
    }
}
