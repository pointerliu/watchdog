//! Subscription Data Transfer Objects
//!
//! This module defines the DTOs used for subscription management.

use serde::{Deserialize, Serialize};

/// Request DTO for creating a new subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: String,
    pub subscription_id: String,
    // TODO: fix me, keywords is not generic.
    pub keywords: Vec<String>,
}

/// Response DTO for subscription operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub user_id: String,
    pub subscription_id: String,
}

/// Response DTO for getting user subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionsResponse {
    pub user_id: String,
    pub subscription_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct RemoveSubscriptionRequest {
    pub user_id: String,
    pub subscription_id: String,
}
