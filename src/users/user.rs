use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use std::fmt::{Display, Formatter};

use crate::{
    database::columns::{
        COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LAST_LOGIN,
        COL_INDEX_ACCOUNT_LOGIN, COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
    },
    traits::{IntoSqlQuery, RowTransformer},
};

use gazebo_core_common::{
    account::{
        gb_account::{AccountID, GB_Account},
        role::{get_role_variant, AccountRole},
    },
    consts::{LABEL_NONE, MSG_HIDDEN_INFO},
};

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
            password: MSG_HIDDEN_INFO.to_string(),
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
