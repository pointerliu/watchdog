#!/bin/bash

# Test script for the ArXiv Subscription API

echo "=== ArXiv Subscription API Test Script ==="
echo ""

# Check if server is running
echo "1. Checking server health..."
curl -s -X GET http://localhost:8080/health | jq .
echo ""

# List existing subscriptions
echo "2. Listing existing subscriptions..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions | jq .
echo ""

# Create a new subscription
echo "3. Creating a new subscription..."
curl -s -X POST http://localhost:8080/api/v1/subscriptions \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "criteria": {
      "id": "test_subscription",
      "keywords": ["machine learning", "AI"]
    }
  }' | jq .
echo ""

# List subscriptions again
echo "4. Listing subscriptions after creation..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions | jq .
echo ""

# Get the specific subscription
echo "5. Getting the specific subscription..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions/test_subscription | jq .
echo ""

# Delete the subscription
echo "6. Deleting the subscription..."
curl -s -X DELETE http://localhost:8080/api/v1/subscriptions/test_subscription | jq .
echo ""

# List subscriptions one final time
echo "7. Listing subscriptions after deletion..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions | jq .
echo ""

echo "=== Test Complete ==="