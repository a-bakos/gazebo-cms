use serde::Serialize;
use std::fmt::Formatter;
use warp::{http::StatusCode, reject::Reject, Rejection};

#[derive(Debug)]
pub struct SqlxError(pub sqlx::Error);
impl Reject for SqlxError {}

// Wrapper type so we have a size at compile time
#[allow(dead_code)]
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
impl ErrorResponse {
    #[allow(dead_code)]
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code, non_camel_case_types)]
pub enum GB_Error {
    //ParseError(std::num::ParseIntError),
    MissingParameters,
    OutOfBounds,
    IncorrectCredentials,
}

impl std::fmt::Display for GB_Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            // GB_Error::ParseError(ref err) => {
            //     // Why ref err?
            //     write!(f, "Cannot parse parameter {err}")
            // }
            GB_Error::MissingParameters => {
                write!(f, "Missing parameter")
            }
            GB_Error::OutOfBounds => {
                write!(f, "Out of bounds")
            }
            GB_Error::IncorrectCredentials => {
                write!(f, "Invalid credentials")
            }
        }
    }
}

#[allow(dead_code)]
pub async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(GB_Error::IncorrectCredentials) = r.find() {
        // event!(Level::ERROR, "Wrong password entered");
        Ok(warp::reply::with_status(
            "Wrong email/password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    // } else if let Some(GB_Error::Unauthorized) = r.find() {
    //     // event!(Level::ERROR, "Not matching account ID");
    //     Ok(warp::reply::with_status(
    //         "No permission to change resource".to_string(),
    //         StatusCode::UNAUTHORIZED,
    //     ))
    } else {
        // event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
