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

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn get_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    println!("User requested by ID: {}", id);
    match sqlx::query("SELECT * FROM gb_accounts WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| {
            let user_id = row.get::<i32, _>("id") as u32;

            let user_role = row.get::<&str, _>("role");
            let user_role = get_role_variant(user_role);

            // Don't need to specify a default/fallback value because the cell will never be empty
            let registered: NaiveDateTime = row.get::<NaiveDateTime, _>("registered");

            User {
                login_name: row.get("login"),
                email: row.get("email"),
                id: UserID(user_id),
                role: user_role,
                password: row.get("password"),
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
