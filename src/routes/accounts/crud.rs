use crate::{
    consts::LABEL_NONE,
    database::{
        columns::{
            COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LAST_LOGIN,
            COL_INDEX_ACCOUNT_LOGIN, COL_INDEX_ACCOUNT_PASSWORD, COL_INDEX_ACCOUNT_REGISTERED,
            COL_INDEX_ACCOUNT_ROLE,
        },
        db::DB_Table,
    },
    errors::error_handler::SqlxError,
    users::{
        credentials,
        credentials::{is_password_valid, AccountIdentifier},
        roles::get_role_variant,
        user::{User, UserID},
    },
};
use std::fmt::format;

use crate::entry::query::GB_QueryArg;
use crate::users::roles::UserRole;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use warp::body::form;
use warp::http::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub login: String,
    pub email: String,
    pub password: String,
}

pub async fn add(
    pool: PgPool,
    params: NewAccountRegistrationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let email = params.email.clone(); // need email check

    // check if accounts exists in accounts table
    let account_exists = credentials::find_account_by_identifier(
        pool.clone(),
        AccountIdentifier::Email,
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
                Ok(_) => {
                    // User notification to go out.
                    // new_user_notification(accounts.id);
                    // send_notification_to_admin( NOTIFICATION::NEW_USER_ADDED)

                    Ok(warp::reply::json(&"Registration successful"))
                }
                Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
            }
        }
        Ok(true) => Ok(warp::reply::json(&"Email address already in use")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}

pub async fn get_all_accounts(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("All accounts requested");
    let mut accounts: Vec<User> = Vec::new();
    let query = format!(
        "SELECT {}, {}, {}, {}, {}, {} FROM {}",
        COL_INDEX_ACCOUNT_LOGIN,
        COL_INDEX_ACCOUNT_EMAIL,
        COL_INDEX_ACCOUNT_ID,
        COL_INDEX_ACCOUNT_ROLE,
        COL_INDEX_ACCOUNT_REGISTERED,
        COL_INDEX_ACCOUNT_LAST_LOGIN,
        DB_Table::Accounts
    );
    match sqlx::query(&query)
        .map(|row: PgRow| {
            // Registered date
            let registered: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);
            let registered = registered.to_string();

            // Last login date
            let last_login: Option<NaiveDateTime> =
                row.get::<Option<NaiveDateTime>, _>(COL_INDEX_ACCOUNT_LAST_LOGIN);
            let last_login = match last_login {
                Some(last_login_date) => last_login_date.to_string(),
                None => String::from(LABEL_NONE),
            };

            let role: String = row.get(COL_INDEX_ACCOUNT_ROLE);
            let role: UserRole = get_role_variant(&role);

            let the_account = User {
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                id: UserID(row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32),
                role,
                password: "hidden".to_string(),
                registered,
                last_login,
            };
            accounts.push(the_account);
        })
        .fetch_all(&pool)
        .await
    {
        Ok(_res) => Ok(warp::reply::json(&accounts)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))), // Unhandled rejection: SqlxError(RowNotFound)
    }
}

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
            // Underscores' meaning here:
            // we don't need to specify a default/fallback value because the cell will never be empty

            // ID
            let user_id = row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;

            // Role
            let role = row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE);
            let role = get_role_variant(role);

            // Registered date
            let registered: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);
            let registered = registered.to_string();

            // Last login
            let last_login: Option<NaiveDateTime> =
                row.get::<Option<NaiveDateTime>, _>(COL_INDEX_ACCOUNT_LAST_LOGIN);
            let last_login = match last_login {
                Some(last_login_date) => last_login_date.to_string(),
                None => String::from(LABEL_NONE),
            };

            User {
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                id: UserID(user_id),
                role,
                password: row.get(COL_INDEX_ACCOUNT_PASSWORD), // todo: hide this later
                registered,
                last_login,
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

#[allow(dead_code)]
fn add_role_to_user(_user_id: u32, _role: UserRole) -> bool {
    // get accounts by id
    // check role
    // change role

    true
}

#[allow(dead_code)]
pub fn get_user_by_email(_email: &str) -> Option<User> {
    todo!()
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn change_username(user_id: UserID, new_username: &str) {
    // username change functionality
    // check if username valid
    // self.login = new_username
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn reset_password(user_id: UserID, new_password: &str) -> bool {
    // password reset functionality
    if is_password_valid(new_password) {
        // todo store new password logic here
        return true;
    }

    false
}
