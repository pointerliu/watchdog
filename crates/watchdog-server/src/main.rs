//! Main entry point for the Watchdog API server
//!
//! This module initializes the application, sets up the Actix-web server,
//! and starts the watchdog system.

use actix_web::{middleware::Logger, App, HttpServer};
use tracing::info;

use watchdog_server::{app::app_config, common::bootstrap::bootstrap_app};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Watchdog API server");

    // Bootstrap the application state
    let app_state = bootstrap_app().await;

    // Create and run the HTTP server
    info!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
