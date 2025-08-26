//! Example of using the watchdog-server API layer with arXiv subscriptions

use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_server::{
    api::subscription_scope,
    service::StorageSubscriptionService,
};
// Import arxiv components from the local crate
mod arxiv;
use arxiv::ArxivCriteria;

/// Main function to start the arXiv subscription API server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Create a storage-based subscription service
    let subscription_service = Arc::new(RwLock::new(StorageSubscriptionService::<ArxivCriteria>::new()));
    
    println!("Starting arXiv subscription API server...");
    println!("API endpoints available at http://localhost:8080/api/v1/subscriptions");

    // Create and start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(subscription_service.clone()))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<ArxivCriteria, StorageSubscriptionService<ArxivCriteria>>())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}