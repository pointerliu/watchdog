//! Complete ArXiv subscription service that combines the API server with the subscription logic

use actix::prelude::*;
use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog::{
    subscription::Subscription,
};
use watchdog_server::{
    api::subscription_scope,
    service::StorageSubscriptionService,
    user::{UserEmailService, user_email_scope},
    server::{SubscriptionServer, AddUserWorkerMsg},
    AddSubscriptionMsg, ServerConfig,
};
// Import arxiv components from the local crate
mod arxiv;
use arxiv::{ArxivFetcher, ArxivFetcherBuilder, ArxivCriteria, ArxivNotifier};

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
    
    // Create a user email service
    let user_email_service = Arc::new(RwLock::new(UserEmailService::new()));
    
    // Create server config
    let config = ServerConfig::default();
    
    // Create and start the multi-user server with console notifier for demo
    let server = SubscriptionServer::<ArxivFetcher, ArxivNotifier, ArxivCriteria>::new(config);
    let server_addr = server.start();
    
    // Add a demo user with console notifier
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(5)
        .build()
        .unwrap();
    let notifier = ArxivNotifier;
    
    let worker_result = server_addr
        .send(AddUserWorkerMsg {
            user_id: "demo_user".to_string(),
            fetcher,
            notifier,
            phantom: std::marker::PhantomData,
        })
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to add user worker: {}", e)))?;
    
    match worker_result {
        Ok(worker_addr) => {
            // Add a demo subscription
            let subscription = Subscription::new(
                "demo_user".to_string(),
                ArxivCriteria::new(
                    "demo_subscription".to_string(),
                    vec!["neural".to_string(), "deep learning".to_string()],
                ),
            );
            worker_addr.do_send(AddSubscriptionMsg { subscription });
        }
        Err(e) => {
            eprintln!("Failed to create user worker: {}", e);
        }
    }
    
    println!("Starting complete ArXiv subscription service...");
    println!("API endpoints available at http://localhost:8080");
    println!("Health check: http://localhost:8080/health");
    println!("Subscriptions API: http://localhost:8080/api/v1/subscriptions");
    println!("User email API: http://localhost:8080/api/v1/users");
    println!("Note: User identification is done through the X-User-ID header");

    // Create and start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(subscription_service.clone()))
            .app_data(web::Data::new(user_email_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<ArxivCriteria, StorageSubscriptionService<ArxivCriteria>>())
                    .service(user_email_scope())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}