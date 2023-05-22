use crate::routes::user::SqlxError;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Error;
use sqlx::Row;
use warp::reject::Reject;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub email: String,
    pub password: String,
}

pub async fn account_exists(pool: PgPool, email: String) -> Result<bool, String> {
    match sqlx::query("SELECT id FROM gb_accounts WHERE email = $1")
        .bind(email)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(_)) => Err("Email address already in use".to_string()),
        Ok(None) => Ok(false),
        Err(e) => Err(format!("Database error {}", e)),
    }
}

pub async fn registration(
    pool: PgPool,
    params: NewAccountRegistrationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // check if user exists in accounts table
    let account_exists = account_exists(pool.clone(), params.email.clone()).await;

    match account_exists {
        Ok(false) => {
            // todo
            // Registration logic goes here

            Ok(warp::reply::json(&"Registration successful"))
        }
        Ok(true) => Ok(warp::reply::json(&"Email address already in use")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}
