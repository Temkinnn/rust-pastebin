use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,
    #[error("Database Error")]
    Database(#[from] sqlx::Error),
    #[error("Resource not found")]
    NotFound,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}
