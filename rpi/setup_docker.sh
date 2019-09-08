#!/usr/bin/env bash

set -euo pipefail

# https://github.com/docker/for-linux/issues/709
curl -sL get.docker.com | sed 's/"buster"/"stretch"/' | sh

sudo groupadd -f docker
sudo usermod -aG docker "${USER}"

sudo systemctl enable docker
