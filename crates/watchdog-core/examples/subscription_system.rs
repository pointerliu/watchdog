use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use watchdog_core::fetchers::FetcherManager;
use watchdog_core::notifiers::NotifierManager;
use watchdog_core::storage::FetchStorage;
use watchdog_core::{
    ConsoleNotifier, FetchResult, Fetcher, Manager, Subscription, SubscriptionCriteria,
    SubscriptionManager,
};

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

#[derive(Debug, Clone)]
pub struct FetchedDataStorage<T: Clone> {
    data: Arc<RwLock<Vec<FetchResult<T>>>>,
}

impl<T: Clone> FetchedDataStorage<T> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl<T: Clone + Send + Sync> FetchStorage<T> for FetchedDataStorage<T> {
    async fn store(&self, result: FetchResult<T>) {
        let mut data = self.data.write().await;
        data.push(result);
    }

    async fn get_all(&self) -> Vec<FetchResult<T>> {
        let data = self.data.read().await;
        data.clone()
    }

    async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
    }
}

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let storage = FetchedDataStorage::<String>::new();
    // Create a fetcher manager that runs every 5 seconds with 4 threads
    let fetcher_manager = FetcherManager::new(Duration::from_secs(5), storage, 4);

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

    // Add fetchers to the manager
    fetcher_manager
        .add_fetcher("news1".to_string(), Box::new(fetcher1))
        .await;
    fetcher_manager
        .add_fetcher("news2".to_string(), Box::new(fetcher2))
        .await;

    // Start the fetcher manager
    fetcher_manager
        .start()
        .map_err(|e| e as Box<dyn std::error::Error + Send + Sync>)?;

    // Create a subscription manager
    let subscription_manager = Arc::new(RwLock::new(SubscriptionManager::<
        SimpleSubscriptionCriteria,
    >::new()));

    // Add some subscriptions
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

    {
        let sm = subscription_manager.read().await;
        sm.add_subscription(subscription1).await;
        sm.add_subscription(subscription2).await;
    }

    // Create a notifier
    let notifier = Arc::new(ConsoleNotifier);

    // Create a notifier manager
    let notifier_manager =
        NotifierManager::<String, SimpleSubscriptionCriteria>::new(notifier, subscription_manager);
    notifier_manager
        .start()
        .map_err(|e| e as Box<dyn std::error::Error + Send + Sync>)?;

    // In a real system, you would have a mechanism to trigger notifications
    // when new data is fetched. For this example, we'll just run for a while.
    println!("Running subscription system for 30 seconds...");
    tokio::time::sleep(Duration::from_secs(30)).await;

    // Stop the managers
    fetcher_manager
        .stop()
        .map_err(|e| e as Box<dyn std::error::Error + Send + Sync>)?;

    println!("System stopped");
    Ok(())
}