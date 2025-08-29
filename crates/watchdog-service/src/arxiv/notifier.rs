use async_trait::async_trait;
use watchdog_core::{Notification, Notifier};
use crate::arxiv::model::ArxivPaper;

/// A notifier for arXiv papers (console output)
#[derive(Clone)]
pub struct ArxivNotifier;

#[async_trait]
impl Notifier<ArxivPaper> for ArxivNotifier {
    async fn send(
        &self,
        notification: Notification<ArxivPaper>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "=== New arXiv Paper Alert ===\n\
             To: {}\n\
             Title: {}\n\
             Authors: {}\n\
             Published: {}\n\
             Summary: {}\n\
             Link: {}\n\
             ==============================",
            notification.user_id,
            notification.content.title,
            notification.content.authors.join(", "),
            notification.content.published,
            notification.content.summary,
            notification.content.link
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "arxiv_notifier"
    }

    fn set_name(&mut self, _name: String) {
        // Not implemented for this example
    }
}
