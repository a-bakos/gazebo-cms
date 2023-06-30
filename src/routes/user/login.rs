use crate::database::columns::{COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_LOGIN};
use crate::database::db::DB_Table;
use crate::errors::error_handler::ErrorResponse;
use crate::http::response::HttpResponse;
use crate::users::credentials;
use crate::users::credentials::find_account_by_identifier;
use crate::users::credentials::AccountIdentifier;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
        "SELECT * FROM {} WHERE {} = $1",
        DB_Table::Accounts,
        column_name
    );
    match sqlx::query(&query)
        .bind(password)
        .bind(binding.clone())
        .execute(&pool)
        .await
    {
        Ok(_) => {
            // Add to users list - if in list: already logged in
            // if !app.users.contains(&user_email.to_string()) {
            //     app.users.push(user_email.to_string());
            // }
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
                Ok(_) => println!("Last login updated!"),
                Err(e) => println!("Last login update error"),
            }

            Ok(warp::reply::json(&crate::consts::MSG_LOGIN_SUCCESS))
        }
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
            find_account_by_identifier(pool.clone(), AccountIdentifier::Email, email.clone()).await;

        return match account_exists_by_email {
            Ok(true) => {
                // Acc exists / go login

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
                    Ok(warp::reply::json(&HttpResponse::unauthorized()))
                }
            }
            Ok(false) => Ok(warp::reply::json(&HttpResponse::unauthorized())),
            Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
        };
    }

    if let Some(login) = params.login {
        let account_exists_by_login =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Login, login.clone()).await;

        return match account_exists_by_login {
            Ok(true) => {
                println!("We're here trying to get user");
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
