#!/usr/bin/env bash

set -euo pipefail

sudo apt-get update
sudo apt-get install -y ruby

curl -sSL https://cli.openfaas.com | sudo -E sh
