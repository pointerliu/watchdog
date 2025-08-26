//! Example of using the EmailNotifier with the watchdog framework

use actix::prelude::*;
use watchdog::{notifier::ConsoleNotifier, subscription::Subscription, EmailNotifier};
use watchdog_server::{
    server::{SubscriptionServer, AddUserWorkerMsg},
    AddSubscriptionMsg, ServerConfig, ShutdownMsg,
};
// Import arxiv components from the local crate
mod arxiv;
use arxiv::{ArxivFetcher, ArxivFetcherBuilder, ArxivCriteria};

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Starting EmailNotifier example...");

    // Note: To run this example, you need to configure real SMTP settings
    // For demonstration purposes, we'll show how to set it up
    
    // Create EmailNotifier with your SMTP settings
    // Replace these with your actual SMTP configuration:

    let email_notifier = EmailNotifier::new(
        "smtp.example.com".to_string(),     // SMTP server
        587,                                // SMTP port
        "your-email@example.com".to_string(), // SMTP username
        "your-password".to_string(),        // SMTP password
    );
    
    // Set user email addresses
    email_notifier.set_user_email("user1".to_string(), "user1@example.com".to_string()).await;
    email_notifier.set_user_email("user2".to_string(), "user2@example.com".to_string()).await;

    
    // For this example, we'll use a console notifier to show how it would work
    let console_notifier = ConsoleNotifier;
    
    // Create server config
    let config = ServerConfig::default();
    
    // Create and start the multi-user server
    let server = SubscriptionServer::<ArxivFetcher, ConsoleNotifier, ArxivCriteria>::new(config);
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
            // notifier: console_notifier,
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
    
    // Run for a while to demonstrate
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // Shutdown the server
    server_addr.send(ShutdownMsg).await?;

    println!("EmailNotifier example finished.");

    Ok(())
}