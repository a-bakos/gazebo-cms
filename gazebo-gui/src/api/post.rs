use crate::api::BACKEND_URL_BASE;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EntryStatus {
    Post(ContentStatus),
    Media(MediaStatus),
    Unknown,
}

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MediaStatus {
    Attached,
    Unattached,
    Unknown,
}

pub async fn api_get_all_posts() -> Result<Vec<GB_Post>, gloo_net::Error> {
    // gloo-net

    // todo : turn response into Vec<GB_Post>
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;

    response.json::<Vec<GB_Post>>().await
}
