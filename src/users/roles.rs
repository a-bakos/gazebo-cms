use crate::consts;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,       // read, write, delete, add ??
    Editor,      // read, write, delete
    Contributor, // read
    NotFound,    // missing or incorrect role
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserRole::Admin => write!(f, "{}", consts::USER_ROLE_ADMIN),
            UserRole::Editor => write!(f, "{}", consts::USER_ROLE_EDITOR),
            UserRole::Contributor => write!(f, "{}", consts::USER_ROLE_CONTRIBUTOR),
            UserRole::NotFound => write!(f, "{}", consts::USER_ROLE_NOT_FOUND),
        }
    }
}

impl From<&str> for UserRole {
    fn from(value: &str) -> Self {
        match value {
            consts::USER_ROLE_ADMIN => UserRole::Admin,
            consts::USER_ROLE_EDITOR => UserRole::Editor,
            consts::USER_ROLE_CONTRIBUTOR => UserRole::Contributor,
            _ => UserRole::NotFound,
        }
    }
}

impl Into<String> for UserRole {
    fn into(self) -> String {
        match self {
            UserRole::Admin => consts::USER_ROLE_ADMIN.to_string(),
            UserRole::Editor => consts::USER_ROLE_EDITOR.to_string(),
            UserRole::Contributor => consts::USER_ROLE_CONTRIBUTOR.to_string(),
            UserRole::NotFound => consts::USER_ROLE_NOT_FOUND.to_string(),
        }
    }
}

pub fn get_role_variant(role: &str) -> UserRole {
    match role {
        consts::USER_ROLE_ADMIN => UserRole::Admin,
        consts::USER_ROLE_EDITOR => UserRole::Editor,
        consts::USER_ROLE_CONTRIBUTOR => UserRole::Contributor,
        _ => UserRole::NotFound,
    }
}

mod test {
    use super::*;

    fn is_role_variant_correct() {
        let role_1 = "administrator";
        let role_2 = "editor";
        let role_3 = "contributor";
        let role_4 = "any_role";

        assert_eq!(UserRole::Admin, get_role_variant(role_1));
        assert_eq!(UserRole::Editor, get_role_variant(role_2));
        assert_eq!(UserRole::Contributor, get_role_variant(role_3));
        assert_eq!(UserRole::NotFound, get_role_variant(role_4));
    }
}
