use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error interno del servidor".to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Recurso no encontrado".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "No tienes permiso para esta acción".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        (status, message).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
