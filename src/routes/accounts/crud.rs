use crate::{
    database::{
        columns::{
            COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LAST_LOGIN,
            COL_INDEX_ACCOUNT_LOGIN, COL_INDEX_ACCOUNT_PASSWORD, COL_INDEX_ACCOUNT_REGISTERED,
            COL_INDEX_ACCOUNT_ROLE,
        },
        db::DB_Table,
    },
    errors::error_handler::SqlxError,
    traits::RowTransformer,
    users::{
        credentials,
        credentials::{is_password_valid, AccountIdentifier},
    },
};

use gazebo_core_common::{
    account::{
        gb_account::{AccountID, GB_Account},
        role::{get_role_variant, AccountRole},
    },
    consts::LABEL_NONE,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
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

            let role = AccountRole::Contributor.to_string();

            let query = format!(
                "INSERT INTO {} ({}, {}, {}, {}) VALUES ($1, $2, $3, $4)",
                DB_Table::Accounts,
                COL_INDEX_ACCOUNT_LOGIN,
                COL_INDEX_ACCOUNT_PASSWORD,
                COL_INDEX_ACCOUNT_EMAIL,
                COL_INDEX_ACCOUNT_ROLE
            );
            match sqlx::query(&query)
                .bind(login)
                .bind(password)
                .bind(email)
                .bind(role)
                .fetch_one(&pool)
                .await
            {
                Ok(result) => {
                    let account_id = result.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;

                    // User notification to go out.
                    // new_user_notification(accounts.id);
                    // send_notification_to_admin( NOTIFICATION::NEW_USER_ADDED)

                    // todo: work on response
                    // Ok(warp::reply::json(&GB_AccountInsertResponse {
                    //     http_status_code: 200,
                    //     account_id: AccountID(account_id),
                    // }))
                    Ok(warp::reply::json(&"Registration successful"))
                }
                Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
            }
        }
        Ok(true) => Ok(warp::reply::json(&"Email address already in use")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}

pub async fn get_all_accounts(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("All accounts requested");
    let mut accounts: Vec<GB_Account> = Vec::new();
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
            let the_account = GB_Account::transform(&row);
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
        .map(|row: PgRow| GB_Account::transform(&row))
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
fn add_role_to_user(_user_id: u32, _role: AccountRole) -> bool {
    // get accounts by id
    // check role
    // change role

    true
}

#[allow(dead_code)]
pub fn get_user_by_email(_email: &str) -> Option<GB_Account> {
    todo!()
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn change_username(user_id: AccountID, new_username: &str) {
    // username change functionality
    // check if username valid
    // self.login = new_username
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn reset_password(user_id: AccountID, new_password: &str) -> bool {
    // password reset functionality
    if is_password_valid(new_password) {
        // todo store new password logic here
        return true;
    }

    false
}
