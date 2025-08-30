//! Main Watchdog system that orchestrates fetchers, notifiers, and subscriptions
//!
//! This module provides a unified interface to configure and run the entire watchdog system,
//! connecting all components together in a cohesive manner.

use crate::fetchers::{Fetcher, FetcherManager};
use crate::notifiers::{Notifier, NotifierManager};
use crate::subscription::{Subscription, SubscriptionCriteria, SubscriptionManager};
use crate::FrameworkError;
use crate::Manager;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};

/// Configuration for the Watchdog system
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// How often fetchers should run
    pub fetch_interval: Duration,
    /// Number of worker threads for fetchers
    pub fetch_worker_threads: usize,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            fetch_interval: Duration::from_secs(60), // 1 minute default
            fetch_worker_threads: 4,
        }
    }
}

/// The main Watchdog system that orchestrates all components
///
/// This struct provides a high-level API for managing the entire watchdog system.
/// It connects fetchers, notifiers, and subscriptions together.
pub struct Watchdog<T, C>
where
    T: Clone + Send + Sync + Debug + 'static + Unpin,
    C: SubscriptionCriteria<Content = T> + Send + Sync + Clone + Debug + 'static + Unpin,
    C::Id: Send + Sync + Hash + Eq + Clone + Debug + 'static,
    <C as SubscriptionCriteria>::Id: Unpin,
{
    fetcher_manager: FetcherManager<T>,
    notifier_manager: NotifierManager<T, C>,
    subscription_manager: Arc<RwLock<SubscriptionManager<C>>>,
}

impl<T, C> Watchdog<T, C>
where
    T: Clone + Send + Sync + Debug + 'static + Unpin,
    C: SubscriptionCriteria<Content = T> + Send + Sync + Clone + Debug + 'static + Unpin,
    C::Id: Send + Sync + Hash + Eq + Clone + Debug + 'static,
    <C as SubscriptionCriteria>::Id: Unpin,
{
    /// Create a new Watchdog system with the given configuration
    pub fn new(config: WatchdogConfig) -> Self {
        // Create communication channel between fetchers and notifiers
        let (sender, receiver) = mpsc::unbounded_channel();

        // Create subscription manager
        let subscription_manager = Arc::new(RwLock::new(SubscriptionManager::new()));

        // Create fetcher manager
        let mut fetcher_manager =
            FetcherManager::new(config.fetch_interval, config.fetch_worker_threads);
        fetcher_manager.set_sender(sender);

        // Create notifier manager
        let mut notifier_manager = NotifierManager::new(subscription_manager.clone());
        notifier_manager.set_receiver(receiver);

        Self {
            fetcher_manager,
            notifier_manager,
            subscription_manager,
        }
    }

    /// Add a fetcher to the system
    pub async fn add_fetcher(
        &self,
        user_id: &str,
        name: &str,
        fetcher: Box<dyn Fetcher<T> + Send + Sync>,
    ) -> Result<(), FrameworkError> {
        self.fetcher_manager
            .add_fetcher(user_id, name, fetcher)
            .await;
        Ok(())
    }

    /// Remove a fetcher for user
    pub async fn remove_fetcher(&self, user_id: &str, name: &str) -> Result<(), FrameworkError> {
        self.fetcher_manager.remove_fetcher(user_id, name).await;
        Ok(())
    }

    /// Get all fetchers of user
    pub async fn get_user_fetchers(&self, user_id: &str) -> Result<Vec<String>, FrameworkError> {
        let fetchers = self.fetcher_manager.get_user_fetchers(user_id).await;
        Ok(fetchers)
    }

    /// Add a notifier for a specific user
    pub async fn add_notifier(
        &self,
        user_id: String,
        notifier: Arc<dyn Notifier<T> + Send + Sync>,
    ) -> Result<(), FrameworkError> {
        self.notifier_manager.add_notifier(user_id, notifier).await;
        Ok(())
    }

    /// Remove a notifier for a specific user
    pub async fn remove_notifier(
        &self,
        user_id: &str,
        notifier_name: &str,
    ) -> Result<(), FrameworkError> {
        self.notifier_manager
            .remove_notifier(user_id, notifier_name)
            .await;
        Ok(())
    }

    pub async fn get_user_notifiers(&self, user_id: &str) -> Result<Vec<String>, FrameworkError> {
        let res = self.notifier_manager.get_user_notifiers(user_id).await;
        Ok(res)
    }

    /// Add a subscription to the system
    pub async fn add_subscription(
        &self,
        subscription: Subscription<C>,
    ) -> Result<(), FrameworkError> {
        self.subscription_manager
            .write()
            .await
            .add_subscription(subscription)
            .await;
        Ok(())
    }

    /// Remove a subscription from the system
    pub async fn remove_subscription(
        &self,
        user_id: &str,
        criteria_id: &C::Id,
    ) -> Result<(), FrameworkError> {
        let result = self
            .subscription_manager
            .write()
            .await
            .remove_subscription(user_id, criteria_id)
            .await;
        Ok(result)
    }

    /// Get all subscriptions of user
    pub async fn get_user_subscriptions(&self, user_id: &str) -> Vec<C::Id> {
        let res = self
            .subscription_manager
            .read()
            .await
            .get_user_subscriptions(user_id)
            .await;
        res
    }

    /// Get subscriptions by subscription_id
    pub async fn get_subscription_by_id(&self, subscription_id: &C::Id) -> Option<Subscription<C>> {
        let res = self
            .subscription_manager
            .read()
            .await
            .get_subscription(subscription_id.clone())
            .await;
        res
    }

    /// Start the watchdog system
    pub fn start(&self) -> Result<(), FrameworkError> {
        // Start fetcher manager
        self.fetcher_manager.start().map_err(|e| {
            FrameworkError::Custom(format!("Failed to start fetcher manager: {}", e))
        })?;

        // Start notifier manager
        self.notifier_manager.start().map_err(|e| {
            FrameworkError::Custom(format!("Failed to start notifier manager: {}", e))
        })?;

        tracing::info!("Watchdog system started");
        Ok(())
    }

    /// Stop the watchdog system
    pub fn stop(&self) -> Result<(), FrameworkError> {
        // Stop fetcher manager
        self.fetcher_manager.stop().map_err(|e| {
            FrameworkError::Custom(format!("Failed to stop fetcher manager: {}", e))
        })?;

        // Stop notifier manager
        self.notifier_manager.stop().map_err(|e| {
            FrameworkError::Custom(format!("Failed to stop notifier manager: {}", e))
        })?;

        tracing::info!("Watchdog system stopped");
        Ok(())
    }

    /// Get a reference to the subscription manager for advanced operations
    pub fn subscription_manager(&self) -> Arc<RwLock<SubscriptionManager<C>>> {
        self.subscription_manager.clone()
    }

    /// Get a reference to the fetcher manager for advanced operations
    pub fn fetcher_manager(&self) -> &FetcherManager<T> {
        &self.fetcher_manager
    }

    /// Get a reference to the notifier manager for advanced operations
    pub fn notifier_manager(&self) -> &NotifierManager<T, C> {
        &self.notifier_manager
    }
}

impl<T, C> Watchdog<T, C>
where
    T: Clone + Send + Sync + Debug + 'static + std::marker::Unpin,
    C: SubscriptionCriteria<Content = T>
        + Send
        + Sync
        + Clone
        + Debug
        + 'static
        + std::marker::Unpin,
    C::Id: Send + Sync + Hash + Eq + Clone + Debug + 'static,
    <C as SubscriptionCriteria>::Id: Unpin,
{
    /// Create a new Watchdog with default configuration
    pub fn with_defaults() -> Self {
        Self::new(WatchdogConfig::default())
    }
}
