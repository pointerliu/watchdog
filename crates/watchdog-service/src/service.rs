//! Implementation of the SubscriptionService trait that works with the new 
//! SubscriptionServer actor from the watchdog framework.

use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::RwLock;
use watchdog_core::{
    subscription::{Subscription, SubscriptionCriteria},
    FrameworkError,
};

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

#[async_trait::async_trait]
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