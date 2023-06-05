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

use crate::allocator::{ID_Allocator, ResourceID, ResourceType};
use crate::app::App;
use crate::database::db;
use crate::datetime::functions as date_functions;
use crate::users::roles::UserRole;
use crate::users::user_manager::{is_email_valid, is_password_valid};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserID(pub u32);

impl Display for UserID {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login_name: String,
    pub email: String,
    pub id: UserID,
    pub role: UserRole,
    pub password: String,
    pub registered: String,
}

impl User {
    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn change_username(&mut self, new_username: &str) {
        // username change functionality
        // self.login = new_username
    }

    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn reset_password(&mut self, new_password: &str) -> bool {
        // password reset functionality
        if is_password_valid(new_password) {
            // todo store new password logic here
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}

// TODO
// define( 'COOKIEHASH', md5( $siteurl ) );
// define( 'USER_COOKIE', 'wordpressuser_' . COOKIEHASH );
