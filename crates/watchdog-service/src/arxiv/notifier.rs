use crate::arxiv::model::ArxivPaper;
use crate::email::EmailService;
use async_trait::async_trait;
use watchdog_core::{Notification, Notifier};

/// A notifier for arXiv papers (console output)
#[derive(Clone)]
pub struct ArxivConsoleNotifier {
    name: String,
}

impl Default for ArxivConsoleNotifier {
    fn default() -> Self {
        Self {
            name: "arxiv_notifier".to_string(),
        }
    }
}

#[async_trait]
impl Notifier<ArxivPaper> for ArxivConsoleNotifier {
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
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Clone)]
pub struct ArxivEmailNotifier {
    name: String,
    email_service: EmailService,
    recipient_email: String,
}

impl ArxivEmailNotifier {
    pub fn new(
        name: &str,
        recipient_email: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let email_service = EmailService::new()?;
        Ok(Self {
            name: name.to_string(),
            email_service,
            recipient_email,
        })
    }
}

#[async_trait]
impl Notifier<ArxivPaper> for ArxivEmailNotifier {
    async fn send(
        &self,
        notification: Notification<ArxivPaper>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let subject = format!("New arXiv Paper: {}", notification.content.title);
        let body = format!(
            "Title: {}\n\
             Authors: {}\n\
             Published: {}\n\
             Summary: {}\n\
             Link: {}",
            notification.content.title,
            notification.content.authors.join(", "),
            notification.content.published,
            notification.content.summary,
            notification.content.link
        );

        self.email_service
            .send_email(&self.recipient_email, &subject, &body)
            .await?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
