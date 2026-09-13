//! The beecode Build backend: BeeCode OAuth2, enterprise OIDC, the operator's auth binary, or a devbox.

use std::sync::Arc;

use super::{AuthBackend, LoginRequest};
use crate::auth::refresh::{
    AuthSnapshot, DiagnosticUploader, ExternalBinaryRefresher, ExternalCommandRunner,
    OidcRefresher, TokenRefresher,
};
use crate::auth::{AuthManager, BeecodeAuth, BeecodeComConfig};

#[derive(Default)]
pub(crate) struct BeecodeAuthBackend;

#[async_trait::async_trait(?Send)]
impl AuthBackend for BeecodeAuthBackend {
    fn scope_key(&self, config: &BeecodeComConfig) -> String {
        config.auth_scope()
    }

    /// Devbox auth files from before the OIDC flow wrote this key, and only this backend ever minted credentials into it.
    fn inherited_scopes(&self) -> &'static [&'static str] {
        &[crate::auth::model::LEGACY_SCOPE]
    }

    /// An BeeCode login can come from OAuth2, a customer's own login provider, the auth binary, or a devbox, so there is no one issuer to check for.
    /// Saying yes to all of them is safe: a credential minted elsewhere still gets sent to BeeCode, which rejects it.
    fn owns(&self, _auth: &BeecodeAuth) -> bool {
        true
    }

    /// Some customers run their own gateway and sign in there with the session BeeCode issued them, so a list of allowed hosts would lock them out.
    /// The models cache stops one backend's models from being used by another: it remembers the URL each entry came from and ignores the rest.
    fn may_receive_session(&self, _url: &str) -> bool {
        true
    }

    fn login_host(&self, config: &BeecodeComConfig) -> String {
        super::host_of(&config.beecode_ws_origin)
    }

    fn is_beecode_authority(&self) -> bool {
        true
    }

    async fn login(&self, req: LoginRequest<'_>) -> anyhow::Result<(BeecodeAuth, bool)> {
        crate::auth::flow::run_auth_flow_steps(
            req.auth_manager,
            req.beecode_com_config,
            req.reauth,
            req.force_interactive,
            req.on_stderr,
            req.url_tx,
            req.code_rx,
            req.login_override,
        )
        .await
    }

    fn refresher(
        &self,
        manager: Arc<AuthManager>,
        auth_provider_command: Option<String>,
        diagnostic_uploader: Option<DiagnosticUploader>,
    ) -> Arc<dyn TokenRefresher> {
        match auth_provider_command {
            Some(cmd) => {
                let runner: Arc<dyn ExternalCommandRunner> = manager;
                Arc::new(ExternalBinaryRefresher::new(runner, cmd))
            }
            None => {
                let snapshot: Arc<dyn AuthSnapshot> = manager;
                let refresher = OidcRefresher::new(snapshot);
                match diagnostic_uploader {
                    Some(uploader) => Arc::new(refresher.with_diagnostic_upload(uploader)),
                    None => Arc::new(refresher),
                }
            }
        }
    }
}
