use crate::account::role::AccountRole;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountID(pub u32);

impl Display for AccountID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GB_Account {
    pub login_name: String,
    pub email: String,
    pub id: AccountID,
    pub role: AccountRole,
    pub password: String,
    pub registered: String,
    pub last_login: String,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct GB_CurrentAccount {
    pub id: AccountID,
    pub username: String,
    pub email: String,
    pub role: AccountRole,
}
