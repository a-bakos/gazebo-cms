use crate::consts;
use crate::database::{columns, db};
use crate::{
    helpers::{str_contains_number, str_contains_special_char, str_contains_uppercase},
    users::{
        functions::turn_row_into_user,
        user::{User, UserID},
    },
};
use std::error::Error;

#[allow(dead_code)]
pub struct UserManager {
    users: Vec<UserID>,
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

pub fn is_username_valid(username: &str) -> bool {
    // Min length validation
    let mut min_length_ok = false;
    if username.len() >= consts::MIN_USER_NAME_LENGTH {
        min_length_ok = true;
    }

    // Make sure it doesn't contain special characters
    let mut special_characters = true;
    special_characters = crate::helpers::str_contains_special_char(username); // must be false

    min_length_ok && !special_characters
}

/// Checks if a password is valid.
///
/// A password is considered valid if it meets the following criteria:
///
/// - It is at least `consts::MIN_PASSWORD_LENGTH` characters long
/// - It contains at least one uppercase letter
/// - It contains at least one number
/// - It contains at least one special character
///
/// # Arguments
///
/// - `password` - A string slice containing the password to check
///
/// # Returns
///
/// Returns `true` if the password is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// let valid_password = "#Abcd1234!";
/// assert_eq!(is_password_valid(valid_password), true);
/// ```
///
/// This function also has a unit test suite in the same module.
pub fn is_password_valid(password: &str) -> bool {
    let mut ok_pw_len: bool = false;
    let mut ok_pw_uppercase: bool = false;
    let mut ok_pw_numeric: bool = false;
    let mut ok_pw_special: bool = false;

    // Password length check
    if password.len() >= consts::MIN_PASSWORD_LENGTH {
        ok_pw_len = true;
    }

    // They all need to be true
    ok_pw_uppercase = str_contains_uppercase(password);
    ok_pw_numeric = str_contains_number(password);
    ok_pw_special = str_contains_special_char(password);

    if ok_pw_numeric && ok_pw_uppercase && ok_pw_len && ok_pw_special {
        return true;
    }
    false
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
        if let Some(db_email) = row.get("email".parse().unwrap()) {
            if db_email.to_lowercase() == email.to_lowercase() {
                found_user = row;
                user = Some(turn_row_into_user(found_user));
                break;
            }
        }
    }

    Ok(user)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn are_password_variations_valid() {
        let password_correct = "#Abcd1234!";
        let password_incorrect_too_short = "_Abc1";
        let password_incorrect_missing_number = "?ABCdefGH";
        let password_incorrect_missing_special = "abcDEF123";
        let password_incorrect_missing_uppercase = "@abcde1234";

        assert_eq!(is_password_valid(password_correct), true);
        assert_eq!(is_password_valid(password_incorrect_too_short), false);
        assert_eq!(is_password_valid(password_incorrect_missing_number), false);
        assert_eq!(is_password_valid(password_incorrect_missing_special), false);
        assert_eq!(
            is_password_valid(password_incorrect_missing_uppercase),
            false
        );
    }
}
