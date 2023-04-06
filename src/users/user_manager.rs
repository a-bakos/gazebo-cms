use crate::app::App;

pub struct UserManager {
    users: Vec<crate::users::user::UserID>,
}

impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}
