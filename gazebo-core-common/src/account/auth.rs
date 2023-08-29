use crate::{
    account::{gb_account::AccountID, login::LoginStatus, role::AccountRole},
    status_code::HttpStatusCode,
};
use serde::{Deserialize, Serialize};

/// Account details to send back on login request
/// Default() is used with error cases
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AuthResponseAccountInfo {
    pub id: AccountID,
    pub role: AccountRole,
    pub login_name: String,
}

/// This is the final wrapper structure that is returned on a login request
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AuthResponsePayload {
    pub http_status_code: u32,
    pub account_details: AuthResponseAccountInfo,
    pub token: Option<String>,
}

impl AuthResponsePayload {
    pub fn response(
        login_status: LoginStatus,
        account_details: Option<AuthResponseAccountInfo>,
        token: Option<String>,
    ) -> Self {
        let (http_status_code, account_details, token) = match login_status {
            LoginStatus::Authorized => (HttpStatusCode::Ok.code(), account_details.unwrap(), token),
            LoginStatus::Unauthorized => (
                HttpStatusCode::Unauthorized.code(),
                AuthResponseAccountInfo::default(),
                None,
            ),
            LoginStatus::ServerError => (
                HttpStatusCode::InternalServerError.code(),
                AuthResponseAccountInfo::default(),
                None,
            ),
        };
        Self {
            http_status_code,
            account_details,
            token,
        }
    }
}
