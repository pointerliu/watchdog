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

To run the example:

```bash
cargo run
```

# Acknowledgement

This project is mainly developed with the help of [qwen-code](https://github.com/QwenLM/qwen-code).

