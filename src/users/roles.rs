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
            UserRole::Admin => write!(f, "admin"),
            UserRole::Editor => write!(f, "editor"),
            UserRole::Contributor => write!(f, "contributor"),
        }
    }
}
