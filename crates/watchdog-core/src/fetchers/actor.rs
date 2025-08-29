use crate::storage::FetchStorage;
use crate::{FetchResult, Fetcher};
use actix::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
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

/// Message to set the sender for sending fetched data to notifiers
#[derive(Message)]
#[rtype(result = "()")]
pub struct SetSender<T: Clone + Send + Sync + 'static> {
    pub sender: mpsc::UnboundedSender<FetchResult<T>>,
}

/// Actor implementation for FetcherManager
pub struct FetcherActor<T, S> {
    fetchers: Arc<RwLock<HashMap<String, Box<dyn Fetcher<T> + Send + Sync>>>>,
    storage: S,
    interval_duration: Duration,
    running: bool,
    thread_count: usize,
    sender: Option<mpsc::UnboundedSender<FetchResult<T>>>,
}

impl<T, S> FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
{
    pub fn new(interval_duration: Duration, storage: S, thread_count: usize) -> Self {
        Self {
            fetchers: Arc::new(RwLock::new(HashMap::new())),
            storage,
            interval_duration,
            running: false,
            thread_count,
            sender: None,
        }
    }
}

// Required implementation for Actix actors
impl<T, S> Actor for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("FetcherActor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("FetcherActor stopped");
    }
}

// Handler implementations for messages
impl<T, S> Handler<AddFetcher<T>> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
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

impl<T, S> Handler<RemoveFetcher> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
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

impl<T, S> Handler<StartFetchCycle> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
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

impl<T, S> Handler<StopFetchCycle> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, _msg: StopFetchCycle, ctx: &mut Self::Context) -> Self::Result {
        self.running = false;
        // Stop the interval by canceling the context
        ctx.stop();
    }
}

impl<T, S> Handler<SetSender<T>> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: SetSender<T>, _ctx: &mut Self::Context) -> Self::Result {
        self.sender = Some(msg.sender);
    }
}

impl<T, S> Handler<RunFetchCycle> for FetcherActor<T, S>
where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + Unpin + 'static,
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
        let thread_count = self.thread_count;
        let sender = self.sender.clone();

        // Spawn the async fetch cycle
        actix::spawn(async move {
            run_fetch_cycle(fetchers, storage, thread_count, sender).await;
        });
    }
}

/// Run the fetch cycle once
pub(crate) async fn run_fetch_cycle<T, S>(
    fetchers: Arc<RwLock<HashMap<String, Box<dyn Fetcher<T> + Send + Sync>>>>,
    storage: S,
    thread_count: usize,
    sender: Option<mpsc::UnboundedSender<FetchResult<T>>>,
) where
    T: Clone + Send + Sync + 'static,
    S: FetchStorage<T> + Clone + Send + Sync + 'static,
{
    let fetcher_names: Vec<String> = {
        let fetchers = fetchers.read().await;
        fetchers.keys().cloned().collect()
    };

    info!(
        "Running fetch cycle for {} fetchers with {} threads",
        fetcher_names.len(),
        thread_count
    );

    // Create a semaphore to limit concurrent fetch operations
    let semaphore = Arc::new(tokio::sync::Semaphore::new(thread_count));

    // Create a vector to hold all the fetch tasks
    let mut fetch_tasks = Vec::new();

    // Spawn a task for each fetcher
    for name in fetcher_names {
        let fetchers = fetchers.clone();
        let storage = storage.clone();
        let semaphore = semaphore.clone();
        let sender = sender.clone();
        let name_clone = name.clone();

        let task = actix::spawn(async move {
            // Acquire a permit from the semaphore
            let _permit = semaphore.acquire().await.unwrap();

            let fetch_result = {
                let fetchers = fetchers.read().await;
                if let Some(fetcher) = fetchers.get(&name) {
                    match fetcher.fetch().await {
                        Ok(result) => {
                            info!("Successfully fetched data from {}", name);
                            Some((name.clone(), result)) // Return name with result
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

            // Store the result if successful and return success status
            let success = if let Some((_name, result)) = fetch_result {
                storage.store(result.clone()).await;

                // Send the result to notifiers if sender is available
                if let Some(sender) = &sender {
                    if let Err(e) = sender.send(result) {
                        error!("Failed to send fetched data to notifiers: {}", e);
                    }
                }

                true
            } else {
                false
            };

            (name_clone, success)
        });

        fetch_tasks.push(task);
    }

    // Wait for all fetch tasks to complete
    let results = futures::future::join_all(fetch_tasks).await;

    let successful_fetches = results
        .iter()
        .filter(|r| r.as_ref().is_ok_and(|(_, success)| *success))
        .count();
    info!(
        "Fetch cycle completed. {} out of {} fetchers succeeded",
        successful_fetches,
        results.len()
    );
}
