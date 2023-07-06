use crate::http::status_code::HttpStatusCode;

pub struct HttpResponse;

impl HttpResponse {
    // 401 - invalid credentials error
    // todo maybe change this to tuple response (code, message)
    pub fn unauthorized() -> String {
        let status_code = HttpStatusCode::Unauthorized.code();
        let status_message = HttpStatusCode::Unauthorized.message();
        format!("{} {}", status_code, status_message)
    }

    // todo
    //pub fn success<T: SuccessfulHttpResponse>(
    //    _values_to_send: T,
    //) -> (HttpStatusCode, impl SuccessfulHttpResponse) {
    //    todo!()
    // }
}
