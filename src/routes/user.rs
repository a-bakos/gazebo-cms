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
use crate::routes::registration::{check_account_exists, AccountExistsCheckBy};

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
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub async fn login(
    pool: PgPool,
    params: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // if email found, ignore login name
    // if no email, look for login name
    if let Some(email) = params.email {
        let account_exists_by_email =
            check_account_exists(pool.clone(), AccountExistsCheckBy::Email, email.clone()).await;

        return match account_exists_by_email {
            Ok(true) => {
                // Acc exists / go login
                let query = format!("SELECT * FROM {} WHERE email = $1", DB_Table::Accounts);
                let binding = email.clone();
                let password = params.password.clone();
                if is_password_match(&pool, &password, AccountExistsCheckBy::Email, &binding).await
                {
                    // Try login and return result
                    try_login(&query, &pool, password, binding).await
                } else {
                    Ok(warp::reply::json(&format!(
                        "Pass doesn't match (from email login)!"
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&"Email address not found")),
            Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
        };
    }

    if let Some(login) = params.login {
        let query = format!("SELECT * FROM {} WHERE login = $1", DB_Table::Accounts);
        let binding = login.clone();
        let account_exists_by_login =
            check_account_exists(pool.clone(), AccountExistsCheckBy::Login, login.clone()).await;

        return match account_exists_by_login {
            Ok(true) => {
                // Acc exists
                let query = format!("SELECT * FROM {} WHERE login = $1", DB_Table::Accounts);
                let binding = login.clone();
                let password = params.password.clone();
                if is_password_match(&pool, &password, AccountExistsCheckBy::Login, &binding).await
                {
                    // Try login and return result
                    try_login(&query, &pool, password, binding).await
                } else {
                    Ok(warp::reply::json(&format!(
                        "Pass doesn't match (from login name login)!"
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&"Login address not found")),
            Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
        };
    }

    let response = ErrorResponse {
        error: "Empty login".to_owned(),
    };
    Ok(warp::reply::json(&response))
}

// Wrapper type so we have a size at compile time
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

const MSG_LOGIN_SUCCESS: &str = "Login successful";
pub async fn try_login(
    query: &str,
    pool: &PgPool,
    password: String,
    binding: String,
) -> Result<warp::reply::Json, warp::Rejection> {
    match sqlx::query(&query)
        .bind(password)
        .bind(binding)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(warp::reply::json(&MSG_LOGIN_SUCCESS)),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
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
        Ok(res) => true,
        Err(err) => false,
    }
}
