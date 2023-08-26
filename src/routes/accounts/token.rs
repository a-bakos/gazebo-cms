use crate::auth::TokenClaims;
use crate::routes::accounts::login::LoginResponseAccountDetails;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenAuthParams {
    pub token: String,
}

pub async fn auth(
    pool: PgPool,
    params: TokenAuthParams,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Think about UUID
    // and bearer tokens

    // decode params.token to get uuid
    // check uuid if matches construct response

    let token_encoded = params.token.clone();
    // Claims is a struct that implements Deserialize
    let token_decoded = decode::<TokenClaims>(
        &token_encoded,
        &DecodingKey::from_secret(crate::private::JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_decoded {
        Ok(token) => {
            // todo do something with claims
            println!("{:?}", token.claims);
        }
        Err(_) => {}
    }

    // TODO get info from DB

    /*
        let update_query = format!(
            "UPDATE {} SET last_login = CURRENT_TIMESTAMP, uuid = $1 WHERE {} = $2",
            DB_Table::Accounts,
            get_column_name_by_login_variant(login_variant) // email or username
        );
        let uuid: uuid::Uuid = crate::auth::generate_session_id();
        match sqlx::query(&update_query)
            .bind(uuid.clone())
            .bind(value)
            .execute(&pool)
            .await
        {
            Ok(_) => {
                println!("Last login datetime + UUID updated!");
                Some(uuid)
            }
            Err(e) => {
                println!("Last login datetime update or UUID error! {:?}", e);
                None
            }
        }
    */
    let response = LoginResponseAccountDetails {
        id: 9000,
        login_name: "THELOGINNAME".to_string(),
        email: "".to_string(),
        role: "admin".to_string(),
        token: params.token,
    };
    Ok(warp::reply::json(&response))
}
