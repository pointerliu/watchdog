//! Example demonstrating how to use the Watchdog system with arXiv fetcher
//!
//! This example shows how to set up and run a complete watchdog system
//! with arXiv fetchers, notifiers, and subscriptions.

use chrono;
use std::sync::Arc;
use std::time::Duration;
use watchdog_core::subscription::Subscription;
use watchdog_core::{FrameworkError, Watchdog};
use watchdog_service::arxiv::criteria::ArxivCriteria;
use watchdog_service::arxiv::fetcher::ArxivFetcherBuilder;
use watchdog_service::arxiv::model::ArxivPaper;
use watchdog_service::arxiv::notifier::ArxivConsoleNotifier;

#[actix::main]
async fn main() -> Result<(), FrameworkError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create the watchdog system with default configuration
    let watchdog: Watchdog<ArxivPaper, ArxivCriteria> = Watchdog::with_defaults();

    // Start the watchdog system
    println!("Starting watchdog system...");
    watchdog.start()?;

    // Create arXiv fetchers
    let ml_fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(3)
        .build()
        .unwrap();

    let rust_fetcher = ArxivFetcherBuilder::default()
        .query("rust programming".to_string())
        .number(3)
        .build()
        .unwrap();

    // Add fetchers to the watchdog
    watchdog
        .add_fetcher("user1", "arxiv_ml", Box::new(ml_fetcher))
        .await?;
    watchdog
        .add_fetcher("user2", "arxiv_rust", Box::new(rust_fetcher))
        .await?;

    // Create notifiers for users
    let notifier1 = Arc::new(ArxivConsoleNotifier::default());
    let notifier2 = Arc::new(ArxivConsoleNotifier::default());

    // Add notifiers to the watchdog
    watchdog
        .add_notifier("researcher1".to_string(), notifier1)
        .await?;
    watchdog
        .add_notifier("researcher2".to_string(), notifier2)
        .await?;

    // Create subscriptions
    let subscription1 = Subscription::new(
        "researcher1".to_string(),
        ArxivCriteria::new("sub1".to_string(), vec!["learning".to_string()]),
    );

    let subscription2 = Subscription::new(
        "researcher2".to_string(),
        ArxivCriteria::new(
            "sub2".to_string(),
            vec!["rust".to_string(), "rust".to_string()],
        ),
    );

    // Add subscriptions to the watchdog
    watchdog.add_subscription(subscription1).await?;
    watchdog.add_subscription(subscription2).await?;

    // Let it run for a while
    println!("Watchdog system running for 60 seconds...");
    tokio::time::sleep(Duration::from_secs(60)).await;

    // Stop the watchdog system
    println!("Stopping watchdog system...");
    watchdog.stop()?;

    println!("Watchdog system finished");
    Ok(())
}
