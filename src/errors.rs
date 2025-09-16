use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
#[error("bad request: {0}")]
BadRequest(String),
#[error("internal error")]
Internal,
}


#[derive(Serialize)]
struct ErrBody { error: String }


impl ResponseError for AppError {
fn error_response(&self) -> HttpResponse {
match self {
AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrBody { error: msg.clone() }),
AppError::Internal => HttpResponse::InternalServerError().json(ErrBody { error: "internal".into() }),
}
}
}