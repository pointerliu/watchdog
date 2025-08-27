use crate::{Notification, Notifier};

/// A simple console notifier for testing
#[derive(Clone)]
pub struct ConsoleNotifier;

#[async_trait::async_trait]
impl<T: std::fmt::Display + Clone + Send + Sync + 'static> Notifier<T> for ConsoleNotifier {
    async fn send(
        &self,
        notification: Notification<T>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Notification to {}: {} - {} (at {})",
            notification.user_id, notification.title, notification.content, notification.timestamp
        );
        Ok(())
    }
}
