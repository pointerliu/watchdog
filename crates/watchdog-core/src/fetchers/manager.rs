use crate::storage::FetchStorage;
use crate::{Fetcher, Manager, Notification, Notifier, SubscriptionCriteria, SubscriptionManager};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{error, info};

/// Manager for fetchers that runs them periodically
pub struct FetcherManager<T: Clone, S: FetchStorage<T>> {
    fetchers: Arc<RwLock<HashMap<String, Box<dyn Fetcher<T> + Send + Sync>>>>,
    storage: S,
    interval_duration: Duration,
    running: Arc<RwLock<bool>>,
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T>> FetcherManager<T, S> {
    pub fn new(interval_duration: Duration, storage: S) -> Self {
        Self {
            fetchers: Arc::new(RwLock::new(HashMap::new())),
            storage,
            interval_duration,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Add a fetcher to the manager
    pub async fn add_fetcher(&self, name: String, fetcher: Box<dyn Fetcher<T> + Send + Sync>) {
        let mut fetchers = self.fetchers.write().await;
        fetchers.insert(name, fetcher);
    }

    /// Remove a fetcher from the manager
    pub async fn remove_fetcher(&self, name: &str) -> Option<Box<dyn Fetcher<T> + Send + Sync>> {
        let mut fetchers = self.fetchers.write().await;
        fetchers.remove(name)
    }

    /// Get the storage for fetched data
    pub fn get_storage(&self) -> &S {
        &self.storage
    }

    /// Run the fetch cycle once
    async fn run_fetch_cycle(&self) {
        let fetcher_names: Vec<String> = {
            let fetchers = self.fetchers.read().await;
            fetchers.keys().cloned().collect()
        };

        info!("Running fetch cycle for {} fetchers", fetcher_names.len());

        for name in fetcher_names {
            let fetch_result = {
                let fetchers = self.fetchers.read().await;
                if let Some(fetcher) = fetchers.get(&name) {
                    match fetcher.fetch().await {
                        Ok(result) => {
                            info!("Successfully fetched data from {}", name);
                            Some(result)
                        }
                        Err(e) => {
                            error!("Failed to fetch from {}: {}", name, e);
                            None
                        }
                    }
                } else {
                    None
                }
            };

            // Store the result if successful
            if let Some(result) = fetch_result {
                self.storage.store(result).await;
            }
        }
    }
}

#[async_trait::async_trait]
impl<T, S> Manager for FetcherManager<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: Clone + FetchStorage<T> + Send + Sync + 'static,
{
    fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Clone necessary data for the task
        let fetchers = self.fetchers.clone();
        let storage = self.storage.clone();
        let interval_duration = self.interval_duration;
        let running = self.running.clone();

        // Spawn the background task
        tokio::spawn(async move {
            // Set running flag
            {
                let mut running_guard = running.write().await;
                *running_guard = true;
            }

            let mut interval = interval(interval_duration);

            info!(
                "FetcherManager started with interval {:?}",
                interval_duration
            );

            loop {
                // Check if we should still be running
                {
                    let running_guard = running.read().await;
                    if !*running_guard {
                        break;
                    }
                }

                interval.tick().await;

                // Run the fetch cycle
                let manager = FetcherManager {
                    fetchers: fetchers.clone(),
                    storage: storage.clone(),
                    interval_duration,
                    running: running.clone(),
                };

                manager.run_fetch_cycle().await;
            }

            info!("FetcherManager stopped");
        });

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let running = self.running.clone();
        tokio::spawn(async move {
            let mut running_guard = running.write().await;
            *running_guard = false;
        });

        Ok(())
    }
}
