use crate::api::{HttpStatusCode, BACKEND_URL_BASE};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginResponseWithStatusCode(pub HttpStatusCode, pub LoginResponse);

#[derive(PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub id: u32,
    pub login_name: String,
    pub email: String,
    pub role: String,
}

pub async fn api_login_request(
    username: String,
    password: String,
) -> Result<LoginResponseWithStatusCode, gloo_net::Error> {
    let response = gloo_net::http::Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": username,
            "password": password
        }))?
        .send()
        .await?;

    response.json::<LoginResponseWithStatusCode>().await
}
