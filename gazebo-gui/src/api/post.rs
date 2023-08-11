use crate::api::{BACKEND_URL_BASE, HttpStatusCode};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

// todo - will be added to common lib
#[allow(non_camel_case_types)]
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GB_Post {
    pub id: u32,
    pub id_author: u32,
    pub id_parent: Option<u32>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: EntryStatus, // todo
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}
// todo - will be added to common lib
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EntryStatus {
    Post(ContentStatus),
    Media(MediaStatus),
    Unknown,
}
// todo - will be added to common lib
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ContentStatus {
    Draft,
    Publish,
    Private,
    Trash,
    Unknown,
    // Future
    // Pending
}
// todo - will be added to common lib
impl From<String> for ContentStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "draft" => ContentStatus::Draft,
            "publish" => ContentStatus::Publish,
            "private" => ContentStatus::Private,
            "trash" => ContentStatus::Trash,
            _ => ContentStatus::Unknown,
        }
    }
}
// todo - will be added to common lib
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MediaStatus {
    Attached,
    Unattached,
    Unknown,
}

pub async fn api_get_all_posts() -> Result<Vec<GB_Post>, gloo_net::Error> {
    // gloo-net
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<GB_Post>>().await
}

pub async fn api_delete_entry_by_id(entry_id: u32) -> Result<ResponseWithStatusCode, gloo_net::Error> {
    let response = Request::delete(&format!("{}/post/{}", BACKEND_URL_BASE, entry_id)).send().await?;
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
    Status
}
#[derive(Serialize)]
pub struct EntryUpdateProps<'a> {
    pub to_update: &'a str,//EntryUpdateType,
    pub value: &'a str,
}
pub async fn update_entry_single_param<'a>(entry_id: u32, update_props: EntryUpdateProps<'a>) -> Result<String, gloo_net::Error> {
    let response = Request::put(&format!("{}/post/{}", BACKEND_URL_BASE, entry_id))
        .json(&json!({
            "update": update_props.to_update,
            "value": update_props.value
        }))?.send().await?;
    response.text().await
}