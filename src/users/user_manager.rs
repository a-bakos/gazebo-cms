use crate::app::App;
use crate::users::roles::UserRole;
use crate::users::user::User;

pub struct UserManager {
    users: Vec<crate::users::user::UserID>,
}

impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}

pub fn is_email_valid(email: &str) -> bool {
    true
}

pub fn is_password_valid(password: &str) -> bool {
    if password.len() < crate::consts::MIN_PASSWORD_LENGTH {
        // Password must be at least crate::consts::MIN_PASSWORD_LENGTH characters long!
        return false;
    }

    // check strength
    // check capital letters
    // check numbers
    // check special chars
    true
}

pub fn user_exists(email: &str) -> bool {
    // get user db/table and look for the email address
    // if found return true
    if get_user_by_email(email).is_some() {
        return true;
    }
    panic!("Couldn't find user");
    false
}

pub fn get_user_by_email(email: &str) -> Option<User> {
    todo!()
}
