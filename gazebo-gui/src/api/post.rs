use crate::api::BACKEND_URL_BASE;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

pub struct GB_Post {
    pub id: u32,
    pub id_author: u32,
    pub id_parent: Option<u32>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}

pub async fn api_get_all_posts() -> Result<Vec<GB_Post>, gloo_net::Error> {
    // gloo-net
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;

    response.json::<Vec<GB_Post>>().await
}
