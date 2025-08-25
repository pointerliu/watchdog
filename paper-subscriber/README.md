# Paper Subscriber

An example implementation of a subscription system for academic papers using the subscription-framework.

## Overview

This application allows users to subscribe to academic papers based on keywords of interest. The system periodically fetches paper data from external sources (simulated in this example) and sends notifications when new papers matching the user's criteria are found.

## Implementation Details

- Uses the `subscription-framework` crate for core functionality
- Implements `PaperCriteria` as the subscription criteria
- Uses a simulated `ArxivFetcher` to fetch paper data
- Uses `ConsoleNotifier` to send notifications to the console

## Running the Example

```bash
cargo run
```

This will run the subscription system and process all registered subscriptions.