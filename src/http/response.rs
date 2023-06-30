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
    pub fn success<T: SuccessfulHttpResponse>(
        _values_to_send: T::SuccessValue,
    ) -> (HttpStatusCode, T::FinalHttpResponse) {
        todo!()
    }
}

// TODO

pub struct TypeA;
pub struct TypeB;

pub trait SuccessfulHttpResponse {
    type SuccessValue;
    type FinalHttpResponse;
}

pub struct TypeAValue;
pub struct TypeAHttpResponse;

impl SuccessfulHttpResponse for TypeA {
    type SuccessValue = TypeAValue;
    type FinalHttpResponse = TypeAHttpResponse;
}

pub struct TypeBValue;
pub struct TypeBHttpResponse;

impl SuccessfulHttpResponse for TypeB {
    type SuccessValue = TypeBValue;
    type FinalHttpResponse = TypeBHttpResponse;
}
