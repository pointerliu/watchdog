//! Application bootstrap
//!
//! This module is responsible for initializing all components of the application
//! and wiring them together.

use actix_web::web::Data;
use std::sync::Arc;
use tracing::info;

use crate::common::app_state::AppState;
use watchdog_core::Watchdog;
use watchdog_service::arxiv::fetcher::ArxivFetcher;
use watchdog_service::arxiv::model::ArxivPaper;
use watchdog_service::arxiv::criteria::ArxivCriteria;

/// Bootstrap the application by creating and wiring all components
pub async fn bootstrap_app() -> Data<AppState<ArxivPaper, ArxivCriteria>> {
    info!("Bootstrapping application");
    // Create the watchdog system with default configuration
    let watchdog: Watchdog<ArxivPaper, ArxivCriteria> = Watchdog::with_defaults();

    // Add default fetcher
    // let arxiv_fetcher = ArxivFetcher::default();
    // if let Err(e) = watchdog.add_fetcher("default_arxiv".to_string(), Box::new(arxiv_fetcher)).await {
    //     panic!("Failed to add default fetcher: {}", e);
    // }

    // Start the watchdog system
    match watchdog.start() {
        Ok(_) => info!("Watchdog system started successfully"),
        Err(e) => panic!("Failed to start watchdog system: {}", e),
    }

    // Create application state
    let app_state = AppState::new(Arc::new(watchdog));

    Data::new(app_state)
}
