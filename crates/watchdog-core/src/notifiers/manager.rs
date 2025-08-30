use crate::notifiers::actor::{AddNotifier, GetUserNotifiers, NotifierActor, RemoveAllNotifiers, RemoveNotifier, SendContent, StartNotifierCycle, StopNotifierCycle};
use crate::{FetchResult, Manager, Notifier, SubscriptionCriteria, SubscriptionManager};
use actix::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
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

impl<T: Clone + Send + Sync + 'static, C: SubscriptionCriteria + Clone + 'static>
    NotifierManager<T, C>
where
    T: Clone + Into<C::Content> + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    pub fn new(subscription_manager: Arc<RwLock<SubscriptionManager<C>>>) -> Self {
        let actor = NotifierActor::new(subscription_manager);
        let actor_address = actor.start();

        Self { actor_address }
    }

    /// Send notifications for the given content to all matching subscribers
    pub async fn send_content(
        &self,
        content: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: Clone + Into<C::Content>,
    {
        self.actor_address
            .send(SendContent { content })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to send notifications: {}", e);
                Err(Box::new(e))
            })
    }

    /// Add a notifier for a specific user
    pub async fn add_notifier(
        &self,
        user_id: String,
        notifier: Arc<dyn Notifier<T> + Send + Sync>,
    ) {
        self.actor_address
            .send(AddNotifier { user_id, notifier })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to add notifier: {}", e);
            })
    }

    /// Remove a specific notifier for a user by name
    pub async fn remove_notifier(&self, user_id: String, notifier_name: String) {
        self.actor_address
            .send(RemoveNotifier {
                user_id,
                notifier_name,
            })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to remove notifier: {}", e);
            })
    }

    pub async fn get_user_notifiers(&self, user_id: &str) -> Vec<String> {
        self.actor_address
            .send(GetUserNotifiers { user_id: user_id.to_string() })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to get user notifiers: {}", e);
                Vec::new()
            })
    }

    /// Remove all notifiers for a specific user
    pub async fn remove_all_notifiers(&self, user_id: String) {
        self.actor_address
            .send(RemoveAllNotifiers { user_id })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to remove all notifiers: {}", e);
            })
    }

    /// Set the receiver for the notifier manager to receive data from fetchers
    pub fn set_receiver(&mut self, receiver: mpsc::UnboundedReceiver<FetchResult<T>>) {
        // Send the receiver to the actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(crate::notifiers::actor::SetReceiver { receiver })
                .await
                .unwrap_or_else(|e| {
                    error!("Failed to set receiver in notifier actor: {}", e);
                });
        });
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
        info!("NotifierManager started");
        // Send start message to actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(StartNotifierCycle).await.unwrap_or_else(|e| {
                error!("Failed to start fetch cycle: {}", e);
            });
        });
        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("NotifierManager stopped");
        // Send stop message to actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(StopNotifierCycle).await.unwrap_or_else(|e| {
                error!("Failed to stop fetch cycle: {}", e);
            });
        });
        Ok(())
    }
}
