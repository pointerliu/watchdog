pub mod actor;
pub mod composition;
mod console;
mod email;
mod manager;

pub use console::ConsoleNotifier;
pub use email::EmailNotifier;
pub use manager::NotifierManager;

/// Represents a notification to be sent to a user
#[derive(Debug, Clone)]
pub struct Notification<T: Clone> {
    pub user_id: String,
    pub title: String,
    pub content: T,
    pub timestamp: u64,
}

impl<T> Notification<T>
where
    T: Clone,
{
    pub fn new(user_id: String, title: String, content: T) -> Self {
        Self {
            user_id,
            title,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
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

    /// Get the name of this notifier
    fn name(&self) -> &str;

    /// Set the name of this notifier
    fn set_name(&mut self, name: String);
}
