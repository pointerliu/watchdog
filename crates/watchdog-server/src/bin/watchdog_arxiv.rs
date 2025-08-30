//! Main entry point for the Watchdog API server
//!
//! This module initializes the application, sets up the Actix-web server,
//! and starts the watchdog system.

use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::time::Duration;
use tracing::info;
use watchdog_core::WatchdogConfig;
use watchdog_server::app::app_config;
use watchdog_server::common::app_state::AppState;
use watchdog_server::common::bootstrap::bootstrap_app;
use watchdog_service::arxiv::criteria::ArxivCriteria;
use watchdog_service::arxiv::model::ArxivPaper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().ok();

    info!("Starting Watchdog API server");

    // Read configuration from environment variables
    let fetch_interval_secs: u64 = env::var("FETCH_INTERVAL_SECS")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .expect("FETCH_INTERVAL_SECS must be a valid u64");

    let fetch_worker_threads: usize = env::var("FETCH_WORKER_THREADS")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("FETCH_WORKER_THREADS must be a valid usize");

    let cfg = WatchdogConfig {
        fetch_interval: Duration::from_secs(fetch_interval_secs),
        fetch_worker_threads,
    };

    // Bootstrap the application state
    let app_state: Data<AppState<ArxivPaper, ArxivCriteria>> = bootstrap_app(cfg).await;

    // Create and run the HTTP server
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    info!("Starting HTTP server on {}:{}", server_host, server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(app_config)
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
