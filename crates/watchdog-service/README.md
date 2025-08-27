# Watchdog Server

This crate provides a server implementation for the Watchdog subscription framework. It includes both an actor-based server for handling subscriptions and a RESTful API layer for easy integration with web applications.

## Features

- Actor-based subscription server using Actix with multi-user support
- RESTful API for subscription management
- User email management for email notifications
- Flexible and extensible design
- Decoupled frontend and backend

## Architecture

The server now supports multiple users with individual workers:

- `SubscriptionServer`: Manages multiple user workers
- `SubscriptionWorker`: Handles subscriptions for a single user
- Each user can have their own fetcher and notifier configurations

## API Endpoints

The API provides the following endpoints for subscription management:

- `POST /api/v1/subscriptions` - Create a new subscription
- `GET /api/v1/subscriptions` - List all subscriptions
- `GET /api/v1/subscriptions/{id}` - Get a specific subscription
- `DELETE /api/v1/subscriptions/{id}` - Remove a subscription

User identification is done through the `X-User-ID` header.

The API also provides endpoints for user email management:

- `POST /api/v1/users/{user_id}/email` - Set a user's email address
- `GET /api/v1/users/{user_id}/email` - Get a user's email address
- `DELETE /api/v1/users/{user_id}/email` - Remove a user's email address
- `GET /api/v1/users/emails` - List all user emails

## Usage

To use the API layer, you need to:

1. Create a service that implements the `SubscriptionService` trait
2. Create a `UserEmailService` for managing user emails
3. Mount the API scope on your Actix Web application

Example:

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_server::{
    api::{self, subscription_scope},
    service::StorageSubscriptionService,
    user::{UserEmailService, user_email_scope},
};

// Your subscription criteria type
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyCriteria {
    id: String,
    // ... your criteria fields
}

impl SubscriptionCriteria for MyCriteria {
    type Id = String;
    type Content = MyContent;
    
    fn matches(&self, content: &Self::Content) -> bool {
        // ... matching logic
    }
    
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscription_service = Arc::new(RwLock::new(StorageSubscriptionService::<MyCriteria>::new()));
    let user_email_service = Arc::new(RwLock::new(UserEmailService::new()));
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(subscription_service.clone()))
            .app_data(web::Data::new(user_email_service.clone()))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<MyCriteria, StorageSubscriptionService<MyCriteria>>())
                    .service(user_email_scope())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

For the actor-based multi-user server:

```rust
use actix::prelude::*;
use watchdog_server::{ServerConfig, SubscriptionServer, AddUserWorkerMsg};

// Create and start the multi-user server
let config = ServerConfig::default();
let server = SubscriptionServer::<MyFetcher, MyNotifier, MyCriteria>::new(config);
let server_addr = server.start();

// Add a user worker
let worker_result = server_addr
    .send(AddUserWorkerMsg {
        user_id: "user1".to_string(),
        fetcher: my_fetcher_instance,
        notifier: my_notifier_instance,
        phantom: std::marker::PhantomData,
    })
    .await?;

// Handle the worker result and add subscriptions as needed
```

## Modules

- `server`: Actor-based subscription server implementation with multi-user support
- `api`: RESTful API layer for subscription management
- `service`: Implementations of the SubscriptionService trait
- `user`: User management functionality including email address management