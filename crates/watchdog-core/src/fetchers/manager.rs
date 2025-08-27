use crate::fetchers::actor::{AddFetcher, RemoveFetcher, StartFetchCycle, StopFetchCycle};
use crate::fetchers::FetcherActor;
use crate::storage::FetchStorage;
use crate::{Fetcher, Manager};
use actix::prelude::*;
use std::time::Duration;
use tracing::error;

/// Manager for fetchers that runs them periodically using Actix actors
pub struct FetcherManager<
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
> {
    actor_address: Addr<FetcherActor<T, S>>,
    storage: S,
}

impl<
        T: Clone + Send + Sync + 'static,
        S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
    > FetcherManager<T, S>
{
    pub fn new(interval_duration: Duration, storage: S, thread_count: usize) -> Self {
        let actor = FetcherActor::new(interval_duration, storage.clone(), thread_count);
        let actor_address = actor.start();

        Self {
            actor_address,
            storage,
        }
    }

    /// Add a fetcher to the manager
    pub async fn add_fetcher(&self, name: String, fetcher: Box<dyn Fetcher<T> + Send + Sync>) {
        self.actor_address
            .send(AddFetcher { name, fetcher })
            .await
            .unwrap_or_else(|e| error!("Failed to add fetcher: {}", e));
    }

    /// Remove a fetcher from the manager
    pub async fn remove_fetcher(&self, name: &str) -> Option<()> {
        let result = self
            .actor_address
            .send(RemoveFetcher {
                name: name.to_string(),
            })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to remove fetcher: {}", e);
                None
            });

        result
    }

    /// Get the storage for fetched data
    pub fn get_storage(&self) -> &S {
        &self.storage
    }
}

impl<T, S> Manager for FetcherManager<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
{
    fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send start message to actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(StartFetchCycle).await.unwrap_or_else(|e| {
                error!("Failed to start fetch cycle: {}", e);
            });
        });

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send stop message to actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(StopFetchCycle).await.unwrap_or_else(|e| {
                error!("Failed to stop fetch cycle: {}", e);
            });
        });

        Ok(())
    }
}
