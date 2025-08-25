use actix::prelude::*;
use std::time::Duration;
use watchdog::{
    fetcher::{FetchResult, Fetcher},
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

/// Message to shutdown the server
#[derive(Message)]
#[rtype(result = "()")]
pub struct ShutdownMsg;

/// Message to trigger content checking
#[derive(Message)]
#[rtype(result = "()")]
struct CheckContentMsg;

/// Actor that manages subscriptions and sends notifications
pub struct SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    config: ServerConfig,
    subscription_manager: SubscriptionManager<C>,
    fetcher: F,
    notifier: N,
}

impl<F, N, C> SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    pub fn new(config: ServerConfig, fetcher: F, notifier: N) -> Self {
        Self {
            config,
            subscription_manager: SubscriptionManager::new(),
            fetcher,
            notifier,
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

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Subscription server started");

        // Start the periodic content checking
        ctx.run_interval(Duration::from_secs(self.config.check_interval), |_, ctx| {
            ctx.notify(CheckContentMsg);
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Subscription server stopped");
    }
}

// Handle AddSubscription message
impl<F, N, C> Handler<AddSubscriptionMsg<C>> for SubscriptionServer<F, N, C>
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
        info!("Added subscription");
    }
}

// Handle RemoveSubscription message
impl<F, N, C> Handler<RemoveSubscriptionMsg<C>> for SubscriptionServer<F, N, C>
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
        info!("Removed subscription");
    }
}

// Handle CheckContent message
impl<F, N, C> Handler<CheckContentMsg> for SubscriptionServer<F, N, C>
where
    F: Fetcher<C::Content> + Clone + Unpin + 'static,
    N: Notifier<C::Content> + Clone + Unpin + 'static,
    C: SubscriptionCriteria + Clone + Unpin + 'static,
    C::Id: Clone + Eq + std::hash::Hash + Unpin + Send + Sync + 'static,
    C::Content: Clone + Unpin + Send + Sync + 'static,
{
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: CheckContentMsg, _ctx: &mut Self::Context) -> Self::Result {
        info!("Checking for new content");

        // Clone the fetcher and notifier to move into the async block
        let fetcher = self.fetcher.clone();
        let notifier = self.notifier.clone();
        let subscriptions = self.subscription_manager.get_subscriptions().clone();

        // Create an async future to fetch content and process it
        let fut = async move {
            match fetcher.fetch().await {
                Ok(fetch_result) => {
                    info!(
                        "Fetched content, checking against {} subscriptions",
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

                            info!("send to notifier");

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
                    error!("Failed to fetch content: {}", e);
                }
            }
        }
        .into_actor(self);

        Box::pin(fut)
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
        info!("Shutting down subscription server");
        ctx.stop();
    }
}
