use crate::{app::App, users::roles::UserRole};

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

impl User {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
