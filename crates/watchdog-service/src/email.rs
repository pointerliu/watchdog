use lettre::{
    message::{header::ContentType, Message},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use std::env;

#[derive(Clone)]
pub struct EmailService {
    // Store configuration parameters instead of the mailer directly
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from: String,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get SMTP configuration from environment variables
        let smtp_server = env::var("SMTP_SERVER")?;
        let smtp_port = env::var("SMTP_PORT")?.parse::<u16>()?;
        let smtp_username = env::var("SMTP_USERNAME")?;
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let from = env::var("EMAIL_FROM")?;

        Ok(EmailService {
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
            from,
        })
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let credentials = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_server)?
                .port(self.smtp_port)
                .credentials(credentials)
                .build();

        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(body))?;

        mailer.send(email).await?;
        Ok(())
    }
}