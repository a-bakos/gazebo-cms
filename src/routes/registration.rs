use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub email: String,
    pub password: String,
}
