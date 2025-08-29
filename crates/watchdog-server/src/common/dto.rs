//! Common Data Transfer Objects
//!
//! This module defines shared DTOs used across different domains.

use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Create a success response with data
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    /// Create a success response with a message but no data
    pub fn success_with_message(message: String) -> Self {
        Self {
            status: 200,
            message,
            data: None,
        }
    }
    
    /// Create an error response
    pub fn error(status: u16, message: String) -> Self {
        Self {
            status,
            message,
            data: None,
        }
    }
}