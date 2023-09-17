use crate::account::gb_account::AccountID;

#[allow(dead_code)]
pub const SYSTEM_USER_ID: AccountID = AccountID(0);

pub const NEW_ACCOUNT_TEMP_ID: AccountID = AccountID(0);

pub const ACCOUNT_ROLE_ADMIN: &str = "administrator";
pub const ACCOUNT_ROLE_EDITOR: &str = "editor";
pub const ACCOUNT_ROLE_CONTRIBUTOR: &str = "contributor";
pub const ACCOUNT_ROLE_NOT_FOUND: &str = "not_found";
