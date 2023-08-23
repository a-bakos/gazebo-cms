use crate::users::credentials::{is_email_valid, is_password_valid, is_username_valid};
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
    pub token: String,
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

    // todo
    // Maybe here allocate tokens??

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
                token: "HARDCODED_TOKEN".to_string(),
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
        // If email or password format is invalid, terminate, don't even go to the database
        if !is_email_valid(&email) || !is_password_valid(&params.password) {
            return Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::Unauthorized,
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
        // If username or password format is invalid, terminate, don't even go to the database
        if !is_username_valid(&login) || !is_password_valid(&params.password) {
            return Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                LoginStatus::Unauthorized,
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
                    Ok(warp::reply::json(&LoginResponseWithStatusCode::response(
                        LoginStatus::Unauthorized,
                        None,
                    )))
                }
            }
            Ok(false) => {
                // No accounts
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

/// Update Last Login timestamp for accounts
pub async fn update_last_login_timestamp(
    pool: PgPool,
    login_variant: AccountIdentifier,
    value: String,
) {
    let update_last_login_query = format!(
        "UPDATE {} SET last_login = CURRENT_TIMESTAMP, uuid = $1 WHERE {} = $2",
        DB_Table::Accounts,
        get_column_name_by_login_variant(login_variant) // email or username
    );
    let uuid: uuid::Uuid = crate::auth::generate_session_id();
    match sqlx::query(&update_last_login_query)
        .bind(uuid)
        .bind(value)
        .execute(&pool)
        .await
    {
        Ok(_) => println!("Last login datetime updated!"),
        Err(e) => println!("Last login datetime update error! {:?}", e),
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenAuthParams {
    pub token: String,
}

pub async fn token_auth(
    pool: PgPool,
    params: TokenAuthParams,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("TOKEN FROM FRONTEND: {}", params.token);

    // Think about UUID
    // and bearer tokens

    let response = "RESPONSE FROM SERVER";
    Ok(warp::reply::json(&response))
}
