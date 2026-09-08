//! `wimo_HOME` override tests in an isolated binary so `wimo_home()`'s process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
#[serial_test::serial(wimo_HOME)]
fn wimo_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let wimo_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("wimo_HOME", &wimo_home);
    }

    assert_eq!(
        wimo ai_wimo_pager::util::pager_toml_path(),
        wimo_home.join("pager.toml")
    );
    assert_eq!(
        wimo ai_wimo_pager::util::display_wimo_home_prefix(),
        "$wimo_HOME"
    );
    assert_eq!(
        wimo ai_wimo_pager::util::display_user_wimo_path("config.toml"),
        "$wimo_HOME/config.toml"
    );

    let memory_path = wimo_home.join("memory/MEMORY.md");
    assert_eq!(
        wimo ai_wimo_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$wimo_HOME/memory/MEMORY.md"
    );

    // The copy toast abbreviates paths the same way, so a custom $wimo_HOME outside $HOME still shows the short form
    assert_eq!(
        wimo ai_wimo_pager::clipboard::display_copy_path(&wimo_home.join("last-copy.txt")),
        "$wimo_HOME/last-copy.txt"
    );

    assert!(wimo ai_wimo_pager::util::is_under_user_wimo_home(&memory_path));
    assert!(!wimo ai_wimo_pager::util::is_under_user_wimo_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}

/// Isolated because `wimo_home()`'s `OnceLock` is already initialized by the time the shared lib-test binary reaches a case like this.
#[test]
#[serial_test::serial(wimo_HOME)]
fn disk_usage_run_creates_no_wimo_home() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ghost = tmp.path().join("ghost-home");
    unsafe {
        std::env::set_var("wimo_HOME", &ghost);
    }

    for json in [false, true] {
        wimo ai_wimo_pager::disk_usage_cmd::run(wimo ai_wimo_pager::disk_usage_cmd::DiskUsageArgs { json })
            .expect("a missing home is not an error");
        assert!(
            !ghost.exists(),
            "wimo du must not create the home it reports on (json={json})"
        );
    }
}
