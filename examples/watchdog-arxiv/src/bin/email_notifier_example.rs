//! Example of using the EmailNotifier with the watchdog framework

use actix::prelude::*;
use watchdog::{subscription::Subscription, EmailNotifier};
use watchdog_server::{
    server::{SubscriptionServer, AddUserWorkerMsg},
    AddSubscriptionMsg, ServerConfig, ShutdownMsg,
};
// Import arxiv components from the local crate
use watchdog_arxiv::{ArxivFetcher, ArxivFetcherBuilder, ArxivCriteria};

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Starting EmailNotifier example...");

    // Create EmailNotifier with 163 SMTP settings
    // IMPORTANT: 
    // 1. You need to enable SMTP in your 163 email settings
    // 2. Use an app password, not your regular password
    // 3. Replace the placeholders with your actual credentials
    let email_notifier = EmailNotifier::new(
        "smtp.163.com".to_string(),         // 163 SMTP server
        465,                                // SMTP port (SSL) - try 994 if this doesn't work
        "ellen7ions@163.com".to_string(),   // Replace with your actual 163 email address
        "GWWzTPZs5XQ4Y5PU".to_string(),    // Replace with your 163 app password (not regular password)
    );
    
    // Set user email addresses - sending notifications to ellen7ions@163.com
    email_notifier.set_user_email("ml_researcher".to_string(), "ellen7ions@163.com".to_string()).await;
    
    // Check if credentials are properly set
    if email_notifier.smtp_username == "YOUR_ACTUAL_163_EMAIL@163.com" || 
       email_notifier.smtp_password == "YOUR_163_APP_PASSWORD" {
        println!("⚠️  Please update the email credentials in the source code!");
        println!("   Replace 'YOUR_ACTUAL_163_EMAIL@163.com' with your actual 163 email address");
        println!("   Replace 'YOUR_163_APP_PASSWORD' with your 163 app password");
        return Ok(());
    }
    
    // Create server config
    let config = ServerConfig::default();
    
    // Create and start the multi-user server - now with EmailNotifier type
    let server = SubscriptionServer::<ArxivFetcher, EmailNotifier, ArxivCriteria>::new(config);
    let server_addr = server.start();
    
    // Add a user worker
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(5)
        .build()
        .unwrap();
    
    let worker_result = server_addr
        .send(AddUserWorkerMsg {
            user_id: "ml_researcher".to_string(),
            fetcher,
            notifier: email_notifier,
            phantom: std::marker::PhantomData,
        })
        .await?;
    
    if let Ok(worker_addr) = worker_result {
        // Add a subscription
        let subscription = Subscription::new(
            "ml_researcher".to_string(),
            ArxivCriteria::new(
                "ml_papers".to_string(),
                vec!["neural".to_string(), "deep learning".to_string()],
            ),
        );
        worker_addr.send(AddSubscriptionMsg { subscription }).await?;
    }

    println!("EmailNotifier example running. Press Ctrl+C to stop.");
    println!("If you get authentication errors, try running the test-email binary first:");
    println!("  cargo run --bin test-email");
    
    // Run for a while to demonstrate
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // Shutdown the server
    server_addr.send(ShutdownMsg).await?;

    println!("EmailNotifier example finished.");

    Ok(())
}