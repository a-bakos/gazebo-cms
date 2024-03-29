use crate::{
    database::{
        columns::{
            COL_INDEX_POST_CONTENT, COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID,
            COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PASSWORD, COL_INDEX_POST_SLUG,
            COL_INDEX_POST_STATUS, COL_INDEX_POST_TITLE, COL_INDEX_POST_TYPE,
        },
        consts::DB_TABLE_POSTS,
        db::DB_Table,
    },
    errors::error_handler::SqlxError,
    traits::RowTransformer,
};
use gazebo_core_common::entry::gb_post::GB_Post;

use gazebo_core_common::entry::entry_id::EntryID;
use gazebo_core_common::entry::gb_entry::{
    GB_EntryInsertRequest, GB_EntryInsertResponse, GB_EntryUpdateRequest, GB_EntryUpdateResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn get_post_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Post requested: {:?}", id);

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Posts);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| GB_Post::transform(&row))
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))), // Unhandled rejection: SqlxError(RowNotFound)
    }
}

pub async fn get_posts(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("All posts requested");
    let mut posts: Vec<GB_Post> = Vec::new();
    let query = format!("SELECT * FROM {}", DB_Table::Posts);
    match sqlx::query(&query)
        .map(|row: PgRow| {
            let gb_post = GB_Post::transform(&row);
            posts.push(gb_post);
        })
        .fetch_all(&pool)
        .await
    {
        Ok(_res) => Ok(warp::reply::json(&posts)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))), // Unhandled rejection: SqlxError(RowNotFound)
    }
}

pub async fn insert_post(
    pool: PgPool,
    params: GB_EntryInsertRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // auth layer to check if accounts can add post

    // checks needed for these values
    // - check author id exists
    // - check slug is unique and treat it if not
    // - check status is valid variant
    let author_id = params.author_id;
    let slug = params.slug.clone();
    let status = params.status.clone();
    let title = params.title.clone();
    let content = params.content.clone();

    // Don't unwrap the optional values
    let password = params.password.clone();
    let excerpt = params.excerpt.clone();

    let query = format!(
        "INSERT INTO {} ({}, {}, {}, {}, {}, {}, {}) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        DB_Table::Posts,
        COL_INDEX_POST_ID_AUTHOR,
        COL_INDEX_POST_SLUG,
        COL_INDEX_POST_TITLE,
        COL_INDEX_POST_EXCERPT,
        COL_INDEX_POST_CONTENT,
        COL_INDEX_POST_PASSWORD,
        COL_INDEX_POST_STATUS
    );

    match sqlx::query(&query)
        .bind(author_id)
        .bind(slug.clone())
        .bind(title.clone())
        .bind(excerpt)
        .bind(content)
        .bind(password)
        .bind(status)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => {
            let post_id = result.get::<i32, _>(COL_INDEX_POST_ID) as u32;
            Ok(warp::reply::json(&GB_EntryInsertResponse {
                http_status_code: 200,
                entry_id: EntryID(post_id),
            }))
        }
        Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
    }
}

// todo - common data structure will be used
#[derive(Deserialize, Serialize)]
pub struct UpdateEntryParams {
    pub to_update: String,
    pub value: String,
}

fn get_table_and_column_name(_params: &UpdateEntryParams) -> (String, String) {
    (
        DB_Table::Posts.to_string(),
        COL_INDEX_POST_STATUS.to_string(),
    )
}

pub async fn update_entry_single_param(
    id: i32,
    pool: PgPool,
    params: UpdateEntryParams,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (table, column) = get_table_and_column_name(&params);
    let query = format!(
        "UPDATE {} SET {} = $1 WHERE id = $2",
        table.clone(),
        column.clone()
    );
    match sqlx::query(&query)
        .bind(params.value.clone())
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Ok(warp::reply::with_status(false.to_string(), StatusCode::OK));
            }
            println!(
                "Entry updated! Type: {}, ID: {}, Param: {}, Value: {}",
                table, id, column, params.value
            );
            Ok(warp::reply::with_status(true.to_string(), StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

pub async fn update_entry(
    id: i32,
    pool: PgPool,
    params: GB_EntryUpdateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // todo -- currently only one item gets updated
    //let (table, column) = get_table_and_column_name(&params);
    let query = format!(
        "UPDATE {} SET {} = $1 WHERE id = $2",
        DB_Table::Posts.to_string(),
        COL_INDEX_POST_TITLE.to_string()
    );
    match sqlx::query(&query)
        .bind(params.title.clone())
        .bind(id.clone())
        .execute(&pool)
        .await
    {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Ok(warp::reply::json(&GB_EntryUpdateResponse {
                    http_status_code: 403, // todo find better status code
                    entry_id: EntryID(id as u32),
                }));
            }
            println!("Entry updated!");
            Ok(warp::reply::json(&GB_EntryUpdateResponse {
                http_status_code: 200,
                entry_id: EntryID(id as u32),
            }))
        }
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

pub async fn delete_post(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Post to be deleted ID: {}", id);

    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let query = format!("DELETE FROM {} WHERE id = $1", DB_Table::Posts);
    match sqlx::query(&query).bind(id).execute(&pool).await {
        Ok(res) => {
            if res.rows_affected() == 0 {
                println!("Unknown post ID to be deleted");
                return Ok(warp::reply::with_status(
                    String::from("Unknown post ID"),
                    StatusCode::OK,
                ));
            }
            println!("Post deleted!");
            Ok(warp::reply::with_status(
                format!("Post {} deleted", id),
                StatusCode::OK,
            ))
        }
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

// Get post title
pub(crate) async fn get_the_title(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Title requested of post: {:?}", id);

    let query = format!(
        "SELECT {} FROM {} WHERE id = $1",
        COL_INDEX_POST_TITLE,
        DB_Table::Posts
    );
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| {
            let title: String = row.get(COL_INDEX_POST_TITLE);
            title
        })
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))), // Unhandled rejection: SqlxError(RowNotFound)
    }
}
