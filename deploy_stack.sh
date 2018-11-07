#!/bin/bash

set -e
set +o pipefail

if ! [ -x "$(command -v docker)" ]; then
  echo 'Cannot find `docker` command, please install Docker (https://www.docker.com/) and retry.' >&2
  exit 1
fi

if ! [ -x "$(command -v faas-cli)" ]; then
  case "$(uname -s)" in
    Darwin*)
      brew install faas-cli
      ;;
    linux*)
      curl -sSL https://cli.openfaas.com | sudo -E sh
      ;;
    msys*|cygwin*)
      choco install faas-cli -y
      ;;
    *)
      echo 'Cannot find `faas-cli` command, please install the OpenFAAS CLI (https://docs.openfaas.com/cli/install/) and retry.' >&2
      exit 1
      ;;
  esac
fi

export BASIC_AUTH='true'

while [[ $# > 0 ]]
do
  KEY="$1"
  case $KEY in
    -r|--restart)
      NEXT_ARG="$2"
      while ! [[ "$NEXT_ARG" =~ -.* ]] && [[ $# > 1 ]]; do
        TASK_ID=$(docker stack services -f "name=$NEXT_ARG" func -q | head -n 1)
        echo "restarting $NEXT_ARG $TASK_ID"
        docker service update --force $TASK_ID

        if ! [[ "$2" =~ -.* ]]; then
          shift
          NEXT_ARG="$2"
        else
          shift
          break
        fi
      done
      exit
      ;;
    -h|--help)
      echo "Usage: \n [default]\tdeploy the OpenFaaS core services\n --no-auth [-n]\tdisable basic authentication.\n --help\tdisplays this screen"
      exit
      ;;
    -n|--no-auth)
      export BASIC_AUTH='false'
      ;;
    *)
      echo "Unknown flag $KEY"
      exit
    ;;
  esac
  shift
done

# Initialize Swarm if not already created.
docker swarm ca &>/dev/null || docker swarm init

sha_cmd="shasum -a 256"
if ! command -v shasum >/dev/null; then
  sha_cmd="sha256sum"
fi

# Secrets should be created even if basic-auth is disabled.
echo 'Creating gateway credentials…'

user='admin'

if ! docker secret inspect basic-auth-user &>/dev/null; then
  printf "$user" | docker secret create basic-auth-user -
fi

if ! docker secret inspect basic-auth-password &>/dev/null; then
  secret="$(head -c 16 /dev/urandom| $sha_cmd | cut -d ' ' -f 1)"

  echo '[Credentials]'
  echo "username: $user"
  echo "password: $secret"
  echo
  echo "printf '$secret' | faas-cli login --username=admin --password-stdin"
  echo

  printf "$secret" | docker secret create basic-auth-password -
fi

echo

if [ "$BASIC_AUTH" = 'true' ]; then
  echo 'Enabling basic authentication…'
else
  echo 'Disabling basic authentication…'
fi

echo

echo 'Deploying OpenFaaS core services…'

mkdir -p faas
cp -f deploy.yml ./faas/deploy.yml
cd faas

mkdir -p prometheus

curl -sL https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alertmanager.yml -o ./prometheus/alertmanager.yml
curl -sL https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alert.rules.yml  -o ./prometheus/alert.rules.yml
curl -sL https://raw.githubusercontent.com/openfaas/faas/master/prometheus/prometheus.yml   -o ./prometheus/prometheus.yml

docker stack deploy func --compose-file deploy.yml
