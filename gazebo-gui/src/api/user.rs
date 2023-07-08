use crate::api::BACKEND_URL_BASE;

use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

#[derive(PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    // pub created_at: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub id: u32,
    pub name: String,
}

pub async fn api_login(
    username: String,
    password: String,
) -> Result<(u16, LoginResponse), gloo_net::Error> {
    // gloo-net
    let response = Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": username,
            "password": password
        }))?
        .send()
        .await?;

    // u16 = HttpStatusCode
    // LoginResponse = currently: TryThis from backend
    response.json::<(u16, LoginResponse)>().await
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
