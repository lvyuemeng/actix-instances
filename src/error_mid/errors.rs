use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserError {
    #[display(fmt = "User not found")]
    NotFound,
    #[display(fmt = "Invalid input")]
    ValidationError,
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::NotFound => HttpResponse::NotFound().json("User not found"),
            UserError::ValidationError => HttpResponse::BadRequest().json("Invalid input"),
            UserError::InternalServerError => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}