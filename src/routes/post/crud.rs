use crate::database::db::DB_Table;
use crate::entry::post::GB_PostItem;
use crate::errors::error_handler::SqlxError;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

pub async fn get_post_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Post requested: {:?}", id);

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Posts);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| {
            let the_post: GB_PostItem = row.into();
            the_post
        })
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))), // Unhandled rejection: SqlxError(RowNotFound)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPostInsertRequest {
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub excerpt: Option<String>,
    pub password: Option<String>,
}

pub async fn insert_post(
    pool: PgPool,
    params: NewPostInsertRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // auth layer to check if user can add post

    // checks needed for these values
    // - check author id exists
    // - check slug is unique and treat it if not
    // - check status is valid variant
    let author_id = params.author_id;
    let slug = params.slug.clone();
    let status = params.status.clone();

    let post_type = "post".to_string();
    let title = params.title.clone();
    let content = params.content.clone();

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
        .bind(title.clone())
        .bind(content)
        .bind(post_type)
        .bind(slug.clone())
        .bind(status)
        .bind(excerpt)
        .bind(password)
        .execute(&pool)
        .await
    {
        Ok(_) => {
            let select_query = format!(
                "SELECT id FROM {} WHERE title = $1 AND slug = $2",
                DB_Table::Posts
            );
            // Retrieve the post_id with a subsequent select query
            match sqlx::query(&select_query)
                .bind(title.clone())
                .bind(slug.clone())
                .map(|row: PgRow| row.get::<i32, _>("id"))
                .fetch_one(&pool)
                .await
            {
                Ok(id) => Ok(warp::reply::json(&format!("ID: {}", id))),
                Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
            }
        }
        Err(e) => Ok(warp::reply::json(&format!("Error: {}", e))),
    }
}

#[allow(dead_code)]
pub fn update_post(_pool: PgPool, _params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!(); // w/ card
}

#[allow(dead_code)]
pub fn delete_post(_pool: PgPool, _params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!(); // w/ card
}
