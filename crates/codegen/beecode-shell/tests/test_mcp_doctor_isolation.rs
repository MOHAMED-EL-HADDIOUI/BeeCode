//! Isolated binary so `beecode_home()`'s process-wide OnceLock initializes from
//! our `BEECODE_HOME`. A lib-test EnvGuard is a no-op if another test already
//! resolved it, and then doctor reads the real ~/.beecode.

use std::path::PathBuf;
use std::sync::OnceLock;

fn isolate_home() -> &'static PathBuf {
    static HOME: OnceLock<PathBuf> = OnceLock::new();
    HOME.get_or_init(|| {
        let dir = tempfile::TempDir::new().unwrap().keep();
        let beecode = dir.join(".beecode");
        std::fs::create_dir_all(&beecode).unwrap();
        std::fs::write(beecode.join("config.toml"), "").unwrap();
        // SAFETY: this binary's only test; set before any beecode_home() call.
        unsafe {
            std::env::set_var("HOME", &dir);
            std::env::set_var("USERPROFILE", &dir);
            std::env::set_var("BEECODE_HOME", &beecode);
        }
        dir
    })
}

#[tokio::test]
async fn run_doctor_skips_managed_gateway_without_configs_probe() {
    let _home = isolate_home();
    let cwd = tempfile::tempdir().unwrap();

    let report = beecode_shell::mcp_doctor::run_doctor(cwd.path(), None).await;
    assert!(
        !report.sources.iter().any(|s| s.path == "beecode.com"),
        "doctor must not invent a beecode.com source: {:?}",
        report.sources
    );
    assert!(
        report.servers.is_empty(),
        "isolated cwd must not probe managed HTTP servers: {:?}",
        report.servers
    );
}
