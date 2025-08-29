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
    use_tls: bool, // Flag to indicate if TLS should be used
}

impl EmailService {
    /// Create a new EmailService instance.
    /// 
    /// This function reads SMTP configuration from environment variables:
    /// - SMTP_SERVER: The SMTP server address (e.g., "smtp.163.com")
    /// - SMTP_PORT: The SMTP server port (e.g., 465 for SSL/TLS, 587 for STARTTLS)
    /// - SMTP_USERNAME: The SMTP username (your email address)
    /// - SMTP_PASSWORD: The SMTP password (your app password)
    /// - EMAIL_FROM: The sender's email address
    /// - SMTP_USE_TLS: Whether to use TLS ("true" or "false")
    /// 
    /// Different SMTP servers may require different ports and encryption methods:
    /// - 163.com: Port 465 with SSL/TLS (SMTP_USE_TLS=true)
    /// - Gmail: Port 587 with STARTTLS (SMTP_USE_TLS=false) or Port 465 with SSL/TLS
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get SMTP configuration from environment variables
        let smtp_server = env::var("SMTP_SERVER")?;
        let smtp_port = env::var("SMTP_PORT")?.parse::<u16>()?;
        let smtp_username = env::var("SMTP_USERNAME")?;
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let from = env::var("EMAIL_FROM")?;
        // Check if TLS should be used (default to false for backward compatibility)
        let use_tls = env::var("SMTP_USE_TLS").unwrap_or_default().to_lowercase() == "true";

        Ok(EmailService {
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
            from,
            use_tls,
        })
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let credentials = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        // Print debugging information
        println!("SMTP Server: {}", self.smtp_server);
        println!("SMTP Port: {}", self.smtp_port);
        println!("SMTP Username: {}", self.smtp_username);
        println!("Use TLS: {}", self.use_tls);

        let mailer: AsyncSmtpTransport<Tokio1Executor> = if self.use_tls {
            // Use TLS connection
            println!("Using TLS connection");
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_server)?
                .port(self.smtp_port)
                .credentials(credentials)
                .build()
        } else {
            // Use STARTTLS connection
            println!("Using STARTTLS connection");
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_server)?
                .port(self.smtp_port)
                .credentials(credentials)
                .build()
        };

        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(body))?;

        match mailer.send(email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => {
                println!("Failed to send email: {:?}", e);
                return Err(e.into());
            }
        }
        
        Ok(())
    }
}