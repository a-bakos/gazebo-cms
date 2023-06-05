use crate::database::db::DB_Table;
use crate::entry::entry_type::{get_entry_type_variant, EntryType};
use crate::entry::functions::get_post_type;
use crate::entry::post::{EntryID, GB_Post};
use crate::entry::status::{get_entry_status_variant, EntryStatus, PostStatus};
use crate::errors::error_handler::SqlxError;
use crate::users::user::UserID;
use chrono::NaiveDateTime;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use warp::body::content_length_limit;
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

pub fn insert_post(pool: PgPool, params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
}

pub fn update_post(pool: PgPool, params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
}

pub fn delete_post(pool: PgPool, params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
}
