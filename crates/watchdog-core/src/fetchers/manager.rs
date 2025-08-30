use crate::fetchers::actor::{
    AddFetcher, GetUserFetchers, RemoveFetcher, SetSender, StartFetchCycle, StopFetchCycle,
};
use crate::fetchers::FetcherActor;
use crate::{FetchResult, Fetcher, Manager};
use actix::prelude::*;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::error;

/// Manager for fetchers that runs them periodically using Actix actors
pub struct FetcherManager<T: Clone + Send + Sync + 'static> {
    actor_address: Addr<FetcherActor<T>>,
    sender: Option<mpsc::UnboundedSender<FetchResult<T>>>,
}

impl<T: Clone + Send + Sync + 'static> FetcherManager<T> {
    pub fn new(interval_duration: Duration, thread_count: usize) -> Self {
        let actor = FetcherActor::new(interval_duration, thread_count);
        let actor_address = actor.start();

        Self {
            actor_address,
            sender: None,
        }
    }

    /// Add a fetcher to the manager
    pub async fn add_fetcher(
        &self,
        user_id: &str,
        name: &str,
        fetcher: Box<dyn Fetcher<T> + Send + Sync>,
    ) {
        self.actor_address
            .send(AddFetcher {
                user_id: user_id.to_string(),
                name: name.to_string(),
                fetcher,
            })
            .await
            .unwrap_or_else(|e| error!("Failed to add fetcher: {}", e));
    }

    /// Remove a fetcher from the manager
    pub async fn remove_fetcher(&self, user_id: &str, name: &str) -> Option<()> {
        let result = self
            .actor_address
            .send(RemoveFetcher {
                user_id: user_id.to_string(),
                name: name.to_string(),
            })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to remove fetcher: {}", e);
                None
            });

        result
    }

    /// Get all fetcher of user
    pub async fn get_user_fetchers(&self, user_id: &str) -> Vec<String> {
        let result = self
            .actor_address
            .send(GetUserFetchers {
                user_id: user_id.to_string(),
            })
            .await
            .unwrap_or_else(|e| {
                error!("Failed to get user fetchers: {}", e);
                vec![]
            });

        result
    }

    /// Set the sender for the fetcher manager to send data to notifiers
    pub fn set_sender(&mut self, sender: mpsc::UnboundedSender<FetchResult<T>>) {
        self.sender = Some(sender.clone());
        // Send the sender to the actor
        let addr = self.actor_address.clone();
        actix::spawn(async move {
            addr.send(SetSender { sender }).await.unwrap_or_else(|e| {
                error!("Failed to set sender in fetcher actor: {}", e);
            });
        });
    }
}

impl<T> Manager for FetcherManager<T>
where
    T: Clone + Send + Sync + 'static,
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
