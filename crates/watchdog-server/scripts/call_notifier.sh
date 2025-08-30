curl -X GET 127.0.0.1:8080 && echo "\n"
curl -X GET "127.0.0.1:8080/api/v1/notifiers/types" && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/notifiers" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "notifier_name": "console_1", "notifier_type": "ArxivConsoleNotifier"}' && echo "\n"

curl -X POST "127.0.0.1:8080/api/v1/notifiers" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "lzz", "notifier_name": "email_1", "notifier_type": "ArxivEmailNotifier", "email_address": "ellen7ions@163.com"}' && echo "\n"


curl -X GET "127.0.0.1:8080/api/v1/notifiers/lzz" && echo "\n"


#curl -X DELETE "127.0.0.1:8080/api/v1/notifiers/lzz/fetcher_1" && echo "\n"
#curl -X GET "127.0.0.1:8080/api/v1/notifiers/lzz" && echo "\n"
