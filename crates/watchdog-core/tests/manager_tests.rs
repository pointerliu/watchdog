use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use watchdog_core::fetchers::FetcherManager;
use watchdog_core::notifiers::NotifierManager;
use watchdog_core::storage::FetchStorage;
use watchdog_core::{
    FetchResult, Fetcher, Manager, Notification, Notifier, Subscription, SubscriptionCriteria,
    SubscriptionManager,
};

// A simple fetcher implementation for testing
struct TestFetcher {
    data: Vec<String>,
    counter: Arc<tokio::sync::Mutex<usize>>,
}

impl TestFetcher {
    fn new(data: Vec<String>) -> Self {
        Self {
            data,
            counter: Arc::new(tokio::sync::Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl Fetcher<String> for TestFetcher {
    async fn fetch(&self) -> Result<FetchResult<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut counter = self.counter.lock().await;
        let content = self.data[*counter % self.data.len()].clone();
        *counter += 1;

        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "test".to_string());

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

// A test notifier to capture notifications
#[derive(Clone)]
struct TestNotifier {
    notifications: Arc<tokio::sync::Mutex<Vec<Notification<String>>>>,
}

impl TestNotifier {
    fn new() -> Self {
        Self {
            notifications: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    async fn get_notifications(&self) -> Vec<Notification<String>> {
        self.notifications.lock().await.clone()
    }
}

#[async_trait::async_trait]
impl Notifier<String> for TestNotifier {
    async fn send(
        &self,
        notification: Notification<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.notifications.lock().await.push(notification);
        Ok(())
    }
}

// A simple subscription criteria for testing
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TestSubscriptionCriteria {
    id: String,
    keywords: Vec<String>,
}

impl TestSubscriptionCriteria {
    fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for TestSubscriptionCriteria {
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

#[actix::test]
async fn test_fetcher_manager() {
    let storage = FetchedDataStorage::<String>::new();
    let fetcher_manager = FetcherManager::new(Duration::from_millis(100), storage, 4);

    let fetcher = TestFetcher::new(vec!["test data".to_string()]);

    fetcher_manager
        .add_fetcher("test".to_string(), Box::new(fetcher))
        .await;

    // Start the manager
    fetcher_manager.start().unwrap();

    // Wait a bit for the fetcher to run
    tokio::time::sleep(Duration::from_millis(250)).await;

    // Stop the manager
    fetcher_manager.stop().unwrap();

    // Check that data was stored
    let storage = fetcher_manager.get_storage();
    let data = storage.get_all().await;

    assert!(!data.is_empty());
    assert!(data[0].content.contains("test data"));
}

#[tokio::test]
async fn test_notifier_manager() {
    // Create a subscription manager
    let subscription_manager = Arc::new(RwLock::new(
        SubscriptionManager::<TestSubscriptionCriteria>::new(),
    ));

    // Add a subscription
    let subscription = Subscription::new(
        "test_user".to_string(),
        TestSubscriptionCriteria::new("test_sub".to_string(), vec!["test".to_string()]),
    );

    {
        let mut sm = subscription_manager.write().await;
        sm.add_subscription(subscription);
    }

    // Create a test notifier
    let notifier = Arc::new(TestNotifier::new());

    // Create notifier manager
    let notifier_manager = NotifierManager::<String, TestSubscriptionCriteria>::new(
        notifier.clone(),
        subscription_manager,
    );
    notifier_manager.start().unwrap();

    // Send a notification
    notifier_manager
        .send_notifications("this is a test message".to_string())
        .await
        .unwrap();

    // Check that notification was sent
    let notifications = notifier.get_notifications().await;
    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].user_id, "test_user");
    assert_eq!(notifications[0].content, "this is a test message");
}