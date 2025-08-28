use crate::{Notification, Notifier, SubscriptionManager};
use actix::prelude::*;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Message to send notifications
#[derive(Message)]
#[rtype(result = "Result<(), Box<dyn std::error::Error + Send + Sync>>")]
pub struct SendContent<T: Clone + Send + Sync + 'static> {
    pub content: T,
}

/// Message to add a notifier for a user
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddNotifier<T: Clone + Send + Sync + 'static> {
    pub user_id: String,
    pub notifier: Arc<dyn Notifier<T> + Send + Sync>,
}

/// Message to remove a specific notifier for a user
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveNotifier {
    pub user_id: String,
    pub notifier_name: String,
}

/// Message to remove all notifiers for a user
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveAllNotifiers {
    pub user_id: String,
}

/// Actor implementation for NotifierManager
pub struct NotifierActor<T: Clone, C: crate::SubscriptionCriteria + 'static>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
    T: Send + Sync + 'static,
{
    // <user_id, notifier>
    user_notifiers: DashMap<String, Vec<Arc<dyn Notifier<T> + Send + Sync>>>,
    subscription_manager: Arc<RwLock<SubscriptionManager<C>>>,
}

impl<T: Clone, C: crate::SubscriptionCriteria + 'static> NotifierActor<T, C>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
    T: Send + Sync + 'static,
{
    pub fn new(subscription_manager: Arc<RwLock<SubscriptionManager<C>>>) -> Self {
        Self {
            user_notifiers: DashMap::new(),
            subscription_manager,
        }
    }
}

// Required implementation for Actix actors
impl<T: Clone, C: crate::SubscriptionCriteria + 'static> Actor for NotifierActor<T, C>
where
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
    T: Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("NotifierActor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("NotifierActor stopped");
    }
}

// Handler implementations for messages
impl<T: Clone, C: crate::SubscriptionCriteria + 'static> Handler<SendContent<T>>
    for NotifierActor<T, C>
where
    T: Clone + Into<C::Content> + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    type Result = ResponseFuture<Result<(), Box<dyn std::error::Error + Send + Sync>>>;

    fn handle(&mut self, msg: SendContent<T>, _ctx: &mut Self::Context) -> Self::Result {
        let subscription_manager = self.subscription_manager.clone();
        let user_notifiers = self.user_notifiers.clone();
        let content = msg.content;

        Box::pin(async move {
            // Get matching subscriptions using the actor
            let matching_subscriptions = {
                let sm = subscription_manager.read().await;
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

                let user_id = subscription.user_id.clone();
                if let Some(notifiers) = user_notifiers.get(&user_id) {
                    for notifier in notifiers.iter() {
                        let notifier = notifier.clone();
                        let notification = notification.clone();
                        match notifier.send(notification).await {
                            Ok(()) => {
                                info!("Successfully sent notification to user {}", user_id);
                            }
                            Err(e) => {
                                error!("Failed to send notification to user {}: {}", user_id, e);
                            }
                        }
                    }
                } else {
                    info!("No notifiers found for user {}", user_id);
                }
            }

            Ok(())
        })
    }
}

impl<T: Clone, C: crate::SubscriptionCriteria + 'static> Handler<AddNotifier<T>>
    for NotifierActor<T, C>
where
    T: Clone + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: AddNotifier<T>, _ctx: &mut Self::Context) -> Self::Result {
        let user_id = msg.user_id;
        let notifier = msg.notifier.clone();

        self.user_notifiers
            .entry(user_id.clone())
            .or_insert_with(Vec::new)
            .push(msg.notifier);

        info!("Added notifier '{}' for user {}", notifier.name(), user_id);
    }
}

impl<T: Clone, C: crate::SubscriptionCriteria + 'static> Handler<RemoveNotifier>
    for NotifierActor<T, C>
where
    T: Clone + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: RemoveNotifier, _ctx: &mut Self::Context) -> Self::Result {
        let user_id = msg.user_id;
        let notifier_name = msg.notifier_name;
        
        if let Some(mut notifiers) = self.user_notifiers.get_mut(&user_id) {
            // Remove specific notifier by name
            notifiers.retain(|notifier| notifier.name() != notifier_name);
            
            // If no notifiers left for this user, remove the entry entirely
            if notifiers.is_empty() {
                self.user_notifiers.remove(&user_id);
            }
        }
        
        info!("Removed notifier '{}' for user {}", notifier_name, user_id);
    }
}

impl<T: Clone, C: crate::SubscriptionCriteria + 'static> Handler<RemoveAllNotifiers>
    for NotifierActor<T, C>
where
    T: Clone + Send + Sync + 'static,
    C::Content: Clone + std::fmt::Debug + Unpin + Send + Sync + 'static,
    C::Id: Send + Sync + Unpin + 'static,
    C: Send + Sync + Clone + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: RemoveAllNotifiers, _ctx: &mut Self::Context) -> Self::Result {
        let user_id = msg.user_id;
        
        self.user_notifiers.remove(&user_id);
        
        info!("Removed all notifiers for user {}", user_id);
    }
}
