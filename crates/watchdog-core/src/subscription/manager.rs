use std::collections::HashMap;
use crate::{Subscription, SubscriptionCriteria};

/// Manages a collection of subscriptions
#[derive(Debug)]
pub struct SubscriptionManager<C: SubscriptionCriteria> {
    subscriptions: HashMap<C::Id, Subscription<C>>,
}

impl<C: SubscriptionCriteria> SubscriptionManager<C> {
    /// Create a new subscription manager
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    /// Add a subscription to the manager
    pub fn add_subscription(&mut self, subscription: Subscription<C>) {
        self.subscriptions.insert(subscription.criteria.id().clone(), subscription);
    }

    /// Remove a subscription by its criteria ID
    pub fn remove_subscription(&mut self, id: &C::Id) -> Option<Subscription<C>> {
        self.subscriptions.remove(id)
    }

    /// Get a subscription by its criteria ID
    pub fn get_subscription(&self, id: &C::Id) -> Option<&Subscription<C>> {
        self.subscriptions.get(id)
    }

    /// Get all subscriptions
    pub fn get_subscriptions(&self) -> &HashMap<C::Id, Subscription<C>> {
        &self.subscriptions
    }

    /// Get all subscriptions that match the given content
    pub fn get_matching_subscriptions(&self, content: &C::Content) -> Vec<&Subscription<C>> {
        self.subscriptions
            .values()
            .filter(|subscription| subscription.criteria.matches(content))
            .collect()
    }
}

impl<C: SubscriptionCriteria> Default for SubscriptionManager<C> {
    fn default() -> Self {
        Self::new()
    }
}