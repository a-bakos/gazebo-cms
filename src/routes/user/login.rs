use crate::{
    database::{
        columns::{
            COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN,
            COL_INDEX_ACCOUNT_ROLE,
        },
        db::DB_Table,
    },
    errors::error_handler::ErrorResponse,
    http::status_code::HttpStatusCode,
    users::{
        credentials,
        credentials::{find_account_by_identifier, AccountIdentifier},
    },
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use warp::Buf;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

/// Account details to send back on login request
/// Default() is used with error cases
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginResponseAccountDetails {
    pub id: u32,
    pub login_name: String,
    pub email: String,
    pub role: String,
}

/// Login status variants
enum LoginStatus {
    Authorized,
    Unauthorized,
    ServerError,
}

/// This is the final structure that is returned on a login request
#[derive(Deserialize, Serialize)]
pub struct LoginResponseWithStatusCode {
    pub http_status_code: u32,
    pub account_details: LoginResponseAccountDetails,
}
impl LoginResponseWithStatusCode {
    fn response(
        login_status: LoginStatus,
        account_details: Option<LoginResponseAccountDetails>,
    ) -> Self {
        let (http_status_code, account_details) = match login_status {
            LoginStatus::Authorized => (HttpStatusCode::Ok.code(), account_details.unwrap()),
            LoginStatus::Unauthorized => (
                HttpStatusCode::Unauthorized.code(),
                LoginResponseAccountDetails::default(),
            ),
            LoginStatus::ServerError => (
                HttpStatusCode::InternalServerError.code(),
                LoginResponseAccountDetails::default(),
            ),
        };
        Self {
            http_status_code,
            account_details,
        }
    }
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
            LoginResponseAccountDetails {
                // Underscores' meaning in row.get():
                // we don't need to specify a default/fallback value because the cell will never be empty
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
            update_last_login_timestamp(pool.clone(), login_variant, binding).await;

            Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::Authorized,
                Some(user),
            )))
        }
        Err(_e) => Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
            LoginStatus::Unauthorized,
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
        // See if account exists with this "email" address
        let account_exists_by_email =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Email, email.clone()).await;

        // Deal with account exists question
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
                    Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                        LoginStatus::Unauthorized,
                        None,
                    )))
                }
            }
            Ok(false) => Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::Unauthorized,
                None,
            ))),
            Err(_e) => Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::ServerError,
                None,
            ))),
        };
    }

    if let Some(login) = params.login {
        // See if account exists with this "login" name
        let account_exists_by_login =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Login, login.clone()).await;

        // Deal with account exists question
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
                    Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                        LoginStatus::Unauthorized,
                        None,
                    )))
                }
            }
            Ok(false) => {
                // No account
                Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                    LoginStatus::Unauthorized,
                    None,
                )))
            }
            Err(_e) => Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::ServerError,
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

/// Update Last Login timestamp for user
pub async fn update_last_login_timestamp(
    pool: PgPool,
    login_variant: AccountIdentifier,
    value: String,
) {
    let update_last_login_query = format!(
        "UPDATE {} SET last_login = CURRENT_TIMESTAMP WHERE {} = $1",
        DB_Table::Accounts,
        get_column_name_by_login_variant(login_variant) // email or username
    );
    match sqlx::query(&update_last_login_query)
        .bind(value)
        .execute(&pool)
        .await
    {
        Ok(_) => println!("Last login datetime updated!"),
        Err(_e) => println!("Last login datetime update error!"),
    }
}
