use agent_client_protocol as acp;

use crate::auth::{AuthManager, wimoAuth};

/// Require wimo AI auth from a sync context: with no `.await` to refresh, a token inside the client's early-invalidation buffer still counts.
pub(crate) fn require_wimoai_auth(
    auth_manager: &AuthManager,
    missing_message: &'static str,
    non_wimoai_message: &'static str,
) -> Result<wimoAuth, acp::Error> {
    let auth = auth_manager
        .current_or_expired()
        .ok_or_else(|| acp::Error::auth_required().data(missing_message))?;
    if !auth.is_wimoai_auth() {
        return Err(acp::Error::auth_required().data(non_wimoai_message));
    }
    Ok(auth)
}
