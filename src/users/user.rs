use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use std::fmt::{Display, Formatter};

use crate::{
    consts::LABEL_NONE,
    database::columns::{
        COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LAST_LOGIN,
        COL_INDEX_ACCOUNT_LOGIN, COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
    },
    traits::RowTransformer,
    users::roles::{get_role_variant, AccountRole},
};

use gazebo_core_common::account::{
    gb_account::GB_Account,
    role::AccountRole as GB_Role,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountID(pub u32);

impl Display for AccountID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub login_name: String,
    pub email: String,
    pub id: AccountID,
    pub role: AccountRole,
    pub password: String,
    pub registered: String,
    pub last_login: String,
}

// This will be used eventually - the other one will be scrapped!
impl RowTransformer<PgRow> for GB_Account {
    type Output = GB_Account;

    fn transform(row: &PgRow) -> Self::Output {
        // Registered date
        let registered: NaiveDateTime = row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);
        let registered = registered.to_string();

        // Last login date
        let last_login: Option<NaiveDateTime> =
            row.get::<Option<NaiveDateTime>, _>(COL_INDEX_ACCOUNT_LAST_LOGIN);
        let last_login = match last_login {
            Some(last_login_date) => last_login_date.to_string(),
            None => String::from(LABEL_NONE),
        };

        let role: String = row.get(COL_INDEX_ACCOUNT_ROLE);
        let role: AccountRole = get_role_variant(&role);

        Self {
            login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
            email: row.get(COL_INDEX_ACCOUNT_EMAIL),
            id: AccountID(row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32),
            role,
            password: "hidden".to_string(),
            registered,
            last_login,
        }
    }
}

impl RowTransformer<PgRow> for Account {
    type Output = Account;

    fn transform(row: &PgRow) -> Self::Output {
        // Registered date
        let registered: NaiveDateTime = row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);
        let registered = registered.to_string();

        // Last login date
        let last_login: Option<NaiveDateTime> =
            row.get::<Option<NaiveDateTime>, _>(COL_INDEX_ACCOUNT_LAST_LOGIN);
        let last_login = match last_login {
            Some(last_login_date) => last_login_date.to_string(),
            None => String::from(LABEL_NONE),
        };

        let role: String = row.get(COL_INDEX_ACCOUNT_ROLE);
        let role: AccountRole = get_role_variant(&role);

        Self {
            login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
            email: row.get(COL_INDEX_ACCOUNT_EMAIL),
            id: AccountID(row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32),
            role,
            password: "hidden".to_string(),
            registered,
            last_login,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
