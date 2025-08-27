//! Example of using the EmailNotifier with the watchdog framework

use actix::prelude::*;
use watchdog_core::subscription::Subscription;
use watchdog_service::{
    server::{AddUserWorkerMsg, SubscriptionServer},
    AddSubscriptionMsg, ServerConfig, ShutdownMsg,
};
// Import arxiv components from the local crate
use watchdog_arxiv::{ArxivCriteria, ArxivFetcher, ArxivFetcherBuilder};

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Starting EmailNotifier example...");

    // Create server config
    let config = ServerConfig::default();

    // Create and start the multi-user server
    let server = SubscriptionServer::<ArxivFetcher, ArxivCriteria>::new(config);
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
        worker_addr
            .send(AddSubscriptionMsg { subscription })
            .await?;
    }

    println!("EmailNotifier example running. Press Ctrl+C to stop.");
    println!("Note: This example uses ConsoleNotifier by default.");
    println!("To use EmailNotifier, you need to add it to the worker after creation.");

    // Run for a while to demonstrate
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Shutdown the server
    server_addr.send(ShutdownMsg).await?;

    println!("EmailNotifier example finished.");

    Ok(())
}
