// MOVE THIS INTO USER::CRUD crate

use crate::database::db::DB_Table;
use crate::users::user_manager;
use crate::users::user_manager::CheckAccountExistsBy;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub login: String,
    pub email: String,
    pub password: String,
}

pub async fn registration(
    pool: PgPool,
    params: NewAccountRegistrationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let email = params.email.clone(); // need email check

    // check if user exists in accounts table
    let account_exists = user_manager::check_account_exists(
        pool.clone(),
        CheckAccountExistsBy::Email,
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
                    // new_user_notification(user.id);
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
