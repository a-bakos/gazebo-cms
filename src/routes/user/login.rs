use crate::{
    database::{
        columns::{
            COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN,
            COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
        },
        db::DB_Table,
    },
    errors::error_handler::ErrorResponse,
    http::status_code::HttpStatusCode,
    users::{
        credentials,
        credentials::{find_account_by_identifier, AccountIdentifier},
        roles::get_role_variant,
    },
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginResponse {
    pub id: u32,
    pub login_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponseWithStatusCode(pub u32, pub LoginResponse);

pub async fn try_login(
    pool: PgPool,
    password: String,
    binding: String,
    login_variant: AccountIdentifier,
) -> Result<warp::reply::Json, warp::Rejection> {
    let column_name = match login_variant {
        AccountIdentifier::Email => COL_INDEX_ACCOUNT_EMAIL,
        AccountIdentifier::Login => COL_INDEX_ACCOUNT_LOGIN,
    };
    let query = format!(
        "SELECT * FROM {} WHERE {} = $1 AND password = $2",
        DB_Table::Accounts,
        column_name
    );
    match sqlx::query(&query)
        .bind(binding.clone())
        .bind(password)
        .map(|row: PgRow| {
            // Underscores' meaning here:
            // we don't need to specify a default/fallback value because the cell will never be empty
            LoginResponse {
                id: row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32,
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                role: row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE).to_string(),
            }
        })
        .fetch_one(&pool)
        .await
    {
        Ok(user) => {
            let update_last_login_query = format!(
                "UPDATE {} SET last_login = CURRENT_TIMESTAMP WHERE {} = $1",
                DB_Table::Accounts,
                column_name // email or username
            );
            match sqlx::query(&update_last_login_query)
                .bind(binding)
                .execute(&pool)
                .await
            {
                Ok(_) => println!("Last login datetime updated!"),
                Err(e) => println!("Last login datetime update error!"),
            }

            Ok(warp::reply::json(&LoginResponseWithStatusCode(
                HttpStatusCode::Ok.code(),
                user,
            )))
        }
        Err(e) => Ok(warp::reply::json(&LoginResponseWithStatusCode(
            HttpStatusCode::Unauthorized.code(),
            LoginResponse::default(),
        ))),
    }
}

pub async fn login(
    pool: PgPool,
    params: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // println!("{:?}", params);
    // If email found, ignore login name
    // If no email, look for login name
    if let Some(email) = params.email {
        let account_exists_by_email =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Email, email.clone()).await;

        return match account_exists_by_email {
            Ok(true) => {
                /**
                 * Account exists, now go and login
                 */
                let binding = email.clone();
                let password = params.password.clone();
                if credentials::is_password_match(
                    &pool,
                    &password,
                    AccountIdentifier::Email,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(pool.clone(), password, binding, AccountIdentifier::Email).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);
                    // Client response
                    Ok(warp::reply::json(&LoginResponseWithStatusCode(
                        HttpStatusCode::Unauthorized.code(),
                        LoginResponse::default(),
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&LoginResponseWithStatusCode(
                HttpStatusCode::Unauthorized.code(),
                LoginResponse::default(),
            ))),
            Err(_e) => Ok(warp::reply::json(&LoginResponseWithStatusCode(
                HttpStatusCode::InternalServerError.code(),
                LoginResponse::default(),
            ))),
        };
    }

    if let Some(login) = params.login {
        let account_exists_by_login =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Login, login.clone()).await;

        return match account_exists_by_login {
            Ok(true) => {
                // Acc exists
                let binding = login.clone();
                let password = params.password.clone();
                if credentials::is_password_match(
                    &pool,
                    &password,
                    AccountIdentifier::Login,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(pool.clone(), password, binding, AccountIdentifier::Login).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);
                    // Client response
                    Ok(warp::reply::json(&LoginResponseWithStatusCode(
                        HttpStatusCode::Unauthorized.code(),
                        LoginResponse::default(),
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&LoginResponseWithStatusCode(
                HttpStatusCode::Unauthorized.code(),
                LoginResponse::default(),
            ))),
            Err(e) => Ok(warp::reply::json(&LoginResponseWithStatusCode(
                HttpStatusCode::InternalServerError.code(),
                LoginResponse::default(),
            ))),
        };
    }

    Ok(warp::reply::json(&ErrorResponse::new(
        "Empty login".to_owned(),
    )))
}
