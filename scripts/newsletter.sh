curl -X POST http://127.0.0.1:8080/newsletter \
  -H "Content-Type: application/json" \
  -w "\nHTTP Status: %{http_code}\n" \
  -d '{
    "title": "Weekly Update",
    "content": "Hello subscribers! Here is this week'\''s update in plain text.",
    "html_content": "<html><body><h1>Weekly Update</h1><p>Hello subscribers! Here is this week'\''s update.</p></body></html>"
  }'
