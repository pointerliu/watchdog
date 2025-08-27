use watchdog::subscription::{Subscription, SubscriptionCriteria, SubscriptionManager};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TestCriteria {
    id: String,
    keywords: Vec<String>,
}

impl TestCriteria {
    fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for TestCriteria {
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

#[tokio::test]
async fn test_subscription_manager() {
    let mut manager = SubscriptionManager::<TestCriteria>::new();
    
    let subscription = Subscription::new(
        "test_user".to_string(),
        TestCriteria::new("test_id".to_string(), vec!["test".to_string()])
    );
    
    manager.add_subscription(subscription);
    
    assert_eq!(manager.get_subscriptions().len(), 1);
    
    let retrieved = manager.get_subscription(&"test_id".to_string());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().user_id, "test_user");
    
    // Test removal
    let removed = manager.remove_subscription(&"test_id".to_string());
    assert!(removed.is_some());
    assert_eq!(manager.get_subscriptions().len(), 0);
}

#[tokio::test]
async fn test_criteria_matching() {
    let criteria = TestCriteria::new("test_id".to_string(), vec!["rust".to_string(), "programming".to_string()]);
    
    assert!(criteria.matches(&"I love rust programming".to_string()));
    assert!(criteria.matches(&"rust is great".to_string()));
    assert!(!criteria.matches(&"Python is awesome".to_string()));
}