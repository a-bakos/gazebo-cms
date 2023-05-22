use crate::database::columns::COL_INDEX_USER_PASSWORD;
use crate::database::db::DB_Table;
use crate::users::roles::{get_role_variant, UserRole};
use crate::users::user::{User, UserID};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Error;
use sqlx::Row;
use std::num::ParseIntError;
use warp::reject::Reject;

#[derive(Debug)]
pub struct SqlxError(pub Error);

impl Reject for SqlxError {}

pub const COL_INDEX_ACCOUNT_ID: &str = "id";
pub const COL_INDEX_ACCOUNT_EMAIL: &str = "email";
pub const COL_INDEX_ACCOUNT_PASSWORD: &str = "email";
pub const COL_INDEX_ACCOUNT_ROLE: &str = "role";
pub const COL_INDEX_ACCOUNT_LOGIN: &str = "login";
pub const COL_INDEX_ACCOUNT_REGISTERED: &str = "registered";

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn get_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("User requested by ID: {}", id);
    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let table_name: String = DB_Table::Accounts.to_string();
    let query = format!("SELECT * FROM {} WHERE id = $1", table_name);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| {
            let user_id = row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;

            let user_role = row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE);
            let user_role = get_role_variant(user_role);

            // Don't need to specify a default/fallback value because the cell will never be empty
            let registered: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);

            User {
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                id: UserID(user_id),
                role: user_role,
                password: row.get(COL_INDEX_USER_PASSWORD), // todo: hide this later
                registered: registered.to_string(),
            }
        })
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}
