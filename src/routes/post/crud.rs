use crate::database::db::DB_Table;
use crate::entry::post::GB_Post;
use crate::errors::error_handler::SqlxError;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn get_post_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Post requested: {:?}", id);

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Posts);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| {
            let the_post: GB_Post = row.into();
            the_post
        })
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPostInsertRequest {
    pub author_id: i32,
    pub slug: String,
    pub the_type: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub excerpt: Option<String>,
    pub password: Option<String>,
}

#[allow(dead_code)]
pub async fn insert_post(
    pool: PgPool,
    params: NewPostInsertRequest, //HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // auth layer to check if user can add post

    let author_id = params.author_id;
    let slug = params.slug.clone();
    let post_type = params.the_type.clone();
    let title = params.title.clone();
    let content = params.content.clone();
    let status = params.status.clone();

    // Don't unwrap the optional values
    let password = params.password.clone();
    let excerpt = params.excerpt.clone();

    let query = format!(
        "INSERT INTO {} ({}, {}, {}, {}, {}, {}, {}, {}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        DB_Table::Posts,
        crate::database::columns::COL_INDEX_POST_ID_AUTHOR,
        crate::database::columns::COL_INDEX_POST_TITLE,
        crate::database::columns::COL_INDEX_POST_CONTENT,
        crate::database::columns::COL_INDEX_POST_TYPE,
        crate::database::columns::COL_INDEX_POST_SLUG,
        crate::database::columns::COL_INDEX_POST_STATUS,
        crate::database::columns::COL_INDEX_POST_EXCERPT,
        crate::database::columns::COL_INDEX_POST_PASSWORD
    );
    match sqlx::query(&query)
        .bind(author_id)
        .bind(title)
        .bind(content)
        .bind(post_type)
        .bind(slug)
        .bind(status)
        .bind(excerpt)
        .bind(password)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(warp::reply::json(&"Post Created!")),
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }

    // Ok(true) => Ok(warp::reply::json(&"Email address already in use")),
    // Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
}

#[allow(dead_code)]
pub fn update_post(_pool: PgPool, _params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
}

#[allow(dead_code)]
pub fn delete_post(_pool: PgPool, _params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
}
