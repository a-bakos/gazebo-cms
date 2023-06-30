use crate::{
    app::App,
    users::{credentials::is_password_valid, roles::UserRole},
};

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserID(pub u32);

impl Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub last_login: String,
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
