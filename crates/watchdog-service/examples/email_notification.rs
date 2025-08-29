use dotenv::dotenv;
use watchdog_core::Notification;
use watchdog_core::Notifier;
use watchdog_service::arxiv::model::ArxivPaper;
use watchdog_service::arxiv::notifier::ArxivEmailNotifier;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv()?;
    // Create a sample arXiv paper
    let paper = ArxivPaper {
        id: "1234.56789".to_string(),
        title: "Sample Paper Title".to_string(),
        summary: "This is a sample abstract for an arXiv paper.".to_string(),
        authors: vec!["Author One".to_string(), "Author Two".to_string()],
        published: "2025-01-01".to_string(),
        updated: "2025-01-01".to_string(),
        categories: vec!["cs.AI".to_string()],
        link: "https://arxiv.org/abs/1234.56789".to_string(),
    };

    // Create a notification
    let notification =
        Notification::new("user123".to_string(), "New arXiv Paper".to_string(), paper);

    // Create the email notifier (recipient email would be provided in practice)
    // Note: You'll need to set the following environment variables:
    // SMTP_SERVER, SMTP_PORT, SMTP_USERNAME, SMTP_PASSWORD, EMAIL_FROM
    let notifier =
        ArxivEmailNotifier::new("arxiv_email_notifier", "ellen7ions@163.com".to_string())?;

    // Send the notification
    notifier.send(notification).await?;

    println!("Email notification sent successfully!");

    Ok(())
}
