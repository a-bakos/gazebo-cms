use crate::api::BACKEND_URL_BASE;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

//#[derive(Deserialize)]
//pub struct GetAllPostsResponse {
//    pub all_posts: Vec<GB_PostItem>,
//}

pub async fn api_get_all_posts() -> Result<Vec<String>, gloo_net::Error> {
    // gloo-net
    let response = Request::get(&format!("{}/posts", BACKEND_URL_BASE))
        .send()
        .await?;
    response.json::<Vec<String>>().await
}
