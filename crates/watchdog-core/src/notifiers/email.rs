use crate::{Notification, Notifier};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Email notifier for sending notifications via email
#[derive(Clone)]
pub struct EmailNotifier {
    name: String,
    /// SMTP server configuration
    smtp_server: String,
    smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    /// User email addresses mapping user_id to email
    user_emails: Arc<RwLock<HashMap<String, String>>>,
}

impl EmailNotifier {
    /// Create a new EmailNotifier with SMTP configuration
    pub fn new(
        name: String,
        smtp_server: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
    ) -> Self {
        Self {
            name,
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
            user_emails: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add or update a user's email address
    pub async fn set_user_email(&self, user_id: String, email: String) {
        let mut emails = self.user_emails.write().await;
        emails.insert(user_id, email);
    }

    /// Get a user's email address
    pub async fn get_user_email(&self, user_id: &str) -> Option<String> {
        let emails = self.user_emails.read().await;
        emails.get(user_id).cloned()
    }
}

#[async_trait::async_trait]
impl<T: std::fmt::Display + Clone + Send + Sync + 'static> Notifier<T> for EmailNotifier {
    async fn send(
        &self,
        notification: Notification<T>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get the user's email address
        let email = {
            let emails = self.user_emails.read().await;
            emails.get(&notification.user_id).cloned()
        };

        if let Some(email_address) = email {
            // Create the email message
            let email = Message::builder()
                .from(self.smtp_username.parse()?)
                .to(email_address.parse()?)
                .subject(&notification.title)
                .body(format!(
                    "Hello {},\n\n{}\n\nSent at: {}",
                    notification.user_id, notification.content, notification.timestamp
                ))?;

            // Create the SMTP transport
            let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

            let mailer: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_server)?
                    .port(self.smtp_port)
                    .credentials(creds)
                    .build();

            // Send the email
            mailer.send(email).await?;

            tracing::info!("Email notification sent to user {}", notification.user_id);
            Ok(())
        } else {
            tracing::warn!("No email address found for user {}", notification.user_id);
            Err(format!("No email address found for user {}", notification.user_id).into())
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
