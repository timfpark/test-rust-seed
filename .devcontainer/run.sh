#!/bin/bash
set -euo pipefail

export TAG=`date -u +"%Y%m%dT%H%M%SZ"`
DOCKER_BUILDKIT=1 docker build -t "localhost:12345/app:${TAG}" .
docker push "localhost:12345/app:${TAG}"

cat deploy/codespace.yaml | envsubst | kubectl apply -f -
