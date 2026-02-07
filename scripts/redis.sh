#!/usr/bin/env bash
set -x
set -eo pipefail

docker run \
  -p "6379:6379" \
  -d \
  --name "redis_store" \
  redis

echo "Redis is running on port 6379"
