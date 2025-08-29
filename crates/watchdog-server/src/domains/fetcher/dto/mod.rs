//! Fetcher domain models and traits

use serde::{Deserialize, Serialize};

/// Represents a fetcher type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetcherType {
    pub name: String,
}

/// Request to add a new fetcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFetcherRequest {
    pub user_id: String,
    pub fetcher_name: String,
    pub fetcher_type: String,
}

/// Request to remove a fetcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveFetcherRequest {
    pub user_id: String,
    pub fetcher_name: String,
}