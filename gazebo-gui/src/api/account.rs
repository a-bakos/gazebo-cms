use crate::api::{HttpStatusCode, BACKEND_URL_BASE};
use gazebo_core_common::account::{
    gb_account::{AccountID, GB_Account},
    role::AccountRole,
};
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub(crate) struct LoginResponseWithStatusCode {
    pub(crate) http_status_code: HttpStatusCode,
    pub(crate) account_details: LoginResponseAccountDetails,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LoginResponseAccountDetails {
    pub id: u32,
    pub login_name: String,
    pub email: String,
    pub role: String,
}

pub(crate) async fn api_login_request(
    username: String,
    password: String,
) -> Result<LoginResponseWithStatusCode, gloo_net::Error> {
    let response = Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": username,
            "password": password
        }))?
        .send()
        .await?;

    response.json::<LoginResponseWithStatusCode>().await
}

pub(crate) async fn api_get_all_accounts() -> Result<Vec<GB_Account>, gloo_net::Error> {
    let response = Request::get(&format!("{}/accounts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Account>>().await
}
