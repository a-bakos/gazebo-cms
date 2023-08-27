use crate::account::consts::{
    ACCOUNT_ROLE_ADMIN, ACCOUNT_ROLE_CONTRIBUTOR, ACCOUNT_ROLE_EDITOR, ACCOUNT_ROLE_NOT_FOUND,
};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountRole {
    // read, write, delete, add ??
    Admin,
    // read, write, delete
    Editor,
    // read
    Contributor,
    NotFound, // missing or incorrect role
}
impl Default for AccountRole {
    fn default() -> Self {
        Self::NotFound
    }
}

pub fn get_role_variant(role: &str) -> AccountRole {
    match role {
        ACCOUNT_ROLE_ADMIN => AccountRole::Admin,
        ACCOUNT_ROLE_EDITOR => AccountRole::Editor,
        ACCOUNT_ROLE_CONTRIBUTOR => AccountRole::Contributor,
        _ => AccountRole::NotFound,
    }
}

impl std::fmt::Display for AccountRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            AccountRole::Admin => write!(f, "{}", ACCOUNT_ROLE_ADMIN),
            AccountRole::Editor => write!(f, "{}", ACCOUNT_ROLE_EDITOR),
            AccountRole::Contributor => write!(f, "{}", ACCOUNT_ROLE_CONTRIBUTOR),
            AccountRole::NotFound => write!(f, "{}", ACCOUNT_ROLE_NOT_FOUND),
        }
    }
}

impl From<&str> for AccountRole {
    fn from(value: &str) -> Self {
        match value {
            ACCOUNT_ROLE_ADMIN => AccountRole::Admin,
            ACCOUNT_ROLE_EDITOR => AccountRole::Editor,
            ACCOUNT_ROLE_CONTRIBUTOR => AccountRole::Contributor,
            _ => AccountRole::NotFound,
        }
    }
}

impl From<AccountRole> for String {
    fn from(val: AccountRole) -> Self {
        match val {
            AccountRole::Admin => ACCOUNT_ROLE_ADMIN.to_string(),
            AccountRole::Editor => ACCOUNT_ROLE_EDITOR.to_string(),
            AccountRole::Contributor => ACCOUNT_ROLE_CONTRIBUTOR.to_string(),
            AccountRole::NotFound => ACCOUNT_ROLE_NOT_FOUND.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_role_variant_correct() {
        let role_1 = ACCOUNT_ROLE_ADMIN;
        let role_2 = ACCOUNT_ROLE_EDITOR;
        let role_3 = ACCOUNT_ROLE_CONTRIBUTOR;
        let role_4 = "any_role";

        assert_eq!(AccountRole::Admin, get_role_variant(role_1));
        assert_eq!(AccountRole::Editor, get_role_variant(role_2));
        assert_eq!(AccountRole::Contributor, get_role_variant(role_3));
        assert_eq!(AccountRole::NotFound, get_role_variant(role_4));
    }
}
