//! Application state management
//!
//! This module defines the shared application state that is injected
//! into all route handlers.

use std::sync::Arc;
use watchdog_core::subscription::SubscriptionCriteria;
use watchdog_core::Watchdog;

/// Application state containing all shared services
pub struct AppState<T, C>
where
    T: Clone + Send + Sync + std::fmt::Debug + 'static + Unpin,
    C: SubscriptionCriteria<Content = T> + Send + Sync + Clone + std::fmt::Debug + 'static + Unpin,
    C::Id: Send + Sync + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static + Unpin,
{
    pub watchdog: Arc<Watchdog<T, C>>,
}

impl<T, C> AppState<T, C>
where
    T: Clone + Send + Sync + std::fmt::Debug + 'static + std::marker::Unpin,
    C: SubscriptionCriteria<Content = T>
        + Send
        + Sync
        + Clone
        + std::fmt::Debug
        + 'static
        + std::marker::Unpin,
    C::Id:
        Send + Sync + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static + std::marker::Unpin,
{
    pub fn new(watchdog: Arc<Watchdog<T, C>>) -> Self {
        Self { watchdog }
    }
}

impl<T, C> Clone for AppState<T, C>
where
    T: Clone + Send + Sync + std::fmt::Debug + 'static + Unpin,
    C: SubscriptionCriteria<Content = T> + Send + Sync + Clone + std::fmt::Debug + 'static + Unpin,
    C::Id: Send + Sync + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static + Unpin,
{
    fn clone(&self) -> Self {
        Self {
            watchdog: self.watchdog.clone(),
        }
    }
}
