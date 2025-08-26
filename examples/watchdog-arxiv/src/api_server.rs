//! HTTP API server for the arXiv subscription service

use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_server::{
    api::subscription_scope,
    service::StorageSubscriptionService,
};
// Import arxiv components from the local crate
mod arxiv;
use arxiv::ArxivCriteria;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        service: "arxiv-subscription-api".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create a storage-based subscription service
    let subscription_service = Arc::new(RwLock::new(StorageSubscriptionService::<ArxivCriteria>::new()));
    
    println!("Starting arXiv subscription API server...");
    println!("API endpoints available at http://localhost:8080");
    println!("Health check: http://localhost:8080/health");
    println!("Subscriptions API: http://localhost:8080/api/v1/subscriptions");

    // Create and start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(subscription_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<ArxivCriteria, StorageSubscriptionService<ArxivCriteria>>())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}