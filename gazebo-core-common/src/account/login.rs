use crate::account::gb_account::AccountID;
use crate::account::role::AccountRole;
use crate::status_code::HttpStatusCode;
use serde::{Deserialize, Serialize};

/// Account details to send back on login request
/// Default() is used with error cases
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginResponseAccountDetails {
    pub id: AccountID,
    pub login_name: String,
    pub email: String,
    pub role: AccountRole,
    pub token: String,
}

/// Login status variants
pub enum LoginStatus {
    Authorized,
    Unauthorized,
    ServerError,
}

/// This is the final structure that is returned on a login request
#[derive(Deserialize, Serialize)]
pub struct LoginResponseWithStatusCode {
    pub http_status_code: u32,
    pub account_details: LoginResponseAccountDetails,
}

impl LoginResponseWithStatusCode {
    pub fn response(
        login_status: LoginStatus,
        account_details: Option<LoginResponseAccountDetails>,
    ) -> Self {
        let (http_status_code, account_details) = match login_status {
            LoginStatus::Authorized => (HttpStatusCode::Ok.code(), account_details.unwrap()),
            LoginStatus::Unauthorized => (
                HttpStatusCode::Unauthorized.code(),
                LoginResponseAccountDetails::default(),
            ),
            LoginStatus::ServerError => (
                HttpStatusCode::InternalServerError.code(),
                LoginResponseAccountDetails::default(),
            ),
        };
        Self {
            http_status_code,
            account_details,
        }
    }
}
