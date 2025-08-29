//! Subscription Data Transfer Objects
//!
//! This module defines the DTOs used for subscription management.

use serde::{Deserialize, Serialize};

/// Request DTO for creating a new subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: String,
    pub criteria_id: String,
    pub keywords: Vec<String>,
}

/// Response DTO for subscription operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub user_id: String,
    pub criteria_id: String,
}

/// Request DTO for removing a subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveSubscriptionRequest {
    pub criteria_id: String,
}
