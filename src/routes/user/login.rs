use crate::database::columns::{COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_LOGIN};
use crate::database::db::DB_Table;
use crate::errors::error_handler::ErrorResponse;
use crate::http::response::HttpResponse;
use crate::users::user_manager;
use crate::users::user_manager::check_account_exists;
use crate::users::user_manager::CheckAccountExistsBy;
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
    pool: PgPool,
    password: String,
    binding: String,
    login_variant: CheckAccountExistsBy,
) -> Result<warp::reply::Json, warp::Rejection> {
    match sqlx::query(query)
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

            let column_name = match login_variant {
                CheckAccountExistsBy::Email => COL_INDEX_ACCOUNT_EMAIL,
                CheckAccountExistsBy::Login => COL_INDEX_ACCOUNT_LOGIN,
            };
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

            Ok(warp::reply::json(&MSG_LOGIN_SUCCESS))
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
            check_account_exists(pool.clone(), CheckAccountExistsBy::Email, email.clone()).await;

        return match account_exists_by_email {
            Ok(true) => {
                // Acc exists / go login
                let query = format!("SELECT * FROM {} WHERE email = $1", DB_Table::Accounts); // this might go into try_login
                let binding = email.clone();
                let password = params.password.clone();
                if user_manager::is_password_match(
                    &pool,
                    &password,
                    CheckAccountExistsBy::Email,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(
                        &query,
                        pool.clone(),
                        password,
                        binding,
                        CheckAccountExistsBy::Email,
                    )
                    .await
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
        let _query = format!("SELECT * FROM {} WHERE login = $1", DB_Table::Accounts);
        let _binding = login.clone();
        let account_exists_by_login =
            check_account_exists(pool.clone(), CheckAccountExistsBy::Login, login.clone()).await;

        return match account_exists_by_login {
            Ok(true) => {
                println!("We're here trying to get user");
                // Acc exists
                let query = format!("SELECT * FROM {} WHERE login = $1", DB_Table::Accounts);
                let binding = login.clone();
                let password = params.password.clone();
                if user_manager::is_password_match(
                    &pool,
                    &password,
                    CheckAccountExistsBy::Login,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(
                        &query,
                        pool.clone(),
                        password,
                        binding,
                        CheckAccountExistsBy::Login,
                    )
                    .await
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
