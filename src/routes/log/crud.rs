use crate::database::db::DB_Table;
use crate::errors::error_handler::SqlxError;
use gazebo_core_common::entry::gb_log::GB_Log;
use sqlx::postgres::PgRow;
use sqlx::PgPool;

pub async fn get_event_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Event requested: {}", id);

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Log);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| /*GB_Log::transform(&row)*/ "HELLO".to_string())
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

// get events
// insert event
// delete event
