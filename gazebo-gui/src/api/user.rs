use gloo_net::http::Request;

use crate::api::BACKEND_URL_BASE;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn api_login(
    username: String,
    password: String,
) -> Result<LoginResponse, gloo_net::Error> {
    // gloo-net
    let response = Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": username,
            "password": password
        }))?
        .send()
        .await?;

    response.json::<LoginResponse>().await
}

#[derive(Deserialize)]
pub struct MeResponse {
    pub id: i32,
    pub username: String,
    pub created_at: String,
}

pub async fn api_me() -> Result<MeResponse, gloo_net::Error> {
    let response = Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": "username",
            "password": "password"
        }))?
        .send()
        .await?;

    response.json::<MeResponse>().await
}
