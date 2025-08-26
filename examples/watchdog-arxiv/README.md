# ArXiv Subscription Service Example

This example demonstrates how to use the Watchdog framework to build a subscription service for ArXiv papers, with both a traditional actor-based server and a modern RESTful API.

## Features

- Subscribe to ArXiv papers based on keywords
- Get notified when new papers match your criteria
- RESTful API for managing subscriptions
- Health check endpoint

## API Endpoints

### Health Check
```
GET /health
```
Returns the status of the service.

**Response:**
```json
{
  "status": "ok",
  "service": "arxiv-subscription-api"
}
```

### Subscription Management

#### Create a Subscription
```
POST /api/v1/subscriptions
```
Create a new subscription for ArXiv papers.

**Request Body:**
```json
{
  "user_id": "string",
  "criteria": {
    "id": "string",
    "keywords": ["string"]
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Subscription created successfully"
  }
}
```

#### List All Subscriptions
```
GET /api/v1/subscriptions
```
Get a list of all subscriptions.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "user_id": "string",
      "criteria": {
        "id": "string",
        "keywords": ["string"]
      }
    }
  ]
}
```

#### Get a Specific Subscription
```
GET /api/v1/subscriptions/{id}
```
Get details of a specific subscription by ID.

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": "string",
    "criteria": {
      "id": "string",
      "keywords": ["string"]
    }
  }
}
```

#### Delete a Subscription
```
DELETE /api/v1/subscriptions/{id}
```
Remove a subscription by ID.

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Subscription removed successfully"
  }
}
```

## Running the Example

### Prerequisites
- Rust toolchain (latest stable version)
- Cargo package manager

### Building
```bash
# Clone the repository
git clone <repository-url>
cd qwen-coder-sbs

# Build all packages
cargo build
```

### Running the Actor-Based Server
```bash
# Run the traditional actor-based server
cargo run --bin watchdog-arxiv
```

### Running the API Server
```bash
# Run the HTTP API server
cargo run --bin watchdog-arxiv-api
```

The API server will start on `http://localhost:8080`.

## API Usage Examples

### Create a Subscription
```bash
curl -X POST http://localhost:8080/api/v1/subscriptions \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "ml_researcher",
    "criteria": {
      "id": "ml_papers",
      "keywords": ["machine learning", "neural networks"]
    }
  }'
```

### List All Subscriptions
```bash
curl -X GET http://localhost:8080/api/v1/subscriptions
```

### Get a Specific Subscription
```bash
curl -X GET http://localhost:8080/api/v1/subscriptions/ml_papers
```

### Delete a Subscription
```bash
curl -X DELETE http://localhost:8080/api/v1/subscriptions/ml_papers
```

### Health Check
```bash
curl -X GET http://localhost:8080/health
```

## Extending the Service

The ArXiv subscription service can be easily extended:

1. **Custom Criteria**: Implement the `SubscriptionCriteria` trait to define your own matching logic
2. **Custom Notifiers**: Implement the `Notifier` trait to send notifications via email, Slack, etc.
3. **Custom Fetchers**: Implement the `Fetcher` trait to fetch content from other sources
4. **Storage Backends**: Replace the in-memory storage with a database implementation

## Architecture

The service is built with a clean separation of concerns:

1. **Core Framework** (`watchdog` crate): Provides the foundational traits and types
2. **Server Implementation** (`watchdog-server` crate): Implements the actor-based server and RESTful API
3. **Application Logic** (`watchdog-arxiv` crate): Implements ArXiv-specific logic for fetching and matching papers

This modular design allows for maximum reusability and extensibility.