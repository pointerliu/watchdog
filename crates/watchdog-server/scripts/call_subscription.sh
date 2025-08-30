curl -X GET 127.0.0.1:8080 && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/subscriptions" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "subscription_id": "machine learning", "keywords": ["learning", "AI"]}' && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/subscriptions" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "subscription_id": "software engineering", "keywords": ["symbolic execution", "fuzzing", "software"]}' && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/subscriptions" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "subscription_id": "software engineering", "keywords": ["symbolic execution", "fuzzing"]}' && echo "\n"



curl -X GET "127.0.0.1:8080/api/v1/subscriptions/lzz" && echo "\n"


curl -X DELETE "127.0.0.1:8080/api/v1/subscriptions/lzz/machine%20learning" && echo "\n"
curl -X GET "127.0.0.1:8080/api/v1/subscriptions/lzz" && echo "\n"
