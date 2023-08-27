use crate::auth::TokenClaims;
use crate::database::db::DB_Table;
use crate::routes::accounts::crud::get_user_by_id;
use crate::traits::RowTransformer;
use gazebo_core_common::account::gb_account::GB_Account;
use gazebo_core_common::account::login::LoginResponseAccountDetails;
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
            // - check uuid is valid
            // - check role

            // Single-step verification vs Two-step verification
            //
            // Two-step verification involves first finding the user by their ID, verifying the UUID,
            // and then making an additional call to the database to retrieve all required user information.
            //
            // Single-step verification retrieves all user information in a single step and then
            // proceeds to verify the UUID. I prefer this approach here for better performance,
            // as it involves only one database call. Additionally, since we're not including highly sensitive
            // data in the token's payload, even if the token is compromised, the potential harm is limited.

            let user_id: i32 = token.claims.user_id.into();
            let uuid = token.claims.uuid;

            let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Accounts);
            match sqlx::query(&query)
                .bind(user_id)
                .map(|row: PgRow| GB_Account::transform(&row))
                .fetch_one(&pool)
                .await
            {
                Ok(res) => {
                    println!("{:?}", res);

                    // TODO think about this response structure (maybe a more generic ResponsePayload would be better)
                    // Also, we may not want to reconstruct the response, but use the existing token payload
                    let response = LoginResponseAccountDetails {
                        id: token.claims.user_id,
                        login_name: res.login_name.to_string(),
                        email: res.email.to_string(),
                        role: res.role.into(),
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
        Err(_) => {
            println!("TOKEN DECODING ERROR");
            Ok(warp::reply::json(&false))
        }
    }
}
