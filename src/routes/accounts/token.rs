use crate::{
    auth::TokenClaims,
    database::{
        columns::{
            COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN, COL_INDEX_ACCOUNT_ROLE,
            COL_INDEX_ACCOUNT_UUID,
        },
        db::DB_Table,
    },
    entry::functions::get_the_author,
    routes::accounts::crud::get_user_by_id,
    traits::RowTransformer,
};

use gazebo_core_common::{
    account::{
        auth::{AuthResponseAccountInfo, AuthResponsePayload},
        gb_account::{AccountID, GB_Account},
        role::{get_role_variant, AccountRole},
    },
    status_code::HttpStatusCode,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::{uuid, Error, Uuid};

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

            let uuid_parse_str = Uuid::parse_str(&token.claims.uuid);
            let uuid_from_token = match uuid_parse_str {
                Ok(uuid) => uuid,
                Err(_) => {
                    println!("UUID NOT FOUND in token!");
                    return Ok(warp::reply::json(&false));
                }
            };

            let query = format!(
                "SELECT {}, {}, {}, {} FROM {} WHERE id = $1",
                COL_INDEX_ACCOUNT_ID,
                COL_INDEX_ACCOUNT_LOGIN,
                COL_INDEX_ACCOUNT_ROLE,
                COL_INDEX_ACCOUNT_UUID,
                DB_Table::Accounts
            );
            match sqlx::query(&query)
                .bind(user_id)
                .map(|row: PgRow| {
                    let uuid_from_db: Uuid = row.get(COL_INDEX_ACCOUNT_UUID);
                    // Check if UUIDs match
                    return if uuid_from_db == uuid_from_token {
                        AuthResponsePayload {
                            http_status_code: HttpStatusCode::Ok.code(),
                            account_details: AuthResponseAccountInfo {
                                id: AccountID(row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32),
                                role: {
                                    let role: String = row.get(COL_INDEX_ACCOUNT_ROLE);
                                    let role: AccountRole = get_role_variant(&role);
                                    role
                                },
                                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                            },
                            token: Some(params.token.clone()),
                        }
                    } else {
                        AuthResponsePayload {
                            http_status_code: HttpStatusCode::Unauthorized.code(),
                            account_details: AuthResponseAccountInfo::default(),
                            token: None,
                        }
                    };
                })
                .fetch_one(&pool)
                .await
            {
                Ok(res) => {
                    println!("{:?}", res);
                    println!("Auth successful");
                    Ok(warp::reply::json(&res))
                }
                Err(e) => {
                    println!("Error user not found: {}", e);
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
