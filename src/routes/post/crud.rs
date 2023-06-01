use crate::database::columns::{
    COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PARENT,
};
use crate::database::db::DB_Table;
use crate::errors::error_handler::SqlxError;
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, GB_Post, PostStatus};
use crate::users::user::UserID;
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
            let post_id = row.get::<i32, _>(COL_INDEX_POST_ID) as u32;
            let author_id = row.get::<i32, _>(COL_INDEX_POST_ID_AUTHOR) as u32;

            //parent_id
            //author_id
            //date_published
            //date_modified
            //slug
            //post_type
            //title
            //excerpt
            //content

            GB_Post {
                id: EntryID(post_id),
                id_author: UserID(author_id),
                id_parent: None,
                date_publish: "".to_string(),
                date_modified: "".to_string(),
                slug: None,
                the_type: EntryType::Post,
                status: PostStatus::Draft,
                title: None,
                excerpt: None,
                content: None,
                password: None,
            }

            // Don't need to specify a default/fallback value because the cell will never be empty
            //let registered: NaiveDateTime =
            //row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);

            // User {
            //     login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
            //     email: row.get(COL_INDEX_ACCOUNT_EMAIL),
            //     id: UserID(user_id),
            //     role: user_role,
            //     password: row.get(COL_INDEX_ACCOUNT_PASSWORD), // todo: hide this later
            //     registered: registered.to_string(),
            // }
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
