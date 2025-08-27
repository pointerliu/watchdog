use std::collections::HashMap;

mod manager;
pub use manager::FetcherManager;

/// Result of a fetch operation
#[derive(Debug, Clone)]
pub struct FetchResult<T> {
    pub content: T,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
}

/// A trait for fetching content from external sources
#[async_trait::async_trait]
pub trait Fetcher<T>: Send + Sync {
    /// Fetch content based on criteria
    async fn fetch(&self) -> Result<FetchResult<T>, Box<dyn std::error::Error + Send + Sync>>;
}
