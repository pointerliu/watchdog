use crate::{Manager, Notification, Notifier, SubscriptionCriteria, SubscriptionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Manager for notifiers that sends notifications based on subscriptions
pub struct NotifierManager<T: Clone, C: SubscriptionCriteria + 'static>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
    T: Send + Sync + 'static,
{
    notifier: Arc<dyn Notifier<T> + Send + Sync>,
    subscription_manager: Arc<RwLock<SubscriptionManager<C>>>,
    running: Arc<RwLock<bool>>,
}

impl<T: Clone + Send + Sync + 'static, C: SubscriptionCriteria + Clone + 'static> NotifierManager<T, C>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    pub fn new(
        notifier: Arc<dyn Notifier<T> + Send + Sync>,
        subscription_manager: Arc<RwLock<SubscriptionManager<C>>>,
    ) -> Self {
        Self {
            notifier,
            subscription_manager,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Send notifications for the given content to all matching subscribers
    pub async fn send_notifications(
        &self,
        content: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: Clone + Into<C::Content>,
    {
        // Get matching subscriptions using the actor
        let matching_subscriptions = {
            let sm = self.subscription_manager.read().await;
            let content_ref: C::Content = content.clone().into();
            sm.get_matching_subscriptions(content_ref).await
        };

        info!(
            "Found {} matching subscriptions",
            matching_subscriptions.len()
        );

        // Create and send notifications
        for subscription in matching_subscriptions {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(std::time::Duration::from_secs(0))
                .as_secs();

            let notification = Notification {
                user_id: subscription.user_id.clone(),
                title: "Subscription Update".to_string(),
                content: content.clone(),
                timestamp,
            };

            match self.notifier.send(notification).await {
                Ok(()) => {
                    info!(
                        "Successfully sent notification to user {}",
                        subscription.user_id
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to send notification to user {}: {}",
                        subscription.user_id, e
                    );
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: Clone + Send + Sync + 'static, C: SubscriptionCriteria + Clone + 'static> Manager
    for NotifierManager<T, C>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // For NotifierManager, start doesn't do anything special since it's
        // triggered by external events (new fetched data)
        let running = self.running.clone();
        tokio::spawn(async move {
            let mut running_guard = running.write().await;
            *running_guard = true;
            info!("NotifierManager started");
        });
        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let running = self.running.clone();
        tokio::spawn(async move {
            let mut running_guard = running.write().await;
            *running_guard = false;
            info!("NotifierManager stopped");
        });
        Ok(())
    }
}
