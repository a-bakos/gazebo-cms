use crate::allocator::{ID_Allocator, ResourceID, ResourceManager, ResourceType};
use crate::app::App;
use crate::dates::date;
use crate::dates::date_functional;
use crate::users::roles::UserRole;
use crate::users::user_functional;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct UserID(pub u32);

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ID_Allocator for UserID {
    fn allocate(app: &mut App) -> Self {
        // resourcemanager to allocate user ID
        &app.resources
            .add_to_allocated(ResourceType::User, ResourceID::UserID(1));
        UserID(1)
    }
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
        app: &mut App,
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
            id: user_functional::get_next_available_user_id(app),
            role,
            password,
            registered: date_functional::get_current_date(),
        }
    }
}
