// todo: rename this module to account

use crate::api::{HttpStatusCode, BACKEND_URL_BASE};
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginResponseWithStatusCode {
    pub http_status_code: HttpStatusCode,
    pub account_details: LoginResponseAccountDetails,
}

#[derive(PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LoginResponseAccountDetails {
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

//////////////////

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AccountRole {
    Admin,       // read, write, delete, add ??
    Editor,      // read, write, delete
    Contributor, // read
    NotFound,    // missing or incorrect role
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GB_Account {
    pub login_name: String,
    pub email: String,
    pub id: u32, //UserID,
    pub role: AccountRole,
    pub password: String,
    pub registered: String,
    pub last_login: Option<String>,
}

pub async fn api_get_all_accounts() -> Result<Vec<GB_Account>, gloo_net::Error> {
    let response = Request::get(&format!("{}/accounts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Account>>().await
}
