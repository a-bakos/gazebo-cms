use crate::consts::{
    USER_ROLE_ADMIN, USER_ROLE_CONTRIBUTOR, USER_ROLE_EDITOR, USER_ROLE_NOT_FOUND,
};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountRole {
    Admin,       // read, write, delete, add ??
    Editor,      // read, write, delete
    Contributor, // read
    NotFound,    // missing or incorrect role
}

pub fn get_role_variant(role: &str) -> AccountRole {
    match role {
        USER_ROLE_ADMIN => AccountRole::Admin,
        USER_ROLE_EDITOR => AccountRole::Editor,
        USER_ROLE_CONTRIBUTOR => AccountRole::Contributor,
        _ => AccountRole::NotFound,
    }
}

impl std::fmt::Display for AccountRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            AccountRole::Admin => write!(f, "{}", USER_ROLE_ADMIN),
            AccountRole::Editor => write!(f, "{}", USER_ROLE_EDITOR),
            AccountRole::Contributor => write!(f, "{}", USER_ROLE_CONTRIBUTOR),
            AccountRole::NotFound => write!(f, "{}", USER_ROLE_NOT_FOUND),
        }
    }
}

impl From<&str> for AccountRole {
    fn from(value: &str) -> Self {
        match value {
            USER_ROLE_ADMIN => AccountRole::Admin,
            USER_ROLE_EDITOR => AccountRole::Editor,
            USER_ROLE_CONTRIBUTOR => AccountRole::Contributor,
            _ => AccountRole::NotFound,
        }
    }
}

impl From<AccountRole> for String {
    fn from(val: AccountRole) -> Self {
        match val {
            AccountRole::Admin => USER_ROLE_ADMIN.to_string(),
            AccountRole::Editor => USER_ROLE_EDITOR.to_string(),
            AccountRole::Contributor => USER_ROLE_CONTRIBUTOR.to_string(),
            AccountRole::NotFound => USER_ROLE_NOT_FOUND.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_role_variant_correct() {
        let role_1 = USER_ROLE_ADMIN;
        let role_2 = USER_ROLE_EDITOR;
        let role_3 = USER_ROLE_CONTRIBUTOR;
        let role_4 = "any_role";

        assert_eq!(AccountRole::Admin, get_role_variant(role_1));
        assert_eq!(AccountRole::Editor, get_role_variant(role_2));
        assert_eq!(AccountRole::Contributor, get_role_variant(role_3));
        assert_eq!(AccountRole::NotFound, get_role_variant(role_4));
    }
}
