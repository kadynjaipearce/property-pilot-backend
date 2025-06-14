use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use thiserror::Error;
use tracing::{error, warn, info};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub error: Option<String>,
    pub body: Option<T>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
    
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Missing token")]
    MissingToken,
    
    #[error("Expired token")]
    ExpiredToken,
}

impl From<AppError> for shuttle_runtime::Error {
    fn from(error: AppError) -> Self {
        shuttle_runtime::Error::Custom(anyhow::anyhow!(error))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::Database(e) => {
                error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", e.to_string())
            }
            AppError::Env(e) => {
                error!("Environment error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "ENVIRONMENT_ERROR", e.to_string())
            }
            AppError::NotFound(msg) => {
                warn!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone())
            }
            AppError::BadRequest(msg) => {
                warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone())
            }
            AppError::Unauthorized(msg) => {
                warn!("Unauthorized: {}", msg);
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone())
            }
            AppError::Internal(msg) => {
                error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg.clone())
            }
            // Auth error cases
            AppError::InvalidToken => {
                warn!("Invalid token provided");
                (StatusCode::UNAUTHORIZED, "INVALID_TOKEN", "Invalid token".to_string())
            }
            AppError::MissingToken => {
                warn!("No token provided");
                (StatusCode::UNAUTHORIZED, "MISSING_TOKEN", "Missing token".to_string())
            }
            AppError::ExpiredToken => {
                warn!("Token has expired");
                (StatusCode::UNAUTHORIZED, "EXPIRED_TOKEN", "Expired token".to_string())
            }
        };

        let body = ApiResponse::<Value> {
            message,
            error: Some(error_type.to_string()),
            body: None,
        };

        (status, Json(body)).into_response()
    }
}

// This is our custom Result type that we'll use throughout the application
pub type Result<T> = std::result::Result<T, AppError>; 