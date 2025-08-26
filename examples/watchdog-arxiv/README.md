# ArXiv Subscription Service Example

This example demonstrates how to use the Watchdog framework to build a subscription service for ArXiv papers, with both a traditional actor-based server and a modern RESTful API.

## Features

- Subscribe to ArXiv papers based on keywords
- Get notified when new papers match your criteria
- Multi-user support with individual fetcher configurations
- RESTful API for managing subscriptions
- User email management for email notifications
- Health check endpoint

## Components

The example provides several different binaries to demonstrate various aspects of the framework:

1. **watchdog-arxiv**: Traditional actor-based server that fetches content and sends notifications
2. **watchdog-arxiv-api**: REST API server for managing subscriptions and user emails
3. **watchdog-arxiv-complete**: Combined server that provides both the API and subscription logic
4. **email-notifier-example**: Example of using the EmailNotifier for sending email notifications

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

**Request Headers:**
```
X-User-ID: string (optional, can also be specified in request body)
```

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

**Request Headers:**
```
X-User-ID: string (required)
```

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

**Request Headers:**
```
X-User-ID: string (required)
```

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

**Request Headers:**
```
X-User-ID: string (required)
```

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Subscription removed successfully"
  }
}
```

### User Email Management

#### Set User Email
```
POST /api/v1/users/{user_id}/email
```
Set or update a user's email address for notifications.

**Request Body:**
```json
{
  "user_id": "string",
  "email": "string"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "User email set successfully"
  }
}
```

#### Get User Email
```
GET /api/v1/users/{user_id}/email
```
Get a user's email address.

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": "string",
    "email": "string"
  }
}
```

#### Delete User Email
```
DELETE /api/v1/users/{user_id}/email
```
Remove a user's email address.

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "User email removed successfully"
  }
}
```

#### List All User Emails
```
GET /api/v1/users/emails
```
List all user email addresses.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "user_id": "string",
      "email": "string"
    }
  ]
}
```

## Running the Example

### Prerequisites
- Rust toolchain (latest stable version)
- Cargo package manager
- `curl` and `jq` for API testing (optional but recommended)

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

### Running the Complete Server
```bash
# Run the combined server that provides both API and subscription logic
cargo run --bin watchdog-arxiv-complete
```

The API server will start on `http://localhost:8080`.

### Running the EmailNotifier Example
```bash
# Run the EmailNotifier example (see source for SMTP configuration)
cargo run --bin email-notifier-example
```

## Testing the API

### Using the Test Script
We provide a convenient test script to verify the API functionality:

```bash
# Make sure the API server is running in another terminal
cargo run --bin watchdog-arxiv-api

# In another terminal, run the test script
./test-api.sh
```

### Manual API Testing
You can also test the API manually using curl:

#### Health Check
```bash
curl -X GET http://localhost:8080/health
```

#### Create a Subscription
```bash
curl -X POST http://localhost:8080/api/v1/subscriptions \
  -H "Content-Type: application/json" \
  -H "X-User-ID: ml_researcher" \
  -d '{
    "user_id": "ml_researcher",
    "criteria": {
      "id": "ml_papers",
      "keywords": ["machine learning", "neural networks"]
    }
  }'
```

#### List All Subscriptions
```bash
curl -X GET http://localhost:8080/api/v1/subscriptions \
  -H "X-User-ID: ml_researcher"
```

#### Get a Specific Subscription
```bash
curl -X GET http://localhost:8080/api/v1/subscriptions/ml_papers \
  -H "X-User-ID: ml_researcher"
```

#### Delete a Subscription
```bash
curl -X DELETE http://localhost:8080/api/v1/subscriptions/ml_papers \
  -H "X-User-ID: ml_researcher"
```

#### Set User Email
```bash
curl -X POST http://localhost:8080/api/v1/users/ml_researcher/email \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "ml_researcher",
    "email": "ml.researcher@example.com"
  }'
```

#### Get User Email
```bash
curl -X GET http://localhost:8080/api/v1/users/ml_researcher/email
```

## Using EmailNotifier

To use the EmailNotifier for sending email notifications:

1. Create an EmailNotifier with your SMTP configuration:
```rust
let email_notifier = EmailNotifier::new(
    "smtp.example.com".to_string(),     // SMTP server
    587,                                // SMTP port
    "your-email@example.com".to_string(), // SMTP username
    "your-password".to_string(),        // SMTP password
);
```

2. Set user email addresses:
```rust
email_notifier.set_user_email("user1".to_string(), "user1@example.com".to_string()).await;
```

3. Use the EmailNotifier when creating user workers:
```rust
let worker_result = server_addr
    .send(AddUserWorkerMsg {
        user_id: "user1".to_string(),
        fetcher: my_fetcher,
        notifier: email_notifier,
        phantom: std::marker::PhantomData,
    })
    .await?;
```

Note: For security reasons, never hardcode SMTP credentials in your source code. Use environment variables or configuration files instead.

## Architecture

### Current Architecture
The service is built with a clean separation of concerns:

1. **Core Framework** (`watchdog` crate): Provides the foundational traits and types
2. **Server Implementation** (`watchdog-server` crate): Implements the actor-based server and RESTful API with multi-user support
3. **Application Logic** (`watchdog-arxiv` crate): Implements ArXiv-specific logic for fetching and matching papers

### How It Works
1. The `SubscriptionServer` manages multiple `SubscriptionWorker` instances, one per user
2. Each `SubscriptionWorker` handles subscriptions for a single user with their specific fetcher and notifier
3. Workers periodically fetch content using their fetcher
4. When new content is found, it's matched against user subscriptions
5. Matching content is sent to users via their configured notifier (ConsoleNotifier, EmailNotifier, etc.)

This modular design allows for maximum reusability and extensibility.