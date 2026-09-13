use agent_client_protocol as acp;

use crate::auth::{AuthManager, BeecodeAuth};

/// Require BeeCode auth from a sync context: with no `.await` to refresh, a token inside the client's early-invalidation buffer still counts.
pub(crate) fn require_beecode_auth(
    auth_manager: &AuthManager,
    missing_message: &'static str,
    non_beecode_message: &'static str,
) -> Result<BeecodeAuth, acp::Error> {
    let auth = auth_manager
        .current_or_expired()
        .ok_or_else(|| acp::Error::auth_required().data(missing_message))?;
    if !auth.is_beecode_auth() {
        return Err(acp::Error::auth_required().data(non_beecode_message));
    }
    Ok(auth)
}
