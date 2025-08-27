pub mod fetchers;
pub mod notifiers;
pub mod subscription;

pub use fetchers::{FetchResult, Fetcher};
pub use notifiers::composition::CompositeNotifier;
pub use notifiers::{ConsoleNotifier, EmailNotifier, Notification, Notifier};
pub use subscription::{Subscription, SubscriptionCriteria, SubscriptionManager};

#[derive(Debug, thiserror::Error)]
pub enum FrameworkError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Email error: {0}")]
    Email(String),
}
