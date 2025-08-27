pub mod composition;
mod console;
mod email;

pub use console::ConsoleNotifier;
pub use email::EmailNotifier;

/// Represents a notification to be sent to a user
#[derive(Debug, Clone)]
pub struct Notification<T: Clone> {
    pub user_id: String,
    pub title: String,
    pub content: T,
    pub timestamp: u64,
}

/// A trait for sending notifications
#[async_trait::async_trait]
pub trait Notifier<T: Clone>: Send + Sync
where
    T: Send + Sync + 'static,
{
    /// Send a notification to a user
    async fn send(
        &self,
        notification: Notification<T>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
