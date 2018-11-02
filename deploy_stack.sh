#!/bin/sh

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

while [ ! $# -eq 0 ]
do
	case "$1" in
		--no-auth | -n)
			export BASIC_AUTH='false'
			;;
    --help | -h)
			echo "Usage: \n [default]\tdeploy the OpenFaaS core services\n --no-auth [-n]\tdisable basic authentication.\n --help\tdisplays this screen"
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
secret="$(head -c 16 /dev/urandom| $sha_cmd | cut -d ' ' -f 1)"

echo '[Credentials]'
echo "username: $user"
echo "password: $secret"
echo
echo "printf '$secret' | faas-cli login --username=admin --password-stdin"
echo

docker secret rm basic-auth-user &>/dev/null|| true
docker secret rm basic-auth-password &>/dev/null|| true

printf "$user" | docker secret create basic-auth-user -
printf "$secret" | docker secret create basic-auth-password -

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
