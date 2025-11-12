# Subscription Service Workspace

This workspace contains three crates that work together to provide a subscription service:

- `watchdog`: A library crate that provides the core functionality for managing subscriptions
- `watchdog-server`: A library crate that provides a server implementation using the watchdog framework
- `watchdog-arxiv`: A binary crate that provides an example implementation using the watchdog crates

## Structure

This project follows a similar structure to the [actix](https://github.com/actix/actix) workspace, with:

- A root `Cargo.toml` that defines the workspace and its members
- Individual crates in their own directories
- Shared dependencies managed through the workspace configuration

## Building

To build all crates in the workspace:

```bash
cargo build
```

## Running the Examples

### Multi-User Actor-Based Server

```bash
cargo run --bin watchdog-arxiv
```

### HTTP API Server

```bash
cargo run --bin watchdog-arxiv-api
```

The API server will start on `http://localhost:8080` with the following endpoints:
- `GET /health` - Health check
- `POST /api/v1/subscriptions` - Create a subscription (requires X-User-ID header)
- `GET /api/v1/subscriptions` - List all subscriptions (requires X-User-ID header)
- `GET /api/v1/subscriptions/{id}` - Get a specific subscription (requires X-User-ID header)
- `DELETE /api/v1/subscriptions/{id}` - Delete a subscription (requires X-User-ID header)

See the [watchdog-arxiv README](examples/watchdog-arxiv/README.md) for detailed API documentation and usage examples.

## Architecture Changes

The `watchdog-server` crate has been updated to support multi-user scenarios:

- `SubscriptionServer`: Now manages multiple user workers
- `SubscriptionWorker`: Handles subscriptions for individual users
- Each user can have their own fetcher and notifier configurations

This allows for more scalable and flexible subscription management where different users can have different content sources and notification methods.

# Acknowledgement

This project is mainly developed with the help of [qwen-code](https://github.com/QwenLM/qwen-code).
