use std::collections::HashMap;
use std::hash::Hash;

/// A trait that defines the criteria for a subscription.
/// Implementors can define their own logic for matching content.
pub trait SubscriptionCriteria: Clone {
    /// The type of the unique identifier for this criteria
    type Id: Clone + Eq + Hash;
    
    /// The type of content that this criteria can match against
    type Content;

    /// Check if the content matches the subscription criteria
    fn matches(&self, content: &Self::Content) -> bool;

    /// Get the unique identifier for this criteria
    fn id(&self) -> &Self::Id;
}

/// Represents a user subscription with their criteria
#[derive(Debug, Clone)]
pub struct Subscription<C: SubscriptionCriteria> {
    pub user_id: String,
    pub criteria: C,
}

impl<C: SubscriptionCriteria> Subscription<C> {
    /// Create a new subscription
    pub fn new(user_id: String, criteria: C) -> Self {
        Self { user_id, criteria }
    }
}

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