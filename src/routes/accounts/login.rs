use crate::{
    auth::TokenClaims,
    database::{
        columns::{
            COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN,
            COL_INDEX_ACCOUNT_ROLE,
        },
        db::DB_Table,
    },
    errors::error_handler::ErrorResponse,
    users::{
        credentials,
        credentials::{
            find_account_by_identifier, is_email_valid, is_password_valid, is_username_valid,
            AccountIdentifier,
        },
    },
};

use gazebo_core_common::{
    account::{
        auth::{AuthResponseAccountInfo, AuthResponsePayload},
        gb_account::AccountID,
        login::LoginStatus,
        role::AccountRole,
    },
    status_code::HttpStatusCode,
};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

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
            AuthResponseAccountInfo {
                // Underscores' meaning in row.get():
                // we don't need to specify a default/fallback value because the cell will never be empty
                id: AccountID(row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32),
                role: row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE).into(),
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
            }
        })
        .fetch_one(&pool)
        .await
    {
        Ok(mut user) => {
            let uuid: Option<uuid::Uuid> =
                update_login_timestamp_and_session_id(pool.clone(), login_variant, binding).await;

            if uuid.is_none() {
                unreachable!();
            }

            // Generate user access token and attach to response
            let the_token: Option<String> = crate::auth::generate_token(&user, &uuid.unwrap());
            if the_token.is_none() {
                return Ok(warp::reply::json(&AuthResponsePayload::response(
                    LoginStatus::Unauthorized,
                    None,
                    None,
                )));
            }

            Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::Authorized,
                Some(user),
                the_token,
            )))
        }
        Err(_e) => Ok(warp::reply::json(&AuthResponsePayload::response(
            LoginStatus::Unauthorized,
            None,
            None,
        ))),
    }
}

// TODO: can potentially be optimized by abstracting the inner logic
pub async fn login(
    pool: PgPool,
    params: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // println!("{:?}", params);
    // If email found, ignore login name
    // If no email, look for login name
    if let Some(email) = params.email {
        // If email or password format is invalid, terminate, don't even go to the database
        if !is_email_valid(&email) || !is_password_valid(&params.password) {
            return Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::Unauthorized,
                None,
                None,
            )));
        }

        // See if accounts exists with this "email" address
        let account_exists_by_email =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Email, email.clone()).await;

        // Deal with accounts exists question
        return match account_exists_by_email {
            Ok(true) => {
                // Account exists, now go and login
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
                    try_login(pool.clone(), password, binding, AccountIdentifier::Email).await
                } else {
                    println!("Wrong password used for: {}", &binding);
                    Ok(warp::reply::json(&AuthResponsePayload::response(
                        LoginStatus::Unauthorized,
                        None,
                        None,
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::Unauthorized,
                None,
                None,
            ))),
            Err(_e) => Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::ServerError,
                None,
                None,
            ))),
        };
    }

    if let Some(login) = params.login {
        // If username or password format is invalid, terminate, don't even go to the database
        if !is_username_valid(&login) || !is_password_valid(&params.password) {
            return Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::Unauthorized,
                None,
                None,
            )));
        }

        // See if accounts exists with this "login" name
        let account_exists_by_login =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Login, login.clone()).await;

        // Deal with accounts exists question
        return match account_exists_by_login {
            Ok(true) => {
                // Account exists
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
                    try_login(pool.clone(), password, binding, AccountIdentifier::Login).await
                } else {
                    // Account exists, but wrong creds
                    println!("Wrong password used for: {}", &binding);
                    Ok(warp::reply::json(&AuthResponsePayload::response(
                        LoginStatus::Unauthorized,
                        None,
                        None,
                    )))
                }
            }
            Ok(false) => {
                // No accounts
                Ok(warp::reply::json(&AuthResponsePayload::response(
                    LoginStatus::Unauthorized,
                    None,
                    None,
                )))
            }
            Err(_e) => Ok(warp::reply::json(&AuthResponsePayload::response(
                LoginStatus::ServerError,
                None,
                None,
            ))),
        };
    }

    Ok(warp::reply::json(&ErrorResponse::new(
        "Empty login".to_owned(),
    )))
}

// Get the "email" or "login" column name based on the login variant
fn get_column_name_by_login_variant<'a>(login_variant: AccountIdentifier) -> &'a str {
    match login_variant {
        AccountIdentifier::Email => COL_INDEX_ACCOUNT_EMAIL,
        AccountIdentifier::Login => COL_INDEX_ACCOUNT_LOGIN,
    }
}

/// Update Last Login timestamp and UUID/Session ID for accounts
///
/// While login timestamp and UUIDs are technically different things, they are kept together here.
/// We only need to generate a new session ID when a user logs in, and then we need to update the
/// login timestamp anyway. This way, we can do both in the same query.
pub async fn update_login_timestamp_and_session_id(
    pool: PgPool,
    login_variant: AccountIdentifier,
    value: String,
) -> Option<uuid::Uuid> {
    let update_query = format!(
        "UPDATE {} SET last_login = CURRENT_TIMESTAMP, uuid = $1 WHERE {} = $2",
        DB_Table::Accounts,
        get_column_name_by_login_variant(login_variant) // email or username
    );
    let uuid: uuid::Uuid = crate::auth::generate_session_id();
    match sqlx::query(&update_query)
        .bind(uuid.clone())
        .bind(value)
        .execute(&pool)
        .await
    {
        Ok(_) => {
            println!("Last login datetime + UUID updated!");
            Some(uuid)
        }
        Err(e) => {
            println!("Last login datetime update or UUID error! {:?}", e);
            None
        }
    }
}
