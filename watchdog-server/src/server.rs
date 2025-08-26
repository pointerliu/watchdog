use actix::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use watchdog::{
    fetcher::Fetcher,
    notifier::{Notification, Notifier},
    subscription::{Subscription, SubscriptionCriteria, SubscriptionManager},
};
use tracing::{error, info};

/// Configuration for the subscription server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// How often to check for new content (in seconds)
    pub check_interval: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            check_interval: 1, // 5 minutes
        }
    }
}

/// Message to add a subscription
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddSubscriptionMsg<C: SubscriptionCriteria> {
    pub subscription: Subscription<C>,
}

/// Message to remove a subscription
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveSubscriptionMsg<C: SubscriptionCriteria> {
    pub id: C::Id,
}

/// Message to get a subscription
#[derive(Message)]
#[rtype(result = "Option<Subscription<C>>")]
pub struct GetSubscriptionMsg<C: SubscriptionCriteria + 'static> {
    pub id: C::Id,
}

/// Message to list all subscriptions
#[derive(Message)]
#[rtype(result = "Vec<Subscription<C>>")]
pub struct ListSubscriptionsMsg<C: SubscriptionCriteria + 'static>(pub std::marker::PhantomData<C>);

/// Message to shutdown the server
#[derive(Message)]
#[rtype(result = "()")]
pub struct ShutdownMsg;

/// Message to trigger content checking
#[derive(Message)]
#[rtype(result = "()")]
struct CheckContentMsg;

/// Message to add a user worker
#[derive(Message)]
#[rtype(result = "Result<Addr<SubscriptionWorker<F, N, C>>, String>")]
pub struct AddUserWorkerMsg<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub user_id: String,
    pub fetcher: F,
    pub notifier: N,
    pub phantom: std::marker::PhantomData<C>,
}

/// Message to get a user worker
#[derive(Message)]
#[rtype(result = "Option<Addr<SubscriptionWorker<F, N, C>>>")]
pub struct GetUserWorkerMsg<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub user_id: String,
    pub phantom: std::marker::PhantomData<(F, N, C)>,
}

/// Actor that manages subscriptions for a single user
pub struct SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    user_id: String,
    config: ServerConfig,
    subscription_manager: SubscriptionManager<C>,
    fetcher: F,
    notifier: N,
}

impl<F, N, C> SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub fn new(user_id: String, config: ServerConfig, fetcher: F, notifier: N) -> Self {
        Self {
            user_id,
            config,
            subscription_manager: SubscriptionManager::new(),
            fetcher,
            notifier,
        }
    }
}

impl<F, N, C> Actor for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Subscription worker for user {} started", self.user_id);

        // Start the periodic content checking
        ctx.run_interval(Duration::from_secs(self.config.check_interval), |_, ctx| {
            ctx.notify(CheckContentMsg);
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Subscription worker for user {} stopped", self.user_id);
    }
}

// Handle AddSubscription message
impl<F, N, C> Handler<AddSubscriptionMsg<C>> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: AddSubscriptionMsg<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscription_manager.add_subscription(msg.subscription);
        info!("Added subscription for user {}", self.user_id);
    }
}

// Handle RemoveSubscription message
impl<F, N, C> Handler<RemoveSubscriptionMsg<C>> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: RemoveSubscriptionMsg<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscription_manager.remove_subscription(&msg.id);
        info!("Removed subscription for user {}", self.user_id);
    }
}

// Handle GetSubscription message
impl<F, N, C> Handler<GetSubscriptionMsg<C>> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = Option<Subscription<C>>;

    fn handle(&mut self, msg: GetSubscriptionMsg<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscription_manager.get_subscription(&msg.id).cloned()
    }
}

// Handle ListSubscriptions message
impl<F, N, C> Handler<ListSubscriptionsMsg<C>> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = Vec<Subscription<C>>;

    fn handle(&mut self, _msg: ListSubscriptionsMsg<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscription_manager
            .get_subscriptions()
            .values()
            .cloned()
            .collect()
    }
}

// Handle CheckContent message
impl<F, N, C> Handler<CheckContentMsg> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: CheckContentMsg, _ctx: &mut Self::Context) -> Self::Result {
        info!("Checking for new content for user {}", self.user_id);

        // Clone the fetcher and notifier to move into the async block
        let fetcher = self.fetcher.clone();
        let notifier = self.notifier.clone();
        let subscriptions = self.subscription_manager.get_subscriptions().clone();
        let user_id = self.user_id.clone();

        // Create an async future to fetch content and process it
        let fut = async move {
            match fetcher.fetch().await {
                Ok(fetch_result) => {
                    info!(
                        "Fetched content for user {}, checking against {} subscriptions",
                        user_id,
                        subscriptions.len()
                    );

                    // Process each subscription
                    for subscription in subscriptions.values() {
                        if subscription.criteria.matches(&fetch_result.content) {
                            info!(
                                "Content matches subscription for user: {}",
                                subscription.user_id
                            );

                            // Create and send notification
                            let notification = Notification {
                                user_id: subscription.user_id.clone(),
                                title: "New Content Available".to_string(),
                                content: fetch_result.content.clone(),
                                timestamp: fetch_result.timestamp,
                            };

                            info!("Sending notification to user {}", subscription.user_id);

                            if let Err(e) = notifier.send(notification).await {
                                error!(
                                    "Failed to send notification to user {}: {}",
                                    subscription.user_id, e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to fetch content for user {}: {}", user_id, e);
                }
            }
        }
        .into_actor(self);

        Box::pin(fut)
    }
}

// Handle Shutdown message
impl<F, N, C> Handler<ShutdownMsg> for SubscriptionWorker<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ();

    fn handle(&mut self, _msg: ShutdownMsg, ctx: &mut Self::Context) -> Self::Result {
        info!("Shutting down subscription worker for user {}", self.user_id);
        ctx.stop();
    }
}

/// Actor that manages multiple subscription workers, one per user
pub struct SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    config: ServerConfig,
    user_workers: HashMap<String, Addr<SubscriptionWorker<F, N, C>>>,
}

impl<F, N, C> SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            user_workers: HashMap::new(),
        }
    }
}

impl<F, N, C> Actor for SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Subscription server started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Subscription server stopped");
    }
}

// Handle AddUserWorker message
impl<F, N, C> Handler<AddUserWorkerMsg<F, N, C>> for SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = Result<Addr<SubscriptionWorker<F, N, C>>, String>;

    fn handle(&mut self, msg: AddUserWorkerMsg<F, N, C>, _ctx: &mut Self::Context) -> Self::Result {
        if self.user_workers.contains_key(&msg.user_id) {
            return Err(format!("User worker for {} already exists", msg.user_id));
        }

        let worker = SubscriptionWorker::new(
            msg.user_id.clone(),
            self.config.clone(),
            msg.fetcher,
            msg.notifier,
        );
        let addr = worker.start();
        self.user_workers.insert(msg.user_id.clone(), addr.clone());

        info!("Added user worker for {}", msg.user_id);
        Ok(addr)
    }
}

// Handle GetUserWorker message
impl<F, N, C> Handler<GetUserWorkerMsg<F, N, C>> for SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = Option<Addr<SubscriptionWorker<F, N, C>>>;

    fn handle(&mut self, msg: GetUserWorkerMsg<F, N, C>, _ctx: &mut Self::Context) -> Self::Result {
        self.user_workers.get(&msg.user_id).cloned()
    }
}

// Handle Shutdown message
impl<F, N, C> Handler<ShutdownMsg> for SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ();

    fn handle(&mut self, _msg: ShutdownMsg, ctx: &mut Self::Context) -> Self::Result {
        info!("Shutting down subscription server and all user workers");
        for (user_id, worker) in self.user_workers.iter() {
            info!("Shutting down worker for user {}", user_id);
            worker.do_send(ShutdownMsg);
        }
        ctx.stop();
    }
}