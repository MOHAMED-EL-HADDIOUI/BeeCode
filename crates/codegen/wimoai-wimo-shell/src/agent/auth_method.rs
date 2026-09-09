use agent_client_protocol as acp;

use crate::agent::config::ModelEntry;
use crate::auth::PreferredAuthMethod;

/// Shared, live handle to the agent's current ACP auth method id.
///
/// `Arc` so a clone can cross the per-session-thread boundary at spawn.
/// The `ArcSwapOption` interior lets the agent's `authenticate` handler publish a new method without re-spawning sessions.
/// Every running session's per-turn auth gate observes the new method on its next turn.
/// `None` until the first `authenticate`.
/// Auth is process-global (one user, one `AuthManager`), so all sessions sharing one cell is correct.
pub(crate) type SharedAuthMethodId = std::sync::Arc<arc_swap::ArcSwapOption<acp::AuthMethodId>>;

/// Construct a [`SharedAuthMethodId`]. `None` is the pre-`authenticate` state.
pub(crate) fn new_shared_auth_method_id(initial: Option<acp::AuthMethodId>) -> SharedAuthMethodId {
    std::sync::Arc::new(arc_swap::ArcSwapOption::new(
        initial.map(std::sync::Arc::new),
    ))
}

/// Env var that, when set, advertises `wimoai.api_key` as a viable auth method.
///
/// Kept as a constant so test code and the production check stay in sync.
pub const wimoai_API_KEY_ENV_VAR: &str = "wimoai_API_KEY";

/// Legacy env var name.
/// Checked as a fallback when `wimoai_API_KEY` is not set, so existing deployments that use the old name keep working.
pub const LEGACY_wimoai_API_KEY_ENV_VAR: &str = "wimo_CODE_wimoai_API_KEY";

/// Read the API key from the environment.
///
/// Checks `wimoai_API_KEY` first, then falls back to the legacy `wimo_CODE_wimoai_API_KEY` for backward compatibility.
pub(crate) fn read_wimoai_api_key_env() -> Result<String, std::env::VarError> {
    std::env::var(wimoai_API_KEY_ENV_VAR).or_else(|_| std::env::var(LEGACY_wimoai_API_KEY_ENV_VAR))
}

/// Returns `true` if either `wimoai_API_KEY` or `wimo_CODE_wimoai_API_KEY` is set.
pub fn has_wimoai_api_key_env() -> bool {
    read_wimoai_api_key_env().is_ok()
}

/// Whether `wimoai.api_key` should be advertised (and pushed FIRST) when building the `auth_methods` list at `initialize()` time.
///
/// Regression: `wimoai.api_key` must stay first when only per-model credentials exist (no global `wimoai_API_KEY`).
/// Deferring it made BYOK users hit the login screen because the pager uses `auth_methods.first()` for startup metadata.
///
/// [`build_auth_methods`] consumes this predicate and pins the ordering; its tests catch call-site and predicate regressions.
///
/// Probes `std::env` at call time and consults each `ModelEntry` for a resolvable api_key/env_key.
/// Both inputs can change between calls, so the result is not cached.
///
/// `disable_api_key_auth` (`[wimo_com_config] disable_api_key_auth` / `wimo_DISABLE_API_KEY_AUTH`) is the admin kill switch.
/// When true the method is never advertised, regardless of available credentials, so `wimoai_API_KEY` can't bypass a deployment's forced IdP login.
///
/// Presence-only for the first-party env key (treats it as usable).
/// Login paths that have run the validity probe should call [`should_advertise_wimoai_api_key_with_env_ok`] with the probe result instead.
pub(crate) fn should_advertise_wimoai_api_key<'a, I>(disable_api_key_auth: bool, models: I) -> bool
where
    I: IntoIterator<Item = &'a ModelEntry>,
{
    should_advertise_wimoai_api_key_with_env_ok(disable_api_key_auth, models, true)
}

/// Single advertise policy for `wimoai.api_key`: the kill switch, BYOK, and the first-party env key.
/// The env key is gated by `first_party_env_ok` (probe result, or `true` for presence-only); BYOK still advertises without a probe.
pub(crate) fn should_advertise_wimoai_api_key_with_env_ok<'a, I>(
    disable_api_key_auth: bool,
    models: I,
    first_party_env_ok: bool,
) -> bool
where
    I: IntoIterator<Item = &'a ModelEntry>,
{
    if disable_api_key_auth {
        return false;
    }
    let has_byok = models.into_iter().any(ModelEntry::has_own_credentials);
    has_byok || (has_wimoai_api_key_env() && first_party_env_ok)
}

/// Inputs to [`build_auth_methods`].
///
/// The caller (`MvpAgent::initialize()`) computes the booleans.
/// The list-construction logic itself is pure so it can be unit-tested without any of that machinery.
///
/// Interactive session login (`wimo.com`, `oidc`, `cached_token`) was removed:
/// authentication is provider API keys only (per-model `[model.*]`
/// `api_key`/`env_key` from `.wimo/settings.json` or `config.toml`, or the
/// `wimoai_API_KEY` fallback). There is deliberately no other variant to
/// advertise, so an empty method list means "no usable key" (fail auth).
pub struct AuthMethodsBuildInputs {
    /// True if API-key credentials are available (per-model keys or the
    /// `wimoai_API_KEY` env fallback). Computed via
    /// [`should_advertise_wimoai_api_key_with_env_ok`].
    /// When `preferred_method` is `Oidc`, this is ignored (fail-closed).
    pub has_external_api_key: bool,
    /// Config pin (`[auth] preferred_method`).
    /// `None` keeps the single key method; `Oidc` fails closed (only an empty
    /// list and `None`, since interactive login no longer exists).
    pub preferred_method: Option<PreferredAuthMethod>,
}

/// Output of [`build_auth_methods`].
pub struct BuiltAuthMethods {
    /// Auth methods in advertised order: either exactly [`wimoai_API_KEY_METHOD_ID`]
    /// or empty. ORDER IS THE CONTRACT: the pager's `startup_auth_metadata()`
    /// reads `methods.first()` to decide whether interactive login is needed —
    /// with only the key method ever advertised, the login screen never shows.
    pub methods: Vec<acp::AuthMethod>,
    /// The default `auth_method_id` to install on the agent:
    /// `wimoai.api_key` when advertised, else `None` (fail auth).
    pub default_auth_method_id: Option<acp::AuthMethodId>,
}

/// Build the `auth_methods` list and default `auth_method_id` from pre-computed inputs.
///
/// Only one method family exists: provider API keys (`wimoai.api_key`, covering
/// per-model `[model.*]` credentials and the `wimoai_API_KEY` fallback).
/// Session login (`cached_token`, `wimo.com`, `oidc`) was removed, so:
///
/// - unpinned with credentials: `[wimoai.api_key]`, default `wimoai.api_key`
/// - unpinned without credentials: `[]`, default `None` (fail with a
///   key-configuration error, never a login screen)
/// - pinned `api_key`: same as unpinned
/// - pinned `oidc`: `[]`, default `None` (fail-closed; interactive login no
///   longer exists, so the pin can never be satisfied)
pub fn build_auth_methods(inputs: AuthMethodsBuildInputs) -> BuiltAuthMethods {
    let AuthMethodsBuildInputs {
        has_external_api_key,
        preferred_method,
    } = inputs;

    if preferred_method == Some(PreferredAuthMethod::Oidc) {
        wimoai_wimo_telemetry::unified_log::warn(
            "auth: preferred_method=oidc is no longer supported (interactive login was removed)",
            None,
            None,
        );
        return BuiltAuthMethods {
            methods: Vec::new(),
            default_auth_method_id: None,
        };
    }
    if !has_external_api_key {
        return BuiltAuthMethods {
            methods: Vec::new(),
            default_auth_method_id: None,
        };
    }
    BuiltAuthMethods {
        methods: vec![wimoai_api_key_auth_method()],
        default_auth_method_id: Some(acp::AuthMethodId::new(wimoai_API_KEY_METHOD_ID)),
    }
}

/// ACP session auth method. Use `is_session_based_method` for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethodKind {
    wimoaiApiKey,
    CachedToken,
    wimoCom,
    Oidc,
    Unknown,
}

impl AuthMethodKind {
    pub fn from_id(id: &acp::AuthMethodId) -> Self {
        match id.0.as_ref() {
            wimoai_API_KEY_METHOD_ID => Self::wimoaiApiKey,
            CACHED_TOKEN_AUTH_METHOD_ID => Self::CachedToken,
            wimo_COM_METHOD_ID => Self::wimoCom,
            OIDC_METHOD_ID => Self::Oidc,
            _ => Self::Unknown,
        }
    }

    /// API key auth: no auth.json, no refresh, no user interaction.
    pub fn is_api_key(self) -> bool {
        matches!(self, Self::wimoaiApiKey)
    }

    /// `true` for session-based methods (cached_token, wimo.com, oidc).
    pub(crate) fn is_session_based(self) -> bool {
        matches!(self, Self::CachedToken | Self::wimoCom | Self::Oidc)
    }

    /// Requires user interaction (browser, OIDC redirect, or external auth command).
    pub fn needs_interactive_login(self) -> bool {
        matches!(self, Self::wimoCom | Self::Oidc)
    }
}

/// `true` for session-based ACP methods (cached_token, wimo.com, oidc).
pub(crate) fn is_session_based_method(method_id: &acp::AuthMethodId) -> bool {
    AuthMethodKind::from_id(method_id).is_session_based()
}

/// Per-model BYOK status: whether the selected model carries its own `[model.*]` `api_key`/`env_key`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModelByok {
    /// Model has its own per-model key (not refreshable).
    Byok,
    /// Model has no per-model key (session auth governs).
    NotByok,
    /// Config couldn't be loaded/parsed; BYOK status indeterminate.
    Unknown,
}

impl ModelByok {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Byok => "byok",
            Self::NotByok => "not_byok",
            Self::Unknown => "unknown",
        }
    }
}

/// Whether this session and model combination uses a refreshable session token.
///
/// Gates on stable inputs, not `Credentials.auth_type`.
/// That field collapses to `ApiKey` when the session-token cache is momentarily empty and `wimoai_API_KEY` is set.
/// The collapse demoted live OIDC sessions to non-refreshable api-key mode and 401'd every prompt until restart.
/// `model_byok` still excludes genuine per-model BYOK, whose keys are not refreshable.
///
/// `Unknown` means BYOK status is indeterminate: config currently unparseable, no sampling config yet, or the per-model memo was cleared.
/// It must **not** demote a live session to non-refreshable api-key mode.
/// That demotion re-sends the stale buffered token on every turn and 401s with `bad-credentials` until restart.
/// Instead, `Unknown` refreshes only when `endpoint_is_first_party`.
/// On a first-party host (cli-chat-proxy / first-party API) the session token cannot leak to a third-party BYOK endpoint.
/// A definite `NotByok` always refreshes (it only ever routes to the session endpoint); a definite `Byok` never does.
pub(crate) fn session_token_auth_gate(
    is_session_based_method: bool,
    model_byok: ModelByok,
    endpoint_is_first_party: bool,
) -> bool {
    is_session_based_method
        && match model_byok {
            ModelByok::NotByok => true,
            ModelByok::Byok => false,
            ModelByok::Unknown => endpoint_is_first_party,
        }
}

pub const AUTH_ERROR_SESSION_EXPIRED: &str =
    "Session expired. Re-check your provider API key in .wimo/settings.json ([model.*] api_key/env_key) or its environment variable.";

pub const AUTH_ERROR_API_KEY: &str = "Authentication failed. Add your provider API key to .wimo/settings.json ([model.*] api_key/env_key) or set the corresponding environment variable.";

/// Next ACP method id when no session credential can proceed, or `None` when
/// no key credentials exist. Interactive login was removed, so there is no
/// interactive fallthrough: the only target is non-interactive `wimoai.api_key`.
///
/// Pinned `oidc` always fails closed (`None`); every other pin falls through
/// to the key method when advertiseable.
pub(crate) fn api_key_fallthrough_method_id(
    has_external_api_key: bool,
    preferred_method: Option<PreferredAuthMethod>,
) -> Option<&'static str> {
    match preferred_method {
        Some(PreferredAuthMethod::Oidc) => None,
        _ if has_external_api_key => Some(wimoai_API_KEY_METHOD_ID),
        _ => None,
    }
}

/// Error when `preferred_method=api_key` but no key/BYOK credentials exist.
pub const PREFERRED_API_KEY_UNAVAILABLE: &str = "preferred_method=api_key but no API key is configured (set a provider env var or model api_key/env_key in .wimo/settings.json).";

/// Error when `preferred_method=oidc` but the session path cannot proceed.
/// Interactive login was removed, so this pin can never be satisfied.
pub const PREFERRED_OIDC_UNAVAILABLE: &str =
    "preferred_method=oidc is no longer supported (interactive login was removed). Use preferred_method=api_key with provider keys in .wimo/settings.json.";

pub const wimoai_API_KEY_METHOD_ID: &str = "wimoai.api_key";
pub(crate) fn wimoai_api_key_auth_method() -> acp::AuthMethod {
    acp::AuthMethod::Agent(
        acp::AuthMethodAgent::new(
            acp::AuthMethodId::new(wimoai_API_KEY_METHOD_ID),
            "wimoai.api_key".to_string(),
        )
        .description(Some(format!(
            "{wimoai_API_KEY_ENV_VAR} or api_key/env_key in .wimo/settings.json"
        ))),
    )
}

/// Session-login method ids (`cached_token`, `wimo.com`, `oidc`) are never
/// advertised anymore (interactive login was removed), but the ids stay
/// reserved so persisted selections and wire traffic keep classifying
/// correctly via [`AuthMethodKind`].
pub const CACHED_TOKEN_AUTH_METHOD_ID: &str = "cached_token";

pub const wimo_COM_METHOD_ID: &str = "wimo.com";

pub const OIDC_METHOD_ID: &str = "oidc";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::config::{Config, resolve_model_list};
    use agent_client_protocol as acp;
    use serial_test::serial;

    /// When API-key credentials are advertiseable, the fallthrough picks non-interactive `wimoai.api_key`.
    /// Interactive login was removed, so there is no other target.
    #[test]
    fn api_key_fallthrough_prefers_api_key_when_advertiseable() {
        assert_eq!(
            api_key_fallthrough_method_id(true, None),
            Some(wimoai_API_KEY_METHOD_ID),
        );
        assert_eq!(
            api_key_fallthrough_method_id(true, Some(PreferredAuthMethod::ApiKey)),
            Some(wimoai_API_KEY_METHOD_ID),
        );
    }

    /// With no advertiseable API-key credentials, the fallthrough is `None` (fail-closed).
    /// There is no interactive login to fall back to anymore.
    #[test]
    fn api_key_fallthrough_fails_closed_without_api_key() {
        assert_eq!(api_key_fallthrough_method_id(false, None), None,);
        assert_eq!(
            api_key_fallthrough_method_id(false, Some(PreferredAuthMethod::ApiKey)),
            None,
        );
    }

    /// Pinned `oidc` never falls through (interactive login no longer exists).
    #[test]
    fn api_key_fallthrough_fails_closed_when_oidc_pinned() {
        assert_eq!(
            api_key_fallthrough_method_id(true, Some(PreferredAuthMethod::Oidc)),
            None,
        );
        assert_eq!(
            api_key_fallthrough_method_id(false, Some(PreferredAuthMethod::Oidc)),
            None,
        );
    }

    /// Classifier matrix for all auth method variants.
    #[test]
    fn auth_method_kind_classifier_matrix() {
        let session_methods = [
            CACHED_TOKEN_AUTH_METHOD_ID,
            wimo_COM_METHOD_ID,
            OIDC_METHOD_ID,
        ];
        for method_id in session_methods {
            let id = acp::AuthMethodId::new(method_id);
            let kind = AuthMethodKind::from_id(&id);
            assert!(
                kind.is_session_based(),
                "{method_id}: kind must be session-based"
            );
            assert!(
                is_session_based_method(&id),
                "{method_id}: wrapper must agree"
            );
        }
        let api_id = acp::AuthMethodId::new(wimoai_API_KEY_METHOD_ID);
        let api_kind = AuthMethodKind::from_id(&api_id);
        assert!(!api_kind.is_session_based());
        assert!(api_kind.is_api_key());
        assert!(!is_session_based_method(&api_id));
        assert!(!is_session_based_method(&acp::AuthMethodId::new(
            "unknown-method"
        )));
    }

    use wimoai_wimo_test_support::EnvGuard;

    // ── Helpers ─────────────────────────────────────────────────────────

    /// Default inputs to `build_auth_methods`: no credentials anywhere.
    /// Tests override only the fields they care about.
    fn default_inputs() -> AuthMethodsBuildInputs {
        AuthMethodsBuildInputs {
            has_external_api_key: false,
            preferred_method: None,
        }
    }

    fn method_ids(built: &BuiltAuthMethods) -> Vec<&str> {
        built.methods.iter().map(|m| m.id().0.as_ref()).collect()
    }

    fn default_id(built: &BuiltAuthMethods) -> Option<&str> {
        built
            .default_auth_method_id
            .as_ref()
            .map(|id| id.0.as_ref())
    }

    fn first_kind(methods: &[acp::AuthMethod]) -> Option<AuthMethodKind> {
        methods.first().map(|m| AuthMethodKind::from_id(m.id()))
    }

    // build_auth_methods: only `wimoai.api_key` is ever advertised.
    // Advertising anything else (or an empty list when a key exists) must fail the tests below.

    /// BYOK with only per-model `env_key` must list `wimoai.api_key` first.
    #[test]
    fn enterprise_byok_first_method_is_wimoai_api_key() {
        let inputs = AuthMethodsBuildInputs {
            has_external_api_key: true, // user with resolved per-model env_key
            ..default_inputs()
        };
        let built = build_auth_methods(inputs);

        assert_eq!(
            first_kind(&built.methods),
            Some(AuthMethodKind::wimoaiApiKey),
            "BYOK enterprise-style: auth_methods.first() MUST be wimoai.api_key \
             (deferred-to-last ordering sends users to the login screen)",
        );
        assert_eq!(
            built
                .default_auth_method_id
                .as_ref()
                .map(|id| id.0.as_ref()),
            Some(wimoai_API_KEY_METHOD_ID),
        );
        // Cross-check with the pager-side predicate: the first method must not require interactive login
        // That is the exact condition the pager's `startup_auth_metadata()` uses
        assert!(
            !AuthMethodKind::from_id(built.methods[0].id()).needs_interactive_login(),
            "first method MUST NOT need interactive login when wimoai.api_key is available",
        );
    }

    /// Credentials present: exactly one method (`wimoai.api_key`) is advertised and defaulted.
    /// No session-login method (`cached_token`, `wimo.com`, `oidc`) may ever appear.
    #[test]
    fn api_key_credentials_advertise_only_wimoai_api_key() {
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key: true,
            ..default_inputs()
        });

        assert_eq!(method_ids(&built), vec![wimoai_API_KEY_METHOD_ID]);
        assert_eq!(default_id(&built), Some(wimoai_API_KEY_METHOD_ID));
        assert!(
            !AuthMethodKind::from_id(built.methods[0].id()).needs_interactive_login(),
            "the only advertised method MUST NOT need interactive login",
        );
    }

    /// No credentials anywhere: empty method list, no default (fail auth, never a login screen).
    #[test]
    fn no_credentials_advertises_nothing() {
        let built = build_auth_methods(default_inputs());

        assert!(built.methods.is_empty());
        assert!(built.default_auth_method_id.is_none());
        assert_eq!(first_kind(&built.methods), None);
    }

    // ── End-to-end: enterprise TOML to resolved models to build_auth_methods ─

    /// END-TO-END REGRESSION TEST: parses the literal enterprise-style
    /// `~/.wimo/config.toml` skeleton from the bug report, walks it through
    /// the same predicate (`should_advertise_wimoai_api_key`) and the same
    /// list-builder (`build_auth_methods`) that `MvpAgent::initialize()` uses
    /// in production, and asserts that `auth_methods.first()` is `wimoai.api_key`
    /// (which causes the pager to skip the login screen).
    ///
    /// This is the test that *would have caught* that regression.
    /// If the bug returns (wimoai.api_key pushed LAST when only per-model credentials exist), `first_kind` stops being `wimoaiApiKey` and this test fails.
    #[test]
    #[serial]
    fn enterprise_byok_config_does_not_require_login() {
        const TEST_ENV_VAR: &str = "TEST_ENTERPRISE_REGRESSION_AUTH_TOKEN";

        // Make sure no global key is masking the per-model path we're trying to exercise
        // Held until end-of-scope so we restore on panic too
        let _global = EnvGuard::unset(wimoai_API_KEY_ENV_VAR);

        let dm = crate::models::default_model();
        let toml: toml::Value = toml::from_str(&format!(
            r#"
            [model."{dm}"]
            model = "{dm}"
            base_url = "https://inference.example.com/v1"
            context_window = 200000
            env_key = "{TEST_ENV_VAR}"
            "#,
        ))
        .unwrap();
        let cfg = Config::new_from_toml_cfg(&toml).expect("config should parse");
        let models = resolve_model_list(&cfg, None);
        let model = models.get(dm).expect("enterprise-style model should exist");
        assert_eq!(
            model.env_key.as_ref().map(|k| k.names()),
            Some(vec![TEST_ENV_VAR])
        );

        // Without the env var present, has_own_credentials() and the predicate return false, and the builder advertises nothing.
        // Confirms the predicate isn't trivially true
        {
            let _unset = EnvGuard::unset(TEST_ENV_VAR);
            let has_external_api_key = should_advertise_wimoai_api_key(false, models.values());
            assert!(!has_external_api_key);
            let built = build_auth_methods(AuthMethodsBuildInputs {
                has_external_api_key,
                ..default_inputs()
            });
            assert!(
                built.methods.is_empty(),
                "without env_key resolved, nothing must be advertised",
            );
        }

        // With the env var present (the actual enterprise scenario), the predicate returns true
        // The builder MUST put `wimoai.api_key` first so the pager's `startup_auth_metadata()` returns `needs_login = false`
        {
            let _set = EnvGuard::set(TEST_ENV_VAR, "enterprise-secret-token");
            let has_external_api_key = should_advertise_wimoai_api_key(false, models.values());
            assert!(has_external_api_key);
            let built = build_auth_methods(AuthMethodsBuildInputs {
                has_external_api_key,
                ..default_inputs()
            });
            assert_eq!(
                first_kind(&built.methods),
                Some(AuthMethodKind::wimoaiApiKey),
                "BYOK: wimoai.api_key must be auth_methods.first(); deferred-to-last \
                 ordering sends enterprise users to the login screen",
            );
            assert!(
                !AuthMethodKind::from_id(built.methods[0].id()).needs_interactive_login(),
                "auth_methods.first() MUST NOT need interactive login -- this \
                 is the exact predicate the pager's startup_auth_metadata() \
                 uses to decide whether to show the login screen",
            );
        }
    }

    /// `wimoai_API_KEY` alone (no per-model creds) also triggers advertising `wimoai.api_key` as the first method.
    /// Historical "external key" path; covered here so the predicate keeps treating env-var-only users the same as per-model users.
    #[test]
    #[serial]
    fn global_external_api_key_advertises_wimoai_api_key_first() {
        let _set = EnvGuard::set(wimoai_API_KEY_ENV_VAR, "wimoai-external-key");
        let cfg = Config::default();
        let models = resolve_model_list(&cfg, None);
        let has_external_api_key = should_advertise_wimoai_api_key(false, models.values());
        assert!(has_external_api_key);
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key,
            ..default_inputs()
        });
        assert_eq!(first_kind(&built.methods), Some(AuthMethodKind::wimoaiApiKey));
    }

    /// Admin kill switch (`disable_api_key_auth`): the predicate must return false even when credentials are available everywhere.
    /// That includes both the global env var and a per-model env_key.
    /// The builder then advertises nothing (fail-closed); there is no login method to fall back to.
    #[test]
    #[serial]
    fn disable_api_key_auth_suppresses_wimoai_api_key_method() {
        let _set = EnvGuard::set(wimoai_API_KEY_ENV_VAR, "wimoai-external-key");
        let cfg = Config::default();
        let models = resolve_model_list(&cfg, None);

        // Flag off: today's behavior (advertised first).
        assert!(should_advertise_wimoai_api_key(false, models.values()));

        // Flag on: never advertised, regardless of credentials.
        let has_external_api_key = should_advertise_wimoai_api_key(true, models.values());
        assert!(!has_external_api_key);
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key,
            ..default_inputs()
        });
        assert!(
            !built
                .methods
                .iter()
                .any(|m| AuthMethodKind::from_id(m.id()) == AuthMethodKind::wimoaiApiKey),
            "wimoai.api_key must not be advertised when disable_api_key_auth is set",
        );
        assert_eq!(
            first_kind(&built.methods),
            None,
            "with api-key auth disabled, nothing is advertised (no login fallback exists)",
        );
        assert!(built.default_auth_method_id.is_none());
    }

    #[test]
    #[serial]
    fn env_key_probe_unusable_suppresses_advertise_without_byok() {
        let _set = EnvGuard::set(wimoai_API_KEY_ENV_VAR, "wimoai-dead-key");
        let _legacy = EnvGuard::unset(LEGACY_wimoai_API_KEY_ENV_VAR);
        let cfg = Config::default();
        let models = resolve_model_list(&cfg, None);
        assert!(
            should_advertise_wimoai_api_key(false, models.values()),
            "presence-only helper still sees the env key"
        );
        assert!(
            !should_advertise_wimoai_api_key_with_env_ok(false, models.values(), false),
            "probe-unusable env key alone must not advertise"
        );
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key: false,
            ..default_inputs()
        });
        assert_eq!(first_kind(&built.methods), None);
    }

    #[test]
    #[serial]
    fn env_key_probe_ok_still_advertises() {
        let _set = EnvGuard::set(wimoai_API_KEY_ENV_VAR, "wimoai-live-key");
        let cfg = Config::default();
        let models = resolve_model_list(&cfg, None);
        assert!(should_advertise_wimoai_api_key_with_env_ok(
            false,
            models.values(),
            true
        ));
    }

    #[test]
    #[serial]
    fn byok_advertises_even_when_env_probe_unusable() {
        const TEST_ENV_VAR: &str = "TEST_BYOK_PROBE_INDEPENDENT_TOKEN";
        let _unset = EnvGuard::unset(wimoai_API_KEY_ENV_VAR);
        let _legacy = EnvGuard::unset(LEGACY_wimoai_API_KEY_ENV_VAR);
        let _byok = EnvGuard::set(TEST_ENV_VAR, "enterprise-secret-token");

        let dm = crate::models::default_model();
        let toml: toml::Value = toml::from_str(&format!(
            r#"
            [model."{dm}"]
            model = "{dm}"
            base_url = "https://inference.example.com/v1"
            context_window = 200000
            env_key = "{TEST_ENV_VAR}"
            "#,
        ))
        .unwrap();
        let cfg = Config::new_from_toml_cfg(&toml).expect("config should parse");
        let models = resolve_model_list(&cfg, None);
        assert!(
            should_advertise_wimoai_api_key_with_env_ok(false, models.values(), false),
            "BYOK must not depend on the first-party env probe"
        );
    }

    /// Legacy `wimo_CODE_wimoai_API_KEY` env var is accepted as a fallback when `wimoai_API_KEY` is not set, so existing deployments keep working.
    #[test]
    #[serial]
    fn legacy_env_var_fallback_advertises_wimoai_api_key() {
        let _unset_new = EnvGuard::unset(wimoai_API_KEY_ENV_VAR);
        let _set_legacy = EnvGuard::set(LEGACY_wimoai_API_KEY_ENV_VAR, "wimoai-legacy-key");
        assert!(has_wimoai_api_key_env());
        assert_eq!(read_wimoai_api_key_env().unwrap(), "wimoai-legacy-key");

        let cfg = Config::default();
        let models = resolve_model_list(&cfg, None);
        let has_external_api_key = should_advertise_wimoai_api_key(false, models.values());
        assert!(has_external_api_key);
    }

    /// When both `wimoai_API_KEY` and `wimo_CODE_wimoai_API_KEY` are set, the new name takes precedence.
    #[test]
    #[serial]
    fn new_env_var_takes_precedence_over_legacy() {
        let _new = EnvGuard::set(wimoai_API_KEY_ENV_VAR, "new-key");
        let _legacy = EnvGuard::set(LEGACY_wimoai_API_KEY_ENV_VAR, "old-key");
        assert_eq!(read_wimoai_api_key_env().unwrap(), "new-key");
    }

    /// Negative case: with no credentials anywhere, `build_auth_methods` advertises nothing.
    /// This pins the "no" answer so the BYOK tests above aren't trivially passing.
    #[test]
    #[serial]
    fn no_credentials_means_nothing_advertised() {
        let _g1 = EnvGuard::unset("wimo_AUTH");
        let _g2 = EnvGuard::unset("wimo_AUTH_PATH");
        let _g3 = EnvGuard::unset(wimoai_API_KEY_ENV_VAR);
        let _g4 = EnvGuard::unset(LEGACY_wimoai_API_KEY_ENV_VAR);

        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key: false,
            ..default_inputs()
        });
        assert!(built.methods.is_empty());
        assert!(built.default_auth_method_id.is_none());
    }

    // ── preferred_method pin (fail-closed) ──────────────────────────────

    #[test]
    fn pin_api_key_with_key_only_advertises_api_key() {
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key: true,
            preferred_method: Some(PreferredAuthMethod::ApiKey),
            ..default_inputs()
        });
        assert_eq!(method_ids(&built), vec![wimoai_API_KEY_METHOD_ID]);
        assert_eq!(default_id(&built), Some(wimoai_API_KEY_METHOD_ID));
    }

    #[test]
    fn pin_api_key_without_key_fails_closed() {
        let built = build_auth_methods(AuthMethodsBuildInputs {
            has_external_api_key: false,
            preferred_method: Some(PreferredAuthMethod::ApiKey),
            ..default_inputs()
        });
        assert!(built.methods.is_empty());
        assert!(built.default_auth_method_id.is_none());
    }

    #[test]
    fn pin_oidc_always_fails_closed() {
        for has_external_api_key in [false, true] {
            let built = build_auth_methods(AuthMethodsBuildInputs {
                has_external_api_key,
                preferred_method: Some(PreferredAuthMethod::Oidc),
                ..default_inputs()
            });
            assert!(
                built.methods.is_empty(),
                "oidc pin advertises nothing (has_external_api_key={has_external_api_key})"
            );
            assert!(built.default_auth_method_id.is_none());
        }
    }
}
