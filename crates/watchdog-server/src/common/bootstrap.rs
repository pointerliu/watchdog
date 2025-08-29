//! Application bootstrap
//!
//! This module is responsible for initializing all components of the application
//! and wiring them together.

use actix_web::web::Data;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use tracing::info;

use crate::common::app_state::AppState;
use watchdog_core::subscription::SubscriptionCriteria;
use watchdog_core::{FetchResult, Watchdog};

/// Bootstrap the application by creating and wiring all components
pub async fn bootstrap_app<T, C>() -> Data<AppState<T, C>>
where
    T: Clone + Send + Sync + Debug + 'static + Unpin,
    C: SubscriptionCriteria<Content = T> + Send + Sync + Clone + Debug + 'static + Unpin,
    C::Id: Send + Sync + Hash + Eq + Clone + Debug + 'static,
    <C as SubscriptionCriteria>::Id: Unpin,
{
    info!("Bootstrapping application");
    // Create the watchdog system with default configuration
    let watchdog: Watchdog<T, C> = Watchdog::with_defaults();

    // Start the watchdog system
    match watchdog.start() {
        Ok(_) => info!("Watchdog system started successfully"),
        Err(e) => panic!("Failed to start watchdog system: {}", e),
    }

    // Create application state
    let app_state = AppState::new(Arc::new(watchdog));

    Data::new(app_state)
}
