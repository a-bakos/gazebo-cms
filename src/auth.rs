use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;

// algorithm used for signing
pub enum TokenSigningAlgorithm {
    HS256,
}

// token type - eg. JWT
pub enum TokenType {
    JWT,
}

pub struct TokenHeader {
    pub alg: TokenSigningAlgorithm,
    pub typ: TokenType,
}

impl TokenHeader {
    pub fn new(alg: TokenSigningAlgorithm, typ: TokenType) -> Self {
        Self { alg, typ }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,
    pub role: String,
    pub uuid: String,
    pub nonce: String,
    pub expiry: i64, // timestamp of expiration
}

impl TokenClaims {
    pub fn new(user_id: String, role: String, uuid: String, nonce: &str, expiry: i64) -> Self {
        Self {
            user_id,
            role,
            uuid,
            nonce: nonce.to_string(),
            expiry,
        }
    }
}

pub struct Token {
    pub claims: TokenClaims,
    pub header: TokenHeader,
}

impl Token {
    pub fn generate(&self) -> String {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &self.claims,
            &EncodingKey::from_secret(crate::private::JWT_SECRET.as_ref()),
        )
        .unwrap();

        if token.is_empty() {
            "NONE".to_string()
        } else {
            token
        }
    }
}

pub fn generate_session_id() -> Uuid {
    Uuid::new_v4()
}

pub fn token_gen() -> String {
    let uuid = generate_session_id();
    let uuid_string = uuid.to_string();
    let token_header = TokenHeader::new(TokenSigningAlgorithm::HS256, TokenType::JWT);
    let token_claims = TokenClaims::new(
        "100".to_string(),
        "admin".to_string(),
        uuid_string,
        "none",
        0,
    );

    Token {
        claims: token_claims,
        header: token_header,
    }
    .generate()
}
