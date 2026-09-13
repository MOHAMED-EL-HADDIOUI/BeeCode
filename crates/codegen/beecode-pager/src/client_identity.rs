pub const PAGER_CLIENT_TYPE: &str = "beecode-pager";
pub const HEADLESS_CLIENT_TYPE: &str = "beecode-shell";

pub const PAGER_CLIENT_VERSION: &str = beecode_version::VERSION;

/// `User-Agent` for the pager's own HTTP clients that call `api.x.ai` directly (voice STT).
///
/// Matches the sampler's `beecode-shell/<version> (os; arch)` shape so server-side dashboards bucket voice traffic alongside chat / imagine requests.
pub fn client_user_agent() -> String {
    format!(
        "{}/{} ({}; {})",
        HEADLESS_CLIENT_TYPE,
        PAGER_CLIENT_VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_user_agent_has_expected_shape() {
        // e.g. "beecode-shell/1.2.3 (macos; aarch64)".
        // Servers parse this UA string, so pin the exact shape
        let ua = client_user_agent();
        assert_eq!(
            ua,
            format!(
                "beecode-shell/{} ({}; {})",
                PAGER_CLIENT_VERSION,
                std::env::consts::OS,
                std::env::consts::ARCH
            )
        );
    }
}
