/// Represents a notification to be sent to a user
#[derive(Debug, Clone)]
pub struct Notification<T> {
    pub user_id: String,
    pub title: String,
    pub content: T,
    pub timestamp: u64,
}

/// A trait for sending notifications
#[async_trait::async_trait]
pub trait Notifier<T>: Send + Sync 
where 
    T: Send + Sync + 'static
{
    /// Send a notification to a user
    async fn send(&self, notification: Notification<T>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// A simple console notifier for testing
pub struct ConsoleNotifier;

#[async_trait::async_trait]
impl<T: std::fmt::Display + Send + Sync + 'static> Notifier<T> for ConsoleNotifier {
    async fn send(&self, notification: Notification<T>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Notification to {}: {} - {} (at {})",
            notification.user_id, notification.title, notification.content, notification.timestamp
        );
        Ok(())
    }
}