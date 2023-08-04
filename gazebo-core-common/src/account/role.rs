use serde::{Deserialize, Serialize};
use crate::consts::{USER_ROLE_ADMIN,USER_ROLE_EDITOR,USER_ROLE_CONTRIBUTOR};

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
