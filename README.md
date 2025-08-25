# Subscription System Framework

A decoupled, scalable, and performant subscription system framework written in Rust, with a concrete implementation for academic paper subscriptions.

## Overview

This project provides a generic framework for building subscription systems where users can define their interests, and the system periodically fetches information from the internet and sends notifications when updates are available.

The project consists of two parts:
1. `subscription-framework`: A generic framework for building subscription systems
2. `paper-subscriber`: A concrete implementation for subscribing to academic papers

## Features

- Decoupled architecture with clearly defined traits
- Scalable design using async/await
- Performant with Tokio runtime
- Extensible components (Fetchers, Notifiers, Storage)
- Error handling with comprehensive error types

## Framework Components

1. **Subscription**: Represents a user's subscription with criteria
2. **Fetcher**: Fetches content from external sources
3. **Notifier**: Sends notifications to users
4. **Storage**: Persists subscriptions and other data

## Paper Subscriber Example

The paper subscriber example demonstrates how to use the framework to create a system that allows users to subscribe to academic papers based on keywords of interest.

## Usage

To run the paper subscriber example:

```bash
cd paper-subscriber
cargo run
```

## Extending the Framework

To create your own subscription system using this framework:

1. Define your subscription criteria by implementing the `SubscriptionCriteria` trait
2. Implement a `Fetcher` to retrieve data from your source
3. Implement a `Notifier` to send notifications to users
4. Use `SubscriptionManager` to manage subscriptions
5. Periodically check for updates and send notifications