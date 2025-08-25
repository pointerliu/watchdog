use subscription_framework::notifier::{ConsoleNotifier, Notification};
use subscription_framework::Notifier;

#[tokio::test]
async fn test_console_notifier() {
    let notifier = ConsoleNotifier;
    let notification = Notification {
        user_id: "test_user".to_string(),
        title: "Test Notification".to_string(),
        content: "This is a test notification".to_string(),
        timestamp: 1234567890,
    };
    
    // This should not panic
    assert!(notifier.send(notification).await.is_ok());
}