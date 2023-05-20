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
use crate::dates::functions as date_functions;
use crate::users::roles::UserRole;
use crate::users::user_manager::{is_email_valid, is_password_valid, user_exists};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub login_name: String,
    pub email: String,
    pub id: UserID,
    pub role: UserRole,
    pub password: String,
    pub registered: String,
}

impl User {
    #[allow(dead_code)]
    pub fn new(login_name: String, email: String, role: UserRole, password: String) -> Self {
        Self {
            login_name,
            email,
            id: UserID(crate::consts::USER_ID_TEMPORARY_DEFAULT),
            role,
            password,
            registered: date_functions::get_current_date(),
        }
    }

    #[allow(unused_variables)]
    pub fn insert(user: User, send_notification: bool) -> bool {
        // ID allocation is taken care of by the User::new() function

        if user_exists(&user.email) {
            return false;
        }

        // We don't need to check user role validity, because it can only be a variant of the UserRole enum
        if !is_email_valid(&user.email) || !is_password_valid(&user.password) {
            return false;
        }

        db::store_user(&user);
        dbg!(user);

        // todo: on successful insertion, maybe send notification email to user + admin
        if send_notification {
            // send email
            // new_user_notification(user.id);
            // send_notification_to_admin( NOTIFICATION::NEW_USER_ADDED)
        }

        true
    }

    pub fn login(app: &mut App, user_email: &str, password: &str) -> bool {
        println!("{user_email} log in request");
        // dummy login functionality

        if !user_exists(&user_email) {
            return false;
        }

        if !is_password_valid(&password) {
            return false;
        }

        // Add to users list - if in list: already logged in
        if !app.users.contains(&user_email.to_string()) {
            app.users.push(user_email.to_string());
            return true;
        }

        false
    }

    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn change_username(&mut self, new_username: &str) {
        // username change functionality
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
