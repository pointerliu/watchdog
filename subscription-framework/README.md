# Subscription Framework

A decoupled, scalable, and performant subscription system framework written in Rust.

## Overview

This framework provides the building blocks for creating subscription systems where users can define their interests, and the system periodically fetches information from the internet and sends notifications when updates are available.

## Features

- Decoupled architecture with clearly defined traits
- Scalable design using async/await
- Performant with Tokio runtime
- Extensible components (Fetchers, Notifiers, Storage)
- Error handling with comprehensive error types

## Components

1. **Subscription**: Represents a user's subscription with criteria
2. **Fetcher**: Fetches content from external sources
3. **Notifier**: Sends notifications to users
4. **Storage**: Persists subscriptions and other data

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
subscription-framework = { path = "../subscription-framework" }
```

### Implementing a Subscription System

1. Define your subscription criteria by implementing the `SubscriptionCriteria` trait
2. Implement a `Fetcher` to retrieve data from your source
3. Implement a `Notifier` to send notifications to users
4. Use `SubscriptionManager` to manage subscriptions
5. Periodically check for updates and send notifications

See the `paper-subscriber` example for a complete implementation.