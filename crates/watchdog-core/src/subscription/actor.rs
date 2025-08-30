use crate::{Subscription, SubscriptionCriteria};
use actix::prelude::*;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Message to add a subscription
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddSubscription<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub subscription: Subscription<C>,
}

/// Message to remove a subscription
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveSubscription<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub user_id: String,
    pub id: C::Id,
}

/// Message to get all subscriptions of a user
#[derive(Message)]
#[rtype(result = "Vec<C::Id>")]
pub struct GetUserSubscription<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub user_id: String,
    pub(crate) _phantom: PhantomData<C>,
}

/// Message to get a subscription
#[derive(Message)]
#[rtype(result = "Option<Subscription<C>>")]
pub struct GetSubscription<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub id: C::Id,
}

/// Message to get all subscriptions
#[derive(Message)]
#[rtype(result = "HashMap<C::Id, Subscription<C>>")]
pub struct GetAllSubscriptions<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    // This message type parameter is needed for the macro
    _phantom: std::marker::PhantomData<C>,
}

impl<C: SubscriptionCriteria + 'static> Default for GetAllSubscriptions<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: SubscriptionCriteria + 'static> GetAllSubscriptions<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Message to get matching subscriptions
#[derive(Message)]
#[rtype(result = "Vec<Subscription<C>>")]
pub struct GetMatchingSubscriptions<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub content: C::Content,
}

/// Actor implementation for SubscriptionManager
pub struct SubscriptionActor<C: SubscriptionCriteria + 'static>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    subscriptions: HashMap<C::Id, Subscription<C>>,
}

impl<C: SubscriptionCriteria + 'static> SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
}

// Required implementation for Actix actors
impl<C: SubscriptionCriteria + 'static> Actor for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("SubscriptionActor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("SubscriptionActor stopped");
    }
}

// Handler implementations for messages
impl<C: SubscriptionCriteria + 'static> Handler<AddSubscription<C>> for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: AddSubscription<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscriptions
            .insert(msg.subscription.criteria.id().clone(), msg.subscription);
    }
}

impl<C: SubscriptionCriteria + 'static> Handler<RemoveSubscription<C>> for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = MessageResult<RemoveSubscription<C>>;

    fn handle(&mut self, msg: RemoveSubscription<C>, _ctx: &mut Self::Context) -> Self::Result {
        self.subscriptions
            .retain(|sub_id, sub| !(sub.user_id == msg.user_id && sub_id == &msg.id));
        self.subscriptions.remove(&msg.id);
        MessageResult(())
    }
}

impl<C: SubscriptionCriteria + 'static> Handler<GetUserSubscription<C>> for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = MessageResult<GetUserSubscription<C>>;

    fn handle(&mut self, msg: GetUserSubscription<C>, _ctx: &mut Self::Context) -> Self::Result {
        let data = self
            .subscriptions
            .iter()
            .filter_map(|(id, sub)| {
                if sub.user_id == msg.user_id {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        MessageResult(data)
    }
}

impl<C: SubscriptionCriteria + 'static> Handler<GetSubscription<C>> for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = MessageResult<GetSubscription<C>>;

    fn handle(&mut self, msg: GetSubscription<C>, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.subscriptions.get(&msg.id).cloned())
    }
}

impl<C: SubscriptionCriteria + 'static> Handler<GetAllSubscriptions<C>> for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = MessageResult<GetAllSubscriptions<C>>;

    fn handle(&mut self, _msg: GetAllSubscriptions<C>, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.subscriptions.clone())
    }
}

impl<C: SubscriptionCriteria + 'static> Handler<GetMatchingSubscriptions<C>>
    for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    type Result = MessageResult<GetMatchingSubscriptions<C>>;

    fn handle(
        &mut self,
        msg: GetMatchingSubscriptions<C>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let matching: Vec<Subscription<C>> = self
            .subscriptions
            .values()
            .filter(|subscription| subscription.criteria.matches(&msg.content))
            .cloned()
            .collect();
        MessageResult(matching)
    }
}

impl<C: SubscriptionCriteria + 'static> Default for SubscriptionActor<C>
where
    C::Id: Unpin + Send + Sync + 'static,
    C::Content: Unpin + Send + Sync + 'static,
    C: Unpin + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
