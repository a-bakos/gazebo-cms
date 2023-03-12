use std::fmt::Formatter;
use crate::allocator::{ID_Allocator, ResourceID, ResourceManager, ResourceType};
use crate::date;

#[derive(Debug)]
pub struct UserID(pub u32);

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl ID_Allocator for UserID {
    fn allocate() -> Self {
        // resourcemanager to allocate user ID
        UserID(1)
    }
}

#[derive(Debug)]
pub enum UserRole {
    Admin,   // read, write, delete, add ??
    Editor,  // read, write, delete
    Visitor, // read
}

/*
 * string $nickname
 * string $description
 * string $user_description
 * [x] string $first_name
 * string $user_firstname
 * [x] string $last_name
 * string $user_lastname
 * [x] string $user_login
 * [x] string $user_pass
 * string $user_nicename
 * [x] string $user_email
 * string $user_url
 * string $user_registered
 * string $user_activation_key
 * string $user_status
 * int    $user_level
 * string $display_name
 * string $spam
 * string $deleted
 * string $locale
 * string $rich_editing
 * string $syntax_highlighting
 * string $use_ssl
 */

#[derive(Debug)]
pub struct User {
    first_name: String,
    last_name: String,
    login_name: String,
    email: String,
    id: UserID,
    role: UserRole,
    password: String,
    registered: String,
}

impl User {
    fn new(
        first_name: String,
        last_name: String,
        login_name: String,
        email: String,
        role: UserRole,
        password: String,
    ) -> Self {
        Self {
            first_name,
            last_name,
            login_name,
            email,
            id: get_next_available_user_id(),
            role,
            password,
            registered: date::get_current_date(),
        }
    }
}

fn get_next_available_user_id() -> UserID {
    UserID::allocate()
}

fn get_current_user() -> User {
    todo!()
}