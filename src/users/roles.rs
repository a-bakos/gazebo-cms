use crate::consts;
use std::fmt::Formatter;

#[derive(Debug)]
#[allow(dead_code)]
pub enum UserRole {
    Admin,       // read, write, delete, add ??
    Editor,      // read, write, delete
    Contributor, // read
}
impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserRole::Admin => write!(f, "{}", consts::USER_ROLE_ADMIN),
            UserRole::Editor => write!(f, "{}", consts::USER_ROLE_EDITOR),
            UserRole::Contributor => write!(f, "{}", consts::USER_ROLE_CONTRIBUTOR),
        }
    }
}

pub fn get_role_variant(role: &str) -> UserRole {
    match role {
        consts::USER_ROLE_ADMIN => UserRole::Admin,
        consts::USER_ROLE_EDITOR => UserRole::Editor,
        consts::USER_ROLE_CONTRIBUTOR => UserRole::Contributor,
        _ => panic!("No role specified"),
    }
}
