//! Example demonstrating how to use the Watchdog system
//!
//! This example shows how to set up and run a complete watchdog system
//! with fetchers, notifiers, and subscriptions.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use watchdog_core::fetchers::Fetcher;
use watchdog_core::notifiers::ConsoleNotifier;
use watchdog_core::subscription::{Subscription, SubscriptionCriteria};
use watchdog_core::{FetchResult, FrameworkError, Watchdog};

// A simple fetcher implementation for demonstration
#[derive(Clone)]
struct SimpleFetcher {
    name: String,
    data: Vec<String>,
    counter: Arc<std::sync::Mutex<usize>>,
}

impl SimpleFetcher {
    fn new(name: String, data: Vec<String>) -> Self {
        Self {
            name,
            data,
            counter: Arc::new(std::sync::Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl Fetcher<String> for SimpleFetcher {
    async fn fetch(&self) -> Result<FetchResult<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut counter = self.counter.lock().unwrap();
        let content = self.data[*counter % self.data.len()].clone();
        *counter += 1;

        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), self.name.clone());

        Ok(FetchResult {
            content,
            metadata,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}

// A simple subscription criteria for the example
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SimpleSubscriptionCriteria {
    id: String,
    keywords: Vec<String>,
}

impl SimpleSubscriptionCriteria {
    fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for SimpleSubscriptionCriteria {
    type Id = String;
    type Content = String;

    fn matches(&self, content: &String) -> bool {
        self.keywords
            .iter()
            .any(|keyword| content.contains(keyword))
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[actix::main]
async fn main() -> Result<(), FrameworkError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create the watchdog system with default configuration
    let watchdog: Watchdog<String, SimpleSubscriptionCriteria> = Watchdog::with_defaults();

    // Start the watchdog system
    println!("Starting watchdog system...");
    watchdog.start()?;

    // Create some sample fetchers
    let fetcher1 = SimpleFetcher::new(
        "news_source_1".to_string(),
        vec![
            "Rust 2.0 release date announced".to_string(),
            "New features in Rust".to_string(),
        ],
    );

    let fetcher2 = SimpleFetcher::new(
        "news_source_2".to_string(),
        vec![
            "Programming tips for beginners".to_string(),
            "Best practices in system design".to_string(),
        ],
    );

    // Add fetchers to the watchdog
    watchdog
        .add_fetcher("news1".to_string(), Box::new(fetcher1))
        .await?;
    watchdog
        .add_fetcher("news2".to_string(), Box::new(fetcher2))
        .await?;

    // Create notifiers for users
    let notifier1 = Arc::new(ConsoleNotifier::new("console1".to_string()));
    let notifier2 = Arc::new(ConsoleNotifier::new("console2".to_string()));

    // Add notifiers to the watchdog
    watchdog
        .add_notifier("user1".to_string(), notifier1)
        .await?;
    watchdog
        .add_notifier("user2".to_string(), notifier2)
        .await?;

    // Create some subscriptions
    let subscription1 = Subscription::new(
        "user1".to_string(),
        SimpleSubscriptionCriteria::new("sub1".to_string(), vec!["Rust".to_string()]),
    );

    let subscription2 = Subscription::new(
        "user2".to_string(),
        SimpleSubscriptionCriteria::new(
            "sub2".to_string(),
            vec!["programming".to_string(), "tips".to_string()],
        ),
    );

    // Add subscriptions to the watchdog
    watchdog.add_subscription(subscription1).await?;
    watchdog.add_subscription(subscription2).await?;

    // Let it run for a while
    println!("Watchdog system running for 30 seconds...");
    tokio::time::sleep(Duration::from_secs(30)).await;

    // Stop the watchdog system
    println!("Stopping watchdog system...");
    watchdog.stop()?;

    println!("Watchdog system finished");
    Ok(())
}
