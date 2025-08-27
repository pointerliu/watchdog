//! Complete ArXiv subscription service that combines the API server with the subscription logic
//! This version supports multiple notifiers per user (email and console)

use actix::prelude::*;
use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_core::{
    notifier::{ConsoleNotifier, EmailNotifier},
};
use watchdog_server::{api::subscription_scope, service::StorageSubscriptionService, user::{UserEmailService, user_email_scope}, server::{
    SubscriptionServer, AddUserWorkerMsg, AddNotifierMsg
}, ServerConfig};
// Import arxiv components from the local crate
use watchdog_arxiv::{ArxivFetcher, ArxivFetcherBuilder, ArxivCriteria, ArxivPaper};

/// Request to add a user with email configuration
#[derive(Deserialize, Serialize, Clone)]
pub struct AddUserRequest {
    pub user_id: String,
    pub email_address: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

/// Response for user operations
#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
}

/// API handler for adding a user with email configuration
pub async fn add_user_with_email(
    server_addr: web::Data<Addr<SubscriptionServer<ArxivFetcher, ArxivCriteria>>>,
    user_email_service: web::Data<Arc<RwLock<UserEmailService>>>,
    req: web::Json<AddUserRequest>,
) -> Result<HttpResponse> {
    let user_id = req.user_id.clone();
    let email_address = req.email_address.clone();
    
    // Store user email in the user email service
    user_email_service
        .write()
        .await
        .set_user_email(user_id.clone(), email_address.clone())
        .await
        .unwrap_or_else(|_| println!("Warning: Could not store user email"));
    
    // Create and start the user worker
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string()) // Default query, can be customized
        .number(5)
        .build()
        .unwrap();
    
    let worker_result = server_addr
        .send(AddUserWorkerMsg {
            user_id: user_id.clone(),
            fetcher,
            phantom: std::marker::PhantomData,
        })
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to add user worker: {}", e))
        })?;
    
    match worker_result {
        Ok(worker_addr) => {
            // Add console notifier
            let console_notifier = ConsoleNotifier;
            worker_addr.do_send(AddNotifierMsg::<ArxivPaper> {
                name: "console".to_string(),
                notifier: Box::new(console_notifier),
            });
            
            // Add email notifier
            let email_notifier = EmailNotifier::new(
                "smtp.163.com".to_string(), // Default SMTP server
                465, // Default port
                req.smtp_username.clone(),
                req.smtp_password.clone(),
            );
            
            // Set the user's email address in the email notifier
            email_notifier.set_user_email(user_id.clone(), email_address).await;
            
            worker_addr.do_send(AddNotifierMsg::<ArxivPaper> {
                name: "email".to_string(),
                notifier: Box::new(email_notifier),
            });
            
            Ok(HttpResponse::Ok().json(UserResponse {
                message: format!("User {} added successfully with console and email notifiers", user_id),
            }))
        }
        Err(e) => {
            Err(actix_web::error::ErrorInternalServerError(format!("Failed to create user worker: {}", e)))
        }
    }
}

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
    
    // Create and start the multi-user server
    let server = SubscriptionServer::<ArxivFetcher, ArxivCriteria>::new(config);
    let server_addr = server.start();
    
    println!("Starting complete ArXiv subscription service...");
    println!("API endpoints available at http://localhost:8080");
    println!("Health check: http://localhost:8080/health");
    println!("Subscriptions API: http://localhost:8080/api/v1/subscriptions");
    println!("User email API: http://localhost:8080/api/v1/users");
    println!("Add user with email: POST http://localhost:8080/api/v1/add-user");
    println!("Note: User identification is done through the X-User-ID header");

    // Create and start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(server_addr.clone()))
            .app_data(web::Data::new(subscription_service.clone()))
            .app_data(web::Data::new(user_email_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<ArxivCriteria, StorageSubscriptionService<ArxivCriteria>>())
                    .service(user_email_scope())
                    .route("/add-user", web::post().to(add_user_with_email))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}