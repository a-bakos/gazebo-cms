use crate::consts;

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

// Capabilities draft:
// POST
// CreatePost
// DeletePost
// UpdatePost
// ReadPost

// USER
// AddUser
// DeleteUser
// UpdateUser

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountRole {
    Admin,       // read, write, delete, add ??
    Editor,      // read, write, delete
    Contributor, // read
    NotFound,    // missing or incorrect role
}

impl std::fmt::Display for AccountRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            AccountRole::Admin => write!(f, "{}", consts::USER_ROLE_ADMIN),
            AccountRole::Editor => write!(f, "{}", consts::USER_ROLE_EDITOR),
            AccountRole::Contributor => write!(f, "{}", consts::USER_ROLE_CONTRIBUTOR),
            AccountRole::NotFound => write!(f, "{}", consts::USER_ROLE_NOT_FOUND),
        }
    }
}

impl From<&str> for AccountRole {
    fn from(value: &str) -> Self {
        match value {
            consts::USER_ROLE_ADMIN => AccountRole::Admin,
            consts::USER_ROLE_EDITOR => AccountRole::Editor,
            consts::USER_ROLE_CONTRIBUTOR => AccountRole::Contributor,
            _ => AccountRole::NotFound,
        }
    }
}

impl From<AccountRole> for String {
    fn from(val: AccountRole) -> Self {
        match val {
            AccountRole::Admin => consts::USER_ROLE_ADMIN.to_string(),
            AccountRole::Editor => consts::USER_ROLE_EDITOR.to_string(),
            AccountRole::Contributor => consts::USER_ROLE_CONTRIBUTOR.to_string(),
            AccountRole::NotFound => consts::USER_ROLE_NOT_FOUND.to_string(),
        }
    }
}

pub fn get_role_variant(role: &str) -> AccountRole {
    match role {
        consts::USER_ROLE_ADMIN => AccountRole::Admin,
        consts::USER_ROLE_EDITOR => AccountRole::Editor,
        consts::USER_ROLE_CONTRIBUTOR => AccountRole::Contributor,
        _ => AccountRole::NotFound,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_role_variant_correct() {
        let role_1 = consts::USER_ROLE_ADMIN;
        let role_2 = consts::USER_ROLE_EDITOR;
        let role_3 = consts::USER_ROLE_CONTRIBUTOR;
        let role_4 = "any_role";

        assert_eq!(AccountRole::Admin, get_role_variant(role_1));
        assert_eq!(AccountRole::Editor, get_role_variant(role_2));
        assert_eq!(AccountRole::Contributor, get_role_variant(role_3));
        assert_eq!(AccountRole::NotFound, get_role_variant(role_4));
    }
}
