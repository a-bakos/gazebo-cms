use crate::{
    account::{auth::AuthResponseAccountInfo, gb_account::AccountID, role::AccountRole},
    status_code::HttpStatusCode,
};
use serde::{Deserialize, Serialize};

/// Login status variants
pub enum LoginStatus {
    Authorized,
    Unauthorized,
    ServerError,
}
