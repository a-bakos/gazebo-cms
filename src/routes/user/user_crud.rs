use crate::database::{
    columns::{
        COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_LOGIN,
        COL_INDEX_ACCOUNT_PASSWORD, COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
    },
    db::DB_Table,
};
use crate::errors::error_handler::SqlxError;
use crate::users::{
    roles::get_role_variant,
    user::{User, UserID},
};
use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, PgPool, Row};
use warp::http::StatusCode;

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn get_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("User requested by ID: {}", id);

    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let query = format!("SELECT * FROM {} WHERE id = $1", DB_Table::Accounts);
    match sqlx::query(&query)
        .bind(id)
        .map(|row: PgRow| {
            let user_id = row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;
            let user_role = row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE);
            let user_role = get_role_variant(user_role);

            // Don't need to specify a default/fallback value because the cell will never be empty
            let registered: NaiveDateTime =
                row.get::<NaiveDateTime, _>(COL_INDEX_ACCOUNT_REGISTERED);

            User {
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                id: UserID(user_id),
                role: user_role,
                password: row.get(COL_INDEX_ACCOUNT_PASSWORD), // todo: hide this later
                registered: registered.to_string(),
            }
        })
        .fetch_one(&pool)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}

// http://localhost:1337/user/{id}
// when ID does not exist: Unhandled rejection: SqlxError(RowNotFound)
pub async fn delete_user_by_id(id: i32, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("User to be deleted ID: {}", id);

    // TODO Authentication layer needs to be here.
    // if ! auth { return Err(warp::reject::custom(AuthError)), }

    let query = format!("DELETE FROM {} WHERE id = $1", DB_Table::Accounts);
    match sqlx::query(&query).bind(id).execute(&pool).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("User {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(SqlxError(e))),
    }
}
