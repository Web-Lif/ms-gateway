use actix_web::{error, HttpResponse, http::{ StatusCode }};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
#[display(fmt = "error: {}", message)]
pub struct Error {
    pub message: String
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let rest = json!({
            "message": &self.message,
        });

        HttpResponse::build(self.status_code())
            .json(rest)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}