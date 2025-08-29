//! Subscription Data Transfer Objects
//!
//! This module defines the DTOs used for subscription management.

use serde::{Deserialize, Serialize};

/// Request DTO for creating a new subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: String,
    pub subscription_name: String,
    pub keywords: Vec<String>,
}

/// Response DTO for subscription operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub user_id: String,
    pub subscription_name: String,
}

/// Response DTO for getting user subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionsResponse {
    pub user_id: String,
    pub subscriptions: Vec<SubscriptionInfo>,
}

/// Information about a subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub subscription_name: String,
    pub keywords: Vec<String>,
}
