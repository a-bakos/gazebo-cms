use crate::http::status_code::HttpStatusCode;

pub struct HttpResponse;

impl HttpResponse {
    // 401 - invalid credentials error
    pub fn unauthorized() -> String {
        let status_code = HttpStatusCode::Unauthorized.code();
        let status_message = HttpStatusCode::Unauthorized.message();
        format!("{} {}", status_code, status_message)
    }
}
