use crate::api::BACKEND_URL_BASE;
use gazebo_core_common::{
    account::{
        auth::AuthResponsePayload,
        gb_account::{AccountID, GB_Account},
        role::AccountRole,
    },
    status_code::HttpStatusCode,
};
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

pub(crate) async fn api_login_request(
    username: String,
    password: String,
) -> Result<AuthResponsePayload, gloo_net::Error> {
    let response = Request::post(&format!("{}/login", BACKEND_URL_BASE))
        .json(&json!({
            "login": username,
            "password": password
        }))?
        .send()
        .await?;

    response.json::<AuthResponsePayload>().await
}

pub(crate) async fn api_get_all_accounts() -> Result<Vec<GB_Account>, gloo_net::Error> {
    let response = Request::get(&format!("{}/accounts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Account>>().await
}

pub(crate) async fn api_auth_me(token: &str) -> Result<AuthResponsePayload, gloo_net::Error> {
    let response = Request::post(&format!("{}/auth", BACKEND_URL_BASE))
        .json(&json!({
            "token": token,
        }))?
        .send()
        .await?;
    response.json::<AuthResponsePayload>().await
}
