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

                let user_id = subscription.user_id;
                let notifiers = self
                    .user_notifiers
                    .get(&user_id)
                    .expect("There is a user, no notifiers.");

                for notifer in notifiers.iter() {
                    let notifier = notifer.clone();
                    let notification = notification.clone();
                    match notifier.send(notification).await {
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
            }

            Ok(())
        })
    }
}
