use crate::api::{HttpStatusCode, BACKEND_URL_BASE};
use gazebo_core_common::account::gb_account::AccountID;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

use gazebo_core_common::entry::{
    entry_id::EntryID, entry_type::EntryType, gb_post::GB_Post, status::EntryStatus,
};

pub async fn api_get_all_posts() -> Result<Vec<GB_Post>, gloo_net::Error> {
    // gloo-net
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Post>>().await
}

pub async fn api_delete_entry_by_id(
    entry_id: u32,
) -> Result<ResponseWithStatusCode, gloo_net::Error> {
    let response = Request::delete(&format!("{}/post/{}", BACKEND_URL_BASE, entry_id))
        .send()
        .await?;
    // todo match on response and turn it into bool!
    gloo_console::log!("{}", response.body());
    response.json::<ResponseWithStatusCode>().await
}

#[derive(Deserialize)]
pub struct ResponseWithStatusCode {
    pub http_status_code: HttpStatusCode,
    pub details: String,
}

// TODO WIP

#[derive(Serialize)]
pub enum EntryUpdateType {
    Status,
}

#[derive(Serialize)]
pub struct EntryUpdateProps<'a> {
    pub to_update: &'a str,
    //EntryUpdateType,
    pub value: &'a str,
}

/// Update a single entry parameter
pub async fn update_entry_single_param<'a>(
    entry_type: EntryType,
    entry_id: EntryID,
    update_props: EntryUpdateProps<'a>,
) -> Result<String, gloo_net::Error> {
    let response = Request::put(&format!(
        "{}/{}/{}",
        BACKEND_URL_BASE,
        entry_type.to_string(),
        entry_id
    ))
    .json(&json!(update_props))?
    .send()
    .await?;
    response.text().await
}
