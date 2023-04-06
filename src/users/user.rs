use crate::allocator::{ID_Allocator, ResourceID, ResourceType};
use crate::app::App;
use crate::dates::functions as date_functions;
use crate::users::functions as user_functions;
use crate::users::roles::UserRole;
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
        let _ = &app
            .resources
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
#[allow(dead_code)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub login_name: String,
    pub email: String,
    pub id: UserID,
    pub role: UserRole,
    password: String,
    pub registered: String,
}

impl User {
    #[allow(dead_code)]
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
            id: user_functions::get_next_available_user_id(app),
            role,
            password,
            registered: date_functions::get_current_date(),
        }
    }

    pub fn login(app: &mut App, user_email: &str) -> bool {
        println!("{} log in request", user_email);
        // dummy login functionality
        // see if user exists
        // get id if it does and push it into users vec
        app.users.push(user_email.to_string());
        true
    }

    pub fn change_username(&mut self, new_username: &str) {
        // username change functionality
    }

    pub fn reset_password(&mut self, new_password: &str) {
        // password reset functionality
    }
}

// TODO
// define( 'COOKIEHASH', md5( $siteurl ) );
// define( 'USER_COOKIE', 'wordpressuser_' . COOKIEHASH );
