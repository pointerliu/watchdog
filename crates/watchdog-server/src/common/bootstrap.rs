//! Application bootstrap
//!
//! This module is responsible for initializing all components of the application
//! and wiring them together.

use actix_web::web::Data;
use std::sync::Arc;
use tracing::info;

use crate::common::app_state::AppState;
use watchdog_core::storage::FetchStorage;
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

// Storage implementation
#[derive(Debug, Clone)]
pub struct FetchedDataStorage<T: Clone> {
    data: Arc<tokio::sync::RwLock<Vec<FetchResult<T>>>>,
}

impl<T: Clone> FetchedDataStorage<T> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl<T: Clone + Send + Sync> FetchStorage<T> for FetchedDataStorage<T> {
    async fn store(&self, result: FetchResult<T>) {
        let mut data = self.data.write().await;
        data.push(result);
    }

    async fn get_all(&self) -> Vec<FetchResult<T>> {
        let data = self.data.read().await;
        data.clone()
    }

    async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
    }
}

/// Bootstrap the application by creating and wiring all components
pub async fn bootstrap_app(
) -> Data<AppState<String, SimpleSubscriptionCriteria, FetchedDataStorage<String>>> {
    info!("Bootstrapping application");

    // Create storage for fetched data
    let storage = FetchedDataStorage::<String>::new();

    // Create the watchdog system with default configuration
    let watchdog: Watchdog<String, SimpleSubscriptionCriteria, FetchedDataStorage<String>> =
        Watchdog::with_defaults(storage);

    // Start the watchdog system
    match watchdog.start() {
        Ok(_) => info!("Watchdog system started successfully"),
        Err(e) => panic!("Failed to start watchdog system: {}", e),
    }

    // Create application state
    let app_state = AppState::new(Arc::new(watchdog));

    Data::new(app_state)
}
