use crate::database::columns::{
    COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN,
    COL_INDEX_ACCOUNT_PASSWORD, COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
};
use crate::database::db::DB_Table;
use crate::users::roles::{get_role_variant, UserRole};
use crate::users::user::{User, UserID};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Error;
use sqlx::Row;
use std::collections::HashMap;
use std::num::ParseIntError;
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::database::columns;
use crate::routes::registration::account_exists;

#[derive(Debug)]
pub struct SqlxError(pub Error);
impl Reject for SqlxError {}

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn get_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("User requested by ID: {}", id);
    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Accounts);
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
                password: row.get(COL_INDEX_ACCOUNT_PASSWORD), // todo: hide this later
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

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn delete_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("User to be deleted ID: {}", id);

    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let query = format!("DELETE FROM {} WHERE id = $1", DB_Table::Accounts);
    match sqlx::query(&query).bind(id).execute(&pool).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("User {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: String,
    pub email: String,
    pub password: String,
}

pub async fn login(
    pool: PgPool,
    params: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let email = params.email.clone(); // need email check

    // check if user exists in accounts table
    let account_exists = account_exists(pool.clone(), params.email.clone()).await;
    match account_exists {
        Ok(true) => {
            let password = params.password.clone(); // todo need password check

            // if !crate::users::user_manager::is_password_valid(params.password.clone()) {
            //     return Err(_);
            // }

            let login = params.login.clone(); // todo need login name check

            let query = format!("SELECT * FROM {} WHERE email = $1", DB_Table::Accounts);
            match sqlx::query(&query)
                .bind(password)
                .bind(email)
                .execute(&pool)
                .await
            {
                Ok(_) => Ok(warp::reply::json(&"Login successful")),
                Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
            }
        }
        Ok(false) => Ok(warp::reply::json(&"Email address not found")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}
