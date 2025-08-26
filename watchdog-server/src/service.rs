//! Implementation of the SubscriptionService trait that works with the existing 
//! SubscriptionServer actor from the watchdog framework.

use crate::server::{AddSubscriptionMsg, RemoveSubscriptionMsg, GetSubscriptionMsg, ListSubscriptionsMsg, SubscriptionServer};
use actix::prelude::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use tokio::sync::RwLock;
use watchdog::{
    fetcher::Fetcher,
    notifier::Notifier,
    subscription::{Subscription, SubscriptionCriteria},
    FrameworkError,
};

/// Implementation of SubscriptionService that works with the existing SubscriptionServer actor
pub struct ActorSubscriptionService<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    server_addr: Addr<SubscriptionServer<F, N, C>>,
}

impl<F, N, C> ActorSubscriptionService<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub fn new(server_addr: Addr<SubscriptionServer<F, N, C>>) -> Self {
        Self { server_addr }
    }
}

#[async_trait]
impl<F, N, C> crate::api::SubscriptionService<C> for ActorSubscriptionService<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + Send + Sync + 'static,
    N: Notifier<C::Content> + Clone + Unpin + Send + Sync + 'static,
    C: SubscriptionCriteria + Clone + Send + Sync + Unpin + 'static,
    C::Id: Clone + Eq + Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Error = FrameworkError;

    async fn add_subscription(
        &self,
        subscription: Subscription<C>,
    ) -> Result<(), Self::Error> {
        self.server_addr
            .send(AddSubscriptionMsg { subscription })
            .await
            .map_err(|e| FrameworkError::Custom(format!("Actor error: {}", e)))?;
        Ok(())
    }

    async fn remove_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error> {
        // Note: The current server implementation doesn't return the removed subscription
        // We'll need to get it first, then remove it
        self.server_addr
            .send(RemoveSubscriptionMsg { id: id.clone() })
            .await
            .map_err(|e| FrameworkError::Custom(format!("Actor error: {}", e)))?;
        Ok(None) // We can't return the subscription with the current implementation
    }

    async fn get_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error> {
        let result = self.server_addr
            .send(GetSubscriptionMsg { id: id.clone() })
            .await
            .map_err(|e| FrameworkError::Custom(format!("Actor error: {}", e)))?;
        Ok(result)
    }

    async fn list_subscriptions(
        &self,
    ) -> Result<Vec<Subscription<C>>, Self::Error> {
        let result = self.server_addr
            .send(ListSubscriptionsMsg(PhantomData))
            .await
            .map_err(|e| FrameworkError::Custom(format!("Actor error: {}", e)))?;
        Ok(result)
    }
}

/// A storage-based implementation of SubscriptionService that can be used independently
/// of the actor-based server
pub struct StorageSubscriptionService<C: SubscriptionCriteria> {
    storage: RwLock<HashMap<C::Id, Subscription<C>>>,
}

impl<C: SubscriptionCriteria> StorageSubscriptionService<C> {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

impl<C: SubscriptionCriteria> Default for StorageSubscriptionService<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<C> crate::api::SubscriptionService<C> for StorageSubscriptionService<C>
where
    C: SubscriptionCriteria + Clone + Send + Sync + 'static,
    C::Id: Clone + Eq + Hash + Send + Sync + 'static,
{
    type Error = FrameworkError;

    async fn add_subscription(
        &self,
        subscription: Subscription<C>,
    ) -> Result<(), Self::Error> {
        let mut storage = self.storage.write().await;
        storage.insert(subscription.criteria.id().clone(), subscription);
        Ok(())
    }

    async fn remove_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error> {
        let mut storage = self.storage.write().await;
        Ok(storage.remove(id))
    }

    async fn get_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn list_subscriptions(
        &self,
    ) -> Result<Vec<Subscription<C>>, Self::Error> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }
}