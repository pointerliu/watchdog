use crate::storage::FetchStorage;
use crate::{Fetcher, Manager};
use actix::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Messages for the FetcherManager actor

/// Message to add a fetcher
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddFetcher<T: Clone + Send + Sync + 'static> {
    pub name: String,
    pub fetcher: Box<dyn Fetcher<T> + Send + Sync>,
}

/// Message to remove a fetcher
#[derive(Message)]
#[rtype(result = "Option<()>")]
pub struct RemoveFetcher {
    pub name: String,
}

/// Message to start the fetch cycle
#[derive(Message)]
#[rtype(result = "()")]
pub struct StartFetchCycle;

/// Message to stop the fetch cycle
#[derive(Message)]
#[rtype(result = "()")]
pub struct StopFetchCycle;

/// Message to run a single fetch cycle
#[derive(Message)]
#[rtype(result = "()")]
struct RunFetchCycle;

/// Actor implementation for FetcherManager
pub struct FetcherActor<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> {
    fetchers: Arc<RwLock<HashMap<String, Box<dyn Fetcher<T> + Send + Sync>>>>,
    storage: S,
    interval_duration: Duration,
    running: bool,
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> FetcherActor<T, S> {
    pub fn new(interval_duration: Duration, storage: S) -> Self {
        Self {
            fetchers: Arc::new(RwLock::new(HashMap::new())),
            storage,
            interval_duration,
            running: false,
        }
    }
}

// Required implementation for Actix actors
impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> Actor for FetcherActor<T, S> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("FetcherActor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("FetcherActor stopped");
    }
}

// Handler implementations for messages
impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Handler<AddFetcher<T>> for FetcherActor<T, S> 
{
    type Result = ();

    fn handle(&mut self, msg: AddFetcher<T>, _ctx: &mut Self::Context) -> Self::Result {
        let fetchers = self.fetchers.clone();
        let name = msg.name;
        let fetcher = msg.fetcher;
        
        // Handle async operation in a spawn
        actix::spawn(async move {
            let mut fetchers = fetchers.write().await;
            fetchers.insert(name, fetcher);
        });
    }
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Handler<RemoveFetcher> for FetcherActor<T, S> 
{
    type Result = MessageResult<RemoveFetcher>;

    fn handle(&mut self, msg: RemoveFetcher, _ctx: &mut Self::Context) -> Self::Result {
        let fetchers = self.fetchers.clone();
        let name = msg.name;
        
        actix::spawn(async move {
            let mut fetchers = fetchers.write().await;
            fetchers.remove(&name);
        });
        
        MessageResult(Some(()))
    }
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Handler<StartFetchCycle> for FetcherActor<T, S> 
{
    type Result = ();

    fn handle(&mut self, _msg: StartFetchCycle, ctx: &mut Self::Context) -> Self::Result {
        self.running = true;
        
        // Start the periodic fetch cycle
        ctx.run_interval(self.interval_duration, |_act, ctx| {
            ctx.notify(RunFetchCycle);
        });
    }
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Handler<StopFetchCycle> for FetcherActor<T, S> 
{
    type Result = ();

    fn handle(&mut self, _msg: StopFetchCycle, ctx: &mut Self::Context) -> Self::Result {
        self.running = false;
        // Stop the interval by canceling the context
        ctx.stop();
    }
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Handler<RunFetchCycle> for FetcherActor<T, S> 
{
    type Result = ();

    fn handle(&mut self, _msg: RunFetchCycle, _ctx: &mut Self::Context) -> Self::Result {
        // Only run if we're in a running state
        if !self.running {
            return;
        }
        
        // Clone necessary data for the async task
        let fetchers = self.fetchers.clone();
        let storage = self.storage.clone();
        
        // Spawn the async fetch cycle
        actix::spawn(async move {
            run_fetch_cycle(fetchers, storage).await;
        });
    }
}

/// Run the fetch cycle once
async fn run_fetch_cycle<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync>(
    fetchers: Arc<RwLock<HashMap<String, Box<dyn Fetcher<T> + Send + Sync>>>>,
    storage: S,
) {
    let fetcher_names: Vec<String> = {
        let fetchers = fetchers.read().await;
        fetchers.keys().cloned().collect()
    };

    info!("Running fetch cycle for {} fetchers", fetcher_names.len());

    for name in fetcher_names {
        let fetch_result = {
            let fetchers = fetchers.read().await;
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
            storage.store(result).await;
        }
    }
}

/// Manager for fetchers that runs them periodically using Actix actors
pub struct FetcherManager<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> {
    actor_address: Addr<FetcherActor<T, S>>,
    storage: S,
}

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    FetcherManager<T, S> 
{
    pub fn new(interval_duration: Duration, storage: S) -> Self {
        let actor = FetcherActor::new(interval_duration, storage.clone());
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
        let result = self.actor_address
            .send(RemoveFetcher { name: name.to_string() })
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

impl<T: Clone + Send + Sync + 'static, S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static> 
    Manager for FetcherManager<T, S> 
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