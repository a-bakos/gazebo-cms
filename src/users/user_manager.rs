use crate::consts;
use crate::database::columns::COL_INDEX_ACCOUNT_ID;
use crate::database::db::DB_Table;
use crate::{
    helpers::{str_contains_number, str_contains_special_char, str_contains_uppercase},
    users::user::UserID,
};
use sqlx::{PgPool, Row};

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

#[allow(unused_variables, dead_code)]
pub fn is_email_valid(email: &str) -> bool {
    true
}

#[allow(dead_code)]
pub fn is_username_valid(username: &str) -> bool {
    // Min length validation
    let mut min_length_ok = false;
    if username.len() >= consts::MIN_USER_NAME_LENGTH {
        min_length_ok = true;
    }

    // Make sure it doesn't contain special characters
    let special_characters = str_contains_special_char(username); // must be false

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

    // Password length check
    if password.len() >= consts::MIN_PASSWORD_LENGTH {
        ok_pw_len = true;
    }

    // They all need to be true
    let ok_pw_uppercase = str_contains_uppercase(password);
    let ok_pw_numeric = str_contains_number(password);
    let ok_pw_special = str_contains_special_char(password);

    if ok_pw_numeric && ok_pw_uppercase && ok_pw_len && ok_pw_special {
        return true;
    }
    false
}

pub async fn is_password_match(
    pool: &PgPool,
    password: &str,
    check_by: AccountExistsCheckBy,
    value: &str,
) -> bool {
    let query = match check_by {
        AccountExistsCheckBy::Email => {
            format!(
                "SELECT id FROM {} WHERE email = $1 AND password = $2",
                DB_Table::Accounts
            )
        }
        AccountExistsCheckBy::Login => {
            format!(
                "SELECT id FROM {} WHERE login = $1 AND password = $2",
                DB_Table::Accounts
            )
        }
    };

    match sqlx::query(&query)
        .bind(value)
        .bind(password)
        .map(|row| {
            let user_id = row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;
            UserID(user_id)
        })
        .fetch_one(pool)
        .await
    {
        Ok(_res) => true,
        Err(_err) => false,
    }
}

pub async fn check_account_exists(
    pool: PgPool,
    param: AccountExistsCheckBy,
    value: String,
) -> Result<bool, String> {
    let query;
    match param {
        AccountExistsCheckBy::Email => {
            query = format!("SELECT id FROM {} WHERE email = $1", DB_Table::Accounts);
        }
        AccountExistsCheckBy::Login => {
            query = format!("SELECT id FROM {} WHERE login = $1", DB_Table::Accounts);
        }
    }

    match sqlx::query(&query).bind(value).fetch_optional(&pool).await {
        Ok(Some(_)) => Ok(true), // email | login found
        Ok(None) => Ok(false),   // email | login not found
        Err(e) => Err(format!("Database error {}", e)),
    }
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

pub enum AccountExistsCheckBy {
    Email,
    Login,
}
