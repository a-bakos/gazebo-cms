use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountRegistrationRequest {
    pub email: String,
    pub password: String,
}

pub async fn registration(
    pool: PgPool,
    params: NewAccountRegistrationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    Ok(warp::reply::with_status(
        "Registration successful",
        warp::http::StatusCode::OK,
    ))
}
