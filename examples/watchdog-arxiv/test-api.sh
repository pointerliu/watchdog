#!/bin/bash

# Test script for the ArXiv Subscription API

echo "=== ArXiv Subscription API Test Script ==="
echo ""

# Check if server is running
echo "1. Checking server health..."
curl -s -X GET http://localhost:8080/health | jq .
echo ""

# List existing subscriptions for test_user
echo "2. Listing existing subscriptions for test_user..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions -H "X-User-ID: test_user" | jq .
echo ""

# Create a new subscription
echo "3. Creating a new subscription for test_user..."
curl -s -X POST http://localhost:8080/api/v1/subscriptions \
  -H "Content-Type: application/json" \
  -H "X-User-ID: test_user" \
  -d '{
    "user_id": "test_user",
    "criteria": {
      "id": "test_subscription",
      "keywords": ["machine learning", "AI"]
    }
  }' | jq .
echo ""

# List subscriptions again for test_user
echo "4. Listing subscriptions for test_user after creation..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions -H "X-User-ID: test_user" | jq .
echo ""

# Get the specific subscription for test_user
echo "5. Getting the specific subscription for test_user..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions/test_subscription -H "X-User-ID: test_user" | jq .
echo ""

# Set user email
echo "6. Setting email for test_user..."
curl -s -X POST http://localhost:8080/api/v1/users/test_user/email \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "email": "test@example.com"
  }' | jq .
echo ""

# Get user email
echo "7. Getting email for test_user..."
curl -s -X GET http://localhost:8080/api/v1/users/test_user/email | jq .
echo ""

# List all user emails
echo "8. Listing all user emails..."
curl -s -X GET http://localhost:8080/api/v1/users/emails | jq .
echo ""

# Delete the subscription for test_user
echo "9. Deleting the subscription for test_user..."
curl -s -X DELETE http://localhost:8080/api/v1/subscriptions/test_subscription -H "X-User-ID: test_user" | jq .
echo ""

# List subscriptions one final time for test_user
echo "10. Listing subscriptions for test_user after deletion..."
curl -s -X GET http://localhost:8080/api/v1/subscriptions -H "X-User-ID: test_user" | jq .
echo ""

echo "=== Test Complete ==="