# Watchdog API Server

A REST API server for the Watchdog subscription system, built with Actix-web following a clean architecture pattern.

## Features

- RESTful API for managing subscriptions
- Clean architecture with separation of concerns
- Domain-driven design
- Actix-web for high-performance HTTP handling

## API Endpoints

### Health Check

- `GET /` - Health check endpoint

### Subscriptions

- `POST /api/v1/subscriptions` - Create a new subscription
- `DELETE /api/v1/subscriptions/{id}` - Remove a subscription

## Getting Started

### Prerequisites

- Rust (latest stable)

### Running the Server

1. Clone the repository
2. Navigate to the `watchdog-server` directory
3. Run the server:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`.

## API Usage

### Create a Subscription

```bash
curl -X POST http://localhost:8080/api/v1/subscriptions \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "criteria_id": "rust_news",
    "keywords": ["rust", "programming"]
  }'
```

### Remove a Subscription

```bash
curl -X DELETE http://localhost:8080/api/v1/subscriptions/rust_news
```

## Architecture

This server follows a clean architecture pattern with the following components:

- **API Layer**: HTTP handlers and route definitions using Actix-web
- **Application State**: Shared state management with the Watchdog system
- **Domain Layer**: Business logic implemented in the Watchdog core
- **DTOs**: Data Transfer Objects for API requests and responses

## Project Structure

```
├── src/
│   ├── main.rs                         # Application entry point
│   ├── app.rs                          # Router setup and middleware
│   ├── lib.rs                          # Module declarations
│   ├── common/                         # Shared components and utilities
│   │   ├── app_state.rs                # AppState struct for dependency injection
│   │   ├── bootstrap.rs                # Service initialization and AppState construction
│   │   ├── dto.rs                      # Shared/global DTOs
│   │   ├── error.rs                    # AppError enum and error mappers
│   │   ├── handlers.rs                 # Common handlers (e.g., health check)
│   ├── domains/                        # Feature modules
│   │   ├── subscription/               # Subscription feature module
│   │   │   ├── api/
│   │   │   │   ├── handlers.rs         # Route handlers
│   │   │   │   └── routes.rs           # Route definitions
│   │   │   ├── domain/                 # Domain models
│   │   │   │   └── model.rs
│   │   │   ├── dto/                    # Data Transfer Objects
│   │   │   │   └── subscription_dto.rs
│   │   │   └── infra/                  # Infrastructure-layer implementations
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.