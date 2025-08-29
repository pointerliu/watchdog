//! Application error types
//!
//! This module defines the centralized error handling for the application.

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use watchdog_core::FrameworkError;

/// Application error types
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Watchdog error: {0}")]
    Watchdog(#[from] FrameworkError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

/// API error response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl ApiErrorResponse {
    pub fn new(status: u16, message: String) -> Self {
        Self {
            status,
            message,
            data: None,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::Watchdog(e) => (500, format!("Watchdog error: {}", e)),
            AppError::Io(e) => (500, format!("IO error: {}", e)),
            AppError::Serialization(e) => (500, format!("Serialization error: {}", e)),
            AppError::InvalidInput(e) => (400, format!("Invalid input: {}", e)),
            AppError::NotFound(e) => (404, format!("Not found: {}", e)),
        };

        let error_response = ApiErrorResponse::new(status, message);

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Watchdog(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serialization(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}

impl From<AppError> for HttpResponse {
    fn from(error: AppError) -> Self {
        error.error_response()
    }
}
