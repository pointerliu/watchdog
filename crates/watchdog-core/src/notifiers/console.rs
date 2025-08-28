use crate::{Notification, Notifier};

/// A simple console notifier for testing
#[derive(Clone)]
pub struct ConsoleNotifier {
    name: String,
}

impl ConsoleNotifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

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
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
