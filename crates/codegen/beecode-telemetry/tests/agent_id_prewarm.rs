//! This test runs in its own process because the assertions consume process-global state.

#[test]
fn prefetched_agent_id_resolves_and_persists() {
    let home = tempfile::tempdir().expect("tempdir");
    // SAFETY: single-threaded here; set before anything caches `beecode_home()`.
    unsafe {
        std::env::remove_var("beecode_AGENT_ID");
        std::env::set_var("BEECODE_HOME", home.path());
    }
    beecode_telemetry::id::prefetch_agent_id();
    let id = beecode_telemetry::id::agent_id();
    assert_eq!(
        std::fs::read_to_string(home.path().join("agent_id"))
            .expect("agent_id cache")
            .trim(),
        id
    );
}
