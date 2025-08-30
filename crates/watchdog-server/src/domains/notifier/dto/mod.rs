//! Notifier domain models and traits

use serde::{Deserialize, Serialize};

/// Represents a notifier type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifierType {
    pub name: String,
}

/// Request to add a new notifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNotifierRequest {
    pub user_id: String,
    pub notifier_name: String,
    pub notifier_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}

/// Request to remove a notifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNotifierRequest {
    pub user_id: String,
    pub notifier_name: String,
}

/// Response for getting user's current notifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNotifierResponse {
    pub user_id: String,
    pub notifier_name: String,
    pub notifier_type: String,
}
