# Watchdog Server

This crate provides a server implementation for the Watchdog subscription framework. It includes both an actor-based server for handling subscriptions and a RESTful API layer for easy integration with web applications.

## Features

- Actor-based subscription server using Actix
- RESTful API for subscription management
- Flexible and extensible design
- Decoupled frontend and backend

## API Endpoints

The API provides the following endpoints for subscription management:

- `POST /api/v1/subscriptions` - Create a new subscription
- `GET /api/v1/subscriptions` - List all subscriptions
- `GET /api/v1/subscriptions/{id}` - Get a specific subscription
- `DELETE /api/v1/subscriptions/{id}` - Remove a subscription

## Usage

To use the API layer, you need to:

1. Create a service that implements the `SubscriptionService` trait
2. Mount the API scope on your Actix Web application

Example:

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_server::{
    api::{self, subscription_scope},
    service::StorageSubscriptionService,
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
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(subscription_service.clone()))
            .service(
                web::scope("/api/v1")
                    .service(subscription_scope::<MyCriteria, StorageSubscriptionService<MyCriteria>>())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Modules

- `server`: Actor-based subscription server implementation
- `api`: RESTful API layer for subscription management
- `service`: Implementations of the SubscriptionService trait