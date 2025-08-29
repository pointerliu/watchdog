//! Subscription domain models

use serde::{Deserialize, Serialize};

/// Subscription criteria for matching content
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct SubscriptionCriteria {
    pub id: String,
    pub keywords: Vec<String>,
}

impl SubscriptionCriteria {
    pub fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}
