struct UUID {
    user_id: String,
    nonce: String,
    mac: String,
}

// algorithm used for signing
enum TokenSigningAlgorithm {
    HS256,
}

// token type - eg. JWT
enum TokenType {
    JWT,
}

struct TokenHeader {
    alg: TokenSigningAlgorithm,
    typ: TokenType,
}

impl TokenHeader {
    fn new(alg: TokenSigningAlgorithm, typ: TokenType) -> Self {
        Self { alg, typ }
    }
}

struct TokenClaims {
    user_id: String,
    role: String,
    uuid: String,
    nonce: String,
    expiry: i64, // timestamp of expiration
}

impl TokenClaims {
    fn new(user_id: String, role: String, uuid: String, nonce: &str, expiry: i64) -> Self {
        Self {
            user_id,
            role,
            uuid,
            nonce: nonce.to_string(),
            expiry,
        }
    }
}

struct Token {
    claims: TokenClaims,
    header: TokenHeader,
}

impl Token {
    fn generate() -> String {
        todo!()
    }
}

fn token_gen() -> String {
    let token_header = TokenHeader::new(TokenSigningAlgorithm::HS256, TokenType::JWT);
    let token_claims = TokenClaims::new(
        "100".to_string(),
        "admin".to_string(),
        "uuid".to_string(),
        "none",
        0,
    );
    let token = Token::generate();
}
