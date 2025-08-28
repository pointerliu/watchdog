use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_core::notifiers::{ConsoleNotifier, NotifierManager};
use watchdog_core::{Subscription, SubscriptionCriteria, SubscriptionManager};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TestSubscriptionCriteria {
    id: String,
    keyword: String,
}

impl TestSubscriptionCriteria {
    fn new(id: String, keyword: String) -> Self {
        Self { id, keyword }
    }
}

impl SubscriptionCriteria for TestSubscriptionCriteria {
    type Id = String;
    type Content = String;

    fn matches(&self, content: &String) -> bool {
        content.contains(&self.keyword)
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[actix::test]
async fn test_notifier_manager_with_actix() {
    // Create a subscription manager
    let subscription_manager = Arc::new(RwLock::new(
        SubscriptionManager::<TestSubscriptionCriteria>::new(),
    ));

    // Add a subscription
    let subscription = Subscription::new(
        "test_user".to_string(),
        TestSubscriptionCriteria::new("sub1".to_string(), "test".to_string()),
    );

    {
        let sm = subscription_manager.read().await;
        sm.add_subscription(subscription).await;
    }

    // Create a notifier
    let notifier = Arc::new(ConsoleNotifier);

    let notification = "this is a test message".to_string();

    // Create a notifier manager
    let notifier_manager =
        NotifierManager::<String, TestSubscriptionCriteria>::new(subscription_manager);
    
    // Add the notifier for the user
    notifier_manager.add_notifier("test_user".to_string(), notifier).await;

    // Send a notification
    let result = notifier_manager.send_content(notification).await;

    // The result should be Ok
    assert!(result.is_ok());
}
