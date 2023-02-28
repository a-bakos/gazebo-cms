#[derive(Debug)]
pub struct UserID(pub u32);

#[derive(Debug)]
pub enum UserRole {
    Admin,
    Editor,
    Visitor,
}

#[derive(Debug)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    id: UserID,
    role: UserRole,
}
