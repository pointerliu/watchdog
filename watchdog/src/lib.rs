pub mod subscription;
pub mod notifier;
pub mod fetcher;
pub mod storage;

pub use subscription::{Subscription, SubscriptionManager, SubscriptionCriteria};
pub use notifier::{Notifier, Notification, ConsoleNotifier};
pub use fetcher::{Fetcher, FetchResult};
pub use storage::{Storage, StorageError, InMemoryStorage};

#[derive(Debug, thiserror::Error)]
pub enum FrameworkError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Custom error: {0}")]
    Custom(String),
}