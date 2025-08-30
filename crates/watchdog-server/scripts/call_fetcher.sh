curl -X GET 127.0.0.1:8080 && echo "\n"
curl -X GET "127.0.0.1:8080/api/v1/fetchers/types" && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/fetchers" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "fetcher_name": "fetcher_1", "fetcher_type": "ArxivFetcher", "subscription_id": "machine learning"}' && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/fetchers" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "fetcher_name": "fetcher_2", "fetcher_type": "ArxivFetcher", "subscription_id": "software engineering"}' && echo "\n"

curl -X GET "127.0.0.1:8080/api/v1/fetchers/lzz" && echo "\n"
#curl -X DELETE "127.0.0.1:8080/api/v1/fetchers/lzz/fetcher_1" && echo "\n"
curl -X GET "127.0.0.1:8080/api/v1/fetchers/lzz" && echo "\n"
