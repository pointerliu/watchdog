# Watchdog Server

This crate provides a generic server implementation for the watchdog framework. It handles the orchestration of fetching content and notifying subscribers based on their criteria.

## Features

- Generic server implementation that works with any fetcher and notifier
- Actix-based actor system for handling subscriptions
- Configurable check intervals
- Message-based API for adding/removing subscriptions

This crate is designed to be used as a library by other crates that want to implement specific subscription services.