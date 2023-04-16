use crate::consts;
use crate::database::{columns, db};
use crate::users::functions::turn_row_into_user;
use crate::users::user::User;
use std::error::Error;

#[allow(dead_code)]
pub struct UserManager {
    users: Vec<crate::users::user::UserID>,
}

#[allow(dead_code)]
impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}

#[allow(unused_variables)]
pub fn is_email_valid(email: &str) -> bool {
    true
}

pub fn is_password_valid(password: &str) -> bool {
    if password.len() < consts::MIN_PASSWORD_LENGTH {
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
    if get_user_by_email(email).is_ok() && get_user_by_email(email).unwrap().is_some() {
        return true;
    }
    false
}

pub fn get_user_by_email(email: &str) -> Result<Option<User>, Box<dyn Error>> {
    // todo: if is valid email
    //if !is_email_valid(&email) {
    //    return an error
    //}

    let csv_db = db::parse_csv(consts::FILE_PATH_USERS)?;
    let found_user;
    let mut user = None;
    for row in csv_db.iter() {
        if let Some(db_email) = row.get(columns::COL_INDEX_USER_EMAIL) {
            if db_email.to_lowercase() == email.to_lowercase() {
                found_user = row;
                user = Some(turn_row_into_user(found_user));
                break;
            }
        }
    }

    Ok(user)
}
