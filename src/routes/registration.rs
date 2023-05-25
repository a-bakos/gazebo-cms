use crate::database::db::DB_Table;
use crate::routes::user::SqlxError;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Error;
use sqlx::Row;
use warp::reject::Reject;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub login: String,
    pub email: String,
    pub password: String,
}

pub enum AccountExistsCheckBy {
    Email,
    Login,
}

// pub async fn check_account_exists_by_email(pool: PgPool, email: String) -> Result<bool, String> {
//     let query = format!("SELECT id FROM {} WHERE email = $1", DB_Table::Accounts);
//     match sqlx::query(&query).bind(email).fetch_optional(&pool).await {
//         Ok(Some(_)) => Ok(true), // email found
//         Ok(None) => Ok(false),   // email not found
//         Err(e) => Err(format!("Database error {}", e)),
//     }
// }

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

pub async fn registration(
    pool: PgPool,
    params: NewAccountRegistrationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let email = params.email.clone(); // need email check

    // check if user exists in accounts table
    let account_exists = check_account_exists(
        pool.clone(),
        AccountExistsCheckBy::Email,
        params.email.clone(),
    )
    .await;
    match account_exists {
        Ok(false) => {
            let password = params.password.clone(); // todo need password check

            // if !crate::users::user_manager::is_password_valid(params.password.clone()) {
            //     return Err(_);
            // }

            let login = params.login.clone(); // todo need login name check

            let role = crate::users::roles::UserRole::Contributor.to_string();

            let query = format!(
                "INSERT INTO {} ({}, {}, {}, {}) VALUES ($1, $2, $3, $4)",
                DB_Table::Accounts,
                crate::database::columns::COL_INDEX_ACCOUNT_LOGIN,
                crate::database::columns::COL_INDEX_ACCOUNT_PASSWORD,
                crate::database::columns::COL_INDEX_ACCOUNT_EMAIL,
                crate::database::columns::COL_INDEX_ACCOUNT_ROLE
            );
            match sqlx::query(&query)
                .bind(login)
                .bind(password)
                .bind(email)
                .bind(role)
                .execute(&pool)
                .await
            {
                Ok(_) => Ok(warp::reply::json(&"Registration successful")),
                Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
            }
        }
        Ok(true) => Ok(warp::reply::json(&"Email address already in use")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}
