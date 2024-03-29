use crate::api::{HttpStatusCode, BACKEND_URL_BASE};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

use gazebo_core_common::entry::gb_entry::{
    GB_EntryInsertRequest, GB_EntryInsertResponse, GB_EntryUpdateRequest, GB_EntryUpdateResponse,
};
use gazebo_core_common::{
    account::gb_account::AccountID,
    entry::{entry_id::EntryID, entry_type::EntryType, gb_post::GB_Post, status::EntryStatus},
};

pub(crate) async fn api_get_all_posts() -> Result<Vec<GB_Post>, gloo_net::Error> {
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Post>>().await
}

pub(crate) async fn api_get_single_post(
    entry_type: String,
    entry_id: String,
) -> Result<GB_Post, gloo_net::Error> {
    let response = Request::get(&format!("{}/{}/{}", BACKEND_URL_BASE, entry_type, entry_id))
        .send()
        .await?;
    response.json::<GB_Post>().await
}

pub(crate) async fn api_delete_entry_by_id(
    entry_id: u32,
) -> Result<ResponseWithStatusCode, gloo_net::Error> {
    let response = Request::delete(&format!("{}/post/{}", BACKEND_URL_BASE, entry_id))
        .send()
        .await?;
    gloo_console::log!("{}", response.body());
    response.json::<ResponseWithStatusCode>().await
}

#[derive(Deserialize)]
pub(crate) struct ResponseWithStatusCode {
    pub(crate) http_status_code: HttpStatusCode,
    pub(crate) details: String,
}

// TODO WIP

#[derive(Serialize)]
pub enum EntryUpdateType {
    Status,
}

#[derive(Serialize)]
pub(crate) struct EntryUpdateProps<'a> {
    pub(crate) to_update: &'a str,
    //EntryUpdateType,
    pub(crate) value: &'a str,
}

/// Update a single entry parameter
pub(crate) async fn update_entry_single_param<'a>(
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

pub(crate) async fn update_entry_properties<'a>(
    entry_type: EntryType,
    entry_id: EntryID,
    update_props: EntryUpdateProps<'a>,
) -> Result<String, gloo_net::Error> {
    todo!()
}

pub(crate) async fn api_new_entry_insert_request(
    entry_data: GB_EntryInsertRequest,
) -> Result<GB_EntryInsertResponse, gloo_net::Error> {
    let response = Request::post(&format!("{}/{}/{}", BACKEND_URL_BASE, "post", "add"))
        .json(&json!(entry_data))?
        .send()
        .await?;
    response.json::<GB_EntryInsertResponse>().await
}

pub(crate) async fn api_entry_update_request(
    entry_data: GB_EntryUpdateRequest,
) -> Result<GB_EntryUpdateResponse, gloo_net::Error> {
    let entry_id = entry_data.entry_id.0 as i32;
    let response = Request::put(&format!("{}/{}/{}", BACKEND_URL_BASE, "post", entry_id))
        .json(&json!(entry_data))?
        .send()
        .await?;
    response.json::<GB_EntryUpdateResponse>().await
}
