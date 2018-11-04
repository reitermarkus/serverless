#!/bin/bash

set -e
set +o pipefail

SERVICE_NAME=func_kafka
TASK_ID=$(docker service ps --filter 'desired-state=running' "$SERVICE_NAME" -q)
CONTAINER_ID=$(docker inspect --format '{{ .Status.ContainerStatus.ContainerID }}' "$TASK_ID")
docker exec -it "$CONTAINER_ID" kafka-console-producer --broker-list kafka:9092 --topic faas-request
