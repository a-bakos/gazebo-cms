#[derive(Debug)]
#[allow(dead_code)]
pub enum UserRole {
    Admin,   // read, write, delete, add ??
    Editor,  // read, write, delete
    Visitor, // read
}
