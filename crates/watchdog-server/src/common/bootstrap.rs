//! Application bootstrap
//!
//! This module is responsible for initializing all components of the application
//! and wiring them together.

use actix_web::web::Data;
use std::sync::Arc;
use tracing::info;

use crate::common::app_state::AppState;
use watchdog_core::subscription::SubscriptionCriteria;
use watchdog_core::{FetchResult, Watchdog};

// For now, we'll use simple string-based content for our example
// In a real application, this would be your domain model

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SimpleSubscriptionCriteria {
    id: String,
    keywords: Vec<String>,
}

impl SimpleSubscriptionCriteria {
    pub fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for SimpleSubscriptionCriteria {
    type Id = String;
    type Content = String;

    fn matches(&self, content: &String) -> bool {
        self.keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(&keyword.to_lowercase()))
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// Bootstrap the application by creating and wiring all components
pub async fn bootstrap_app() -> Data<AppState<String, SimpleSubscriptionCriteria>> {
    info!("Bootstrapping application");
    // Create the watchdog system with default configuration
    let watchdog: Watchdog<String, SimpleSubscriptionCriteria> = Watchdog::with_defaults();

    // Start the watchdog system
    match watchdog.start() {
        Ok(_) => info!("Watchdog system started successfully"),
        Err(e) => panic!("Failed to start watchdog system: {}", e),
    }

    // Create application state
    let app_state = AppState::new(Arc::new(watchdog));

    Data::new(app_state)
}
