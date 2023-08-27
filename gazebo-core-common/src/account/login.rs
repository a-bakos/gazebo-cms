use crate::account::auth::AuthResponseAccountInfo;
use crate::account::gb_account::AccountID;
use crate::account::role::AccountRole;
use crate::status_code::HttpStatusCode;
use serde::{Deserialize, Serialize};

/// Login status variants
pub enum LoginStatus {
    Authorized,
    Unauthorized,
    ServerError,
}
