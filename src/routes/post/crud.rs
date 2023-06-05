use crate::database::db::DB_Table;
use crate::entry::post::GB_Post;
use crate::errors::error_handler::SqlxError;
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use std::collections::HashMap;

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

#[allow(dead_code)]
pub fn insert_post(_pool: PgPool, _params: HashMap<String, String>) {
    // -> Result<impl warp::Reply, warp::Rejection> {
    todo!();
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
