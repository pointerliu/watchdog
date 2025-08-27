use std::collections::HashMap;
use crate::{Subscription, SubscriptionCriteria};
use actix::prelude::*;

/// Manages a collection of subscriptions
#[derive(Debug)]
pub struct SubscriptionManager<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    actor_address: Addr<crate::subscription::actor::SubscriptionActor<C>>,
}

impl<C: SubscriptionCriteria + 'static> SubscriptionManager<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    /// Create a new subscription manager
    pub fn new() -> Self {
        let actor = crate::subscription::actor::SubscriptionActor::<C>::new();
        let actor_address = actor.start();
        
        Self {
            actor_address,
        }
    }

    /// Add a subscription to the manager
    pub async fn add_subscription(&self, subscription: Subscription<C>) {
        self.actor_address
            .send(crate::subscription::actor::AddSubscription { subscription })
            .await
            .unwrap_or_else(|e| tracing::error!("Failed to add subscription: {}", e));
    }

    /// Remove a subscription by its criteria ID
    pub async fn remove_subscription(&self, id: C::Id) -> Option<Subscription<C>> {
        self.actor_address
            .send(crate::subscription::actor::RemoveSubscription { id })
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to remove subscription: {}", e);
                None
            })
    }

    /// Get a subscription by its criteria ID
    pub async fn get_subscription(&self, id: C::Id) -> Option<Subscription<C>> {
        self.actor_address
            .send(crate::subscription::actor::GetSubscription { id })
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to get subscription: {}", e);
                None
            })
    }

    /// Get all subscriptions
    pub async fn get_subscriptions(&self) -> HashMap<C::Id, Subscription<C>> {
        self.actor_address
            .send(crate::subscription::actor::GetAllSubscriptions::<C>::new())
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to get subscriptions: {}", e);
                HashMap::new()
            })
    }

    /// Get all subscriptions that match the given content
    pub async fn get_matching_subscriptions(&self, content: C::Content) -> Vec<Subscription<C>> {
        self.actor_address
            .send(crate::subscription::actor::GetMatchingSubscriptions { content })
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to get matching subscriptions: {}", e);
                Vec::new()
            })
    }
    
    /// Get the actor address for direct access
    pub fn get_actor_address(&self) -> Addr<crate::subscription::actor::SubscriptionActor<C>> {
        self.actor_address.clone()
    }
}

impl<C: SubscriptionCriteria + 'static> Default for SubscriptionManager<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}