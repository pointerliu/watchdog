use crate::notifiers::actor::{NotifierActor, SendNotifications};
use crate::{Manager, Notifier, SubscriptionCriteria, SubscriptionManager};
use actix::prelude::*;
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
    actor_address: Addr<NotifierActor<T, C>>,
}

impl<T: Clone + Send + Sync + 'static, C: SubscriptionCriteria + Clone + 'static> NotifierManager<T, C>
where
    T: Clone + Into<C::Content> + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    pub fn new(
        notifier: Arc<dyn Notifier<T> + Send + Sync>,
        subscription_manager: Arc<RwLock<SubscriptionManager<C>>>,
    ) -> Self {
        let actor = NotifierActor::new(notifier, subscription_manager);
        let actor_address = actor.start();

        Self { actor_address }
    }

    /// Send notifications for the given content to all matching subscribers
    pub async fn send_notifications(
        &self,
        content: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: Clone + Into<C::Content>,
    {
        self.actor_address
            .send(SendNotifications { content })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to send notifications: {}", e);
                Err(Box::new(e))
            })
    }
}

#[async_trait::async_trait]
impl<T: Clone + Send + Sync + 'static, C: SubscriptionCriteria + Clone + 'static> Manager
    for NotifierManager<T, C>
where
    T: Clone + Into<C::Content> + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // For NotifierManager, start doesn't do anything special since it's
        // triggered by external events (new fetched data)
        info!("NotifierManager started");
        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("NotifierManager stopped");
        Ok(())
    }
}
