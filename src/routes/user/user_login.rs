use crate::database::db::DB_Table;
use crate::errors::error_handler::ErrorResponse;
use crate::http::response::HttpResponse;
use crate::routes::registration::{check_account_exists, AccountExistsCheckBy};
use crate::users::user_manager;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
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
                if user_manager::is_password_match(
                    &pool,
                    &password,
                    AccountExistsCheckBy::Email,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(&query, &pool, password, binding).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);

                    // Client response
                    Ok(warp::reply::json(&HttpResponse::unauthorized()))
                }
            }
            Ok(false) => Ok(warp::reply::json(&HttpResponse::unauthorized())),
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
                if user_manager::is_password_match(
                    &pool,
                    &password,
                    AccountExistsCheckBy::Login,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(&query, &pool, password, binding).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);
                    // Client response
                    Ok(warp::reply::json(&HttpResponse::unauthorized()))
                }
            }
            Ok(false) => Ok(warp::reply::json(&HttpResponse::unauthorized())),
            Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
        };
    }

    Ok(warp::reply::json(&ErrorResponse::new(
        "Empty login".to_owned(),
    )))
}
