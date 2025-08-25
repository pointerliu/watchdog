# Subscription Service Workspace

This workspace contains two crates that work together to provide a subscription service:

- `subscription-framework-lib`: A library crate that provides the core functionality for managing subscriptions
- `subscription-server-bin`: A binary crate that provides a server implementation using the framework

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

To run the server:

```bash
cargo run -p subscription-server
```