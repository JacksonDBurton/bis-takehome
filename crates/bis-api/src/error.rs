use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display("Custom Error 1")]
    CustomOne,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::CustomOne => {
                println!("Handle Custom error 1");
                HttpResponse::InternalServerError().finish()
            }
            _ => {
                println!("Hey look a catch all error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}
