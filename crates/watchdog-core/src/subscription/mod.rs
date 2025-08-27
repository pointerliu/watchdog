use std::hash::Hash;

pub mod manager;
pub mod actor;

pub use manager::SubscriptionManager;
pub use actor::SubscriptionActor;

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
