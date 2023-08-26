use crate::auth::TokenClaims;
use crate::database::db::DB_Table;
use crate::routes::accounts::crud::get_user_by_id;
use crate::routes::accounts::login::LoginResponseAccountDetails;
use crate::traits::RowTransformer;
use gazebo_core_common::account::gb_account::GB_Account;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenAuthParams {
    pub token: String,
}

pub async fn auth(
    pool: PgPool,
    params: TokenAuthParams,
) -> Result<impl warp::Reply, warp::Rejection> {
    match decode::<TokenClaims>(
        &params.token.clone(), // decoded token
        &DecodingKey::from_secret(crate::private::JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => {
            println!("{:?}", token.claims);

            // TODO get info from DB
            // - get user by id
            // - check uuid is valid
            // - check role

            let user_id = token.claims.user_id;
            let uuid = token.claims.uuid;

            // TODO WIP HERE!
            // ERR: error returned from database: operator does not exist: integer = text
            let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Accounts);
            match sqlx::query(&query)
                .bind(user_id.clone())
                .map(|row: PgRow| GB_Account::transform(&row))
                .fetch_one(&pool)
                .await
            {
                Ok(res) => {
                    let response = LoginResponseAccountDetails {
                        id: res.id.0,
                        login_name: res.login_name.to_string(),
                        email: res.email.to_string(),
                        role: res.role.to_string(),
                        token: params.token,
                    };
                    println!("Auth successful");
                    Ok(warp::reply::json(&response))
                }
                Err(e) => {
                    println!("Error: {}", e);
                    Ok(warp::reply::json(&false))
                }
            }
        }
        Err(_) => Ok(warp::reply::json(&false)),
    }
}
