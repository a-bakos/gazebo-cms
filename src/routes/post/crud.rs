use crate::consts;
use crate::database::columns::{
    COL_INDEX_POST_CONTENT, COL_INDEX_POST_DATE_MODIFIED, COL_INDEX_POST_DATE_PUBLISH,
    COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PARENT,
    COL_INDEX_POST_SLUG, COL_INDEX_POST_STATUS, COL_INDEX_POST_TITLE, COL_INDEX_POST_TYPE,
};
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
            // Underscores' meaning here:
            // we don't need to specify a default/fallback value because the cell will never be empty

            let post_id = row.get::<i32, _>(COL_INDEX_POST_ID) as u32;
            let author_id = row.get::<i32, _>(COL_INDEX_POST_ID_AUTHOR) as u32;
            let parent_id = row
                .try_get(COL_INDEX_POST_PARENT)
                .ok()
                .unwrap_or(consts::ENTRY_ID_NO_PARENT) as u32;

            let entry_type_as_str: &str = row.get(COL_INDEX_POST_TYPE);
            let the_entry_type: EntryType = get_entry_type_variant(entry_type_as_str);

            let date_published: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_PUBLISH);
            let date_modified: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_MODIFIED);

            let entry_status_as_str: &str = row.get(COL_INDEX_POST_STATUS);
            let the_post_status: EntryStatus =
                get_entry_status_variant(entry_status_as_str, &the_entry_type);

            GB_Post {
                id: EntryID(post_id),
                id_author: UserID(author_id),
                id_parent: Some(EntryID(parent_id)),
                date_publish: date_published.to_string(),
                date_modified: date_modified.to_string(),
                slug: row.get(COL_INDEX_POST_SLUG),
                the_type: the_entry_type,
                status: the_post_status,
                title: row.get(COL_INDEX_POST_TITLE),
                excerpt: row.get(COL_INDEX_POST_EXCERPT),
                content: row.get(COL_INDEX_POST_CONTENT),
                password: None,
            }
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
