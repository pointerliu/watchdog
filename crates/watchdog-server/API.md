# Watchdog API Documentation

This document provides an overview of the Watchdog API endpoints and their usage.

## Base URL

All API endpoints are prefixed with `/api/v1` unless otherwise specified.

## Health Check

### `GET /`

Health check endpoint to verify the server is running.

**Response:**
```json
{
  "status": 200,
  "message": "Watchdog API server is running",
  "data": null
}
```

## Subscription Management

### `GET /api/v1/subscriptions/{user_id}`

Get all subscriptions for a specific user.

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": ["subscription1", "subscription2"]
}
```

### `POST /api/v1/subscriptions`

Create a new subscription.

**Request Body:**
```json
{
  "user_id": "string",
  "subscription_id": "string",
  "keywords": ["string"]
}
```

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": {
    "user_id": "string",
    "subscription_id": "string"
  }
}
```

### `DELETE /api/v1/subscriptions/{user_id}/{subscription_id}`

Remove a subscription.

**Response:**
```json
{
  "status": 200,
  "message": "Subscription removed successfully",
  "data": null
}
```

## Fetcher Management

### `GET /api/v1/fetchers/types`

Get available fetcher types.

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": ["ArxivFetcher"]
}
```

### `GET /api/v1/fetchers/{user_id}`

Get all fetchers for a specific user.

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": ["fetcher1", "fetcher2"]
}
```

### `POST /api/v1/fetchers`

Add a new fetcher.

**Request Body:**
```json
{
  "user_id": "string",
  "fetcher_name": "string",
  "fetcher_type": "string",
  "subscription_id": "string"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Fetcher added successfully",
  "data": null
}
```

### `DELETE /api/v1/fetchers/{user_id}/{fetcher_name}`

Remove a fetcher.

**Response:**
```json
{
  "status": 200,
  "message": "Fetcher removed successfully",
  "data": null
}
```

## Notifier Management

### `GET /api/v1/notifiers/types`

Get available notifier types.

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": ["ArxivConsoleNotifier", "ArxivEmailNotifier"]
}
```

### `GET /api/v1/notifiers/{user_id}`

Get all notifiers for a specific user.

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": [
    {
      "user_id": "string",
      "notifier_name": "string",
      "notifier_type": "string"
    }
  ]
}
```

### `POST /api/v1/notifiers`

Add a new notifier.

**Request Body:**
```json
{
  "user_id": "string",
  "notifier_name": "string",
  "notifier_type": "string",
  "email_address": "string (optional, required for ArxivEmailNotifier)"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Notifier added successfully",
  "data": null
}
```

### `DELETE /api/v1/notifiers/{user_id}/{notifier_name}`

Remove a notifier.

**Response:**
```json
{
  "status": 200,
  "message": "Notifier removed successfully",
  "data": null
}
```

## Error Responses

All error responses follow the same structure:

```json
{
  "status": 500,
  "message": "Error message",
  "data": null
}
```

Status codes:
- 400: Bad Request
- 500: Internal Server Error