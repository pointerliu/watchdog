use watchdog_core::{ConsoleNotifier, Notification, Notifier};

#[tokio::test]
async fn test_console_notifier() {
    let mut notifier: ConsoleNotifier = ConsoleNotifier::new("console".to_string());
    let notification = Notification {
        user_id: "test_user".to_string(),
        title: "Test Notification".to_string(),
        content: "This is a test notification".to_string(),
        timestamp: 1234567890,
    };

    // Test that we can set the name
    Notifier::<String>::set_name(&mut notifier, "new_name".to_string());
    assert_eq!(Notifier::<String>::name(&notifier), "new_name");

    // This should not panic
    assert!(notifier.send(notification).await.is_ok());
}
