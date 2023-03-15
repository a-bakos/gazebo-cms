#[derive(Debug)]
pub enum UserRole {
    Admin,   // read, write, delete, add ??
    Editor,  // read, write, delete
    Visitor, // read
}
