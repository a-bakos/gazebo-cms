use gazebo_core_common::account::auth::AuthResponseAccountInfo;
use gazebo_core_common::account::auth::AuthResponsePayload;
use gazebo_core_common::account::gb_account::AccountID;
use gazebo_core_common::datetime::GB_DateTime_Variant;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: AccountID,
    pub role: String,
    pub uuid: String,
    pub exp: i64, // unix timestamp of expiration
}

impl TokenClaims {
    pub fn generate(&self) -> Option<String> {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(crate::private::JWT_SECRET.as_ref()),
        )
        .unwrap();

        if token.is_empty() {
            None
        } else {
            println!("{:?}", token);
            Some(token)
        }
    }
}

pub fn generate_session_id() -> Uuid {
    Uuid::new_v4()
}

pub fn generate_token(user_data: &AuthResponseAccountInfo, uuid: &Uuid) -> Option<String> {
    let uuid_string = uuid.to_string();
    TokenClaims {
        user_id: user_data.id.clone(),
        role: user_data.role.to_string(),
        uuid: uuid_string,
        exp: gazebo_core_common::datetime::GB_DateTime::new(GB_DateTime_Variant::NextDaySameTime)
            .timestamp,
    }
    .generate()
}
