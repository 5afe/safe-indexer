#!/bin/sh

set -e

# RabbitMQ as message broker
# docker run -p 5672:5672 --rm -e RABBITMQ_DEFAULT_VHOST=my_vhost rabbitmq:3

# Redis as a message broker
docker run -p 6379:6379 --rm redis

# Findings:
# RabbitMQ is good as long as payloads are small and scalability is an issue
# Redis is good when performance is required and behaves better with bigger size payloads
