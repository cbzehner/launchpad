#!/usr/bin/env bash
# inspiration for this approach: https://coderjourney.com/deploy-docker-digital-ocean/
# script template source & explaination: https://betterdev.blog/minimal-safe-bash-script-template/

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

usage() {
  cat <<EOF
Usage: $(basename "${BASH_SOURCE[0]}") [-h] [-v] [-f] -p param_value arg1 [arg2...]

Script description here.

Available options:

-h, --help      Print this help and exit
-v, --verbose   Print script debug info
-f, --flag      Some flag description
-p, --param     Some param description
EOF
  exit
}

cleanup() {
  trap - SIGINT SIGTERM ERR EXIT
  # script cleanup here
}

setup_colors() {
  if [[ -t 2 ]] && [[ -z "${NO_COLOR-}" ]] && [[ "${TERM-}" != "dumb" ]]; then
    NOFORMAT='\033[0m' RED='\033[0;31m' GREEN='\033[0;32m' ORANGE='\033[0;33m' BLUE='\033[0;34m' PURPLE='\033[0;35m' CYAN='\033[0;36m' YELLOW='\033[1;33m'
  else
    NOFORMAT='' RED='' GREEN='' ORANGE='' BLUE='' PURPLE='' CYAN='' YELLOW=''
  fi
}

msg() {
  echo >&2 -e "${1-}"
}

die() {
  local msg=$1
  local code=${2-1} # default exit status 1
  msg "$msg"
  exit "$code"
}

parse_params() {
  # default values of variables set from params
  flag=0
  param=''

  while :; do
    case "${1-}" in
    -h | --help) usage ;;
    -v | --verbose) set -x ;;
    --no-color) NO_COLOR=1 ;;
    -f | --flag) flag=1 ;; # example flag
    -p | --param) # example named parameter
      param="${2-}"
      shift
      ;;
    -?*) die "Unknown option: $1" ;;
    *) break ;;
    esac
    shift
  done

  args=("$@")

  # check required params and arguments
  [[ -z "${param-}" ]] && die "Missing required parameter: param"
  [[ ${#args[@]} -eq 0 ]] && die "Missing script arguments"

  return 0
}

parse_params "$@"
setup_colors

## Start deploy-machine.sh logic ##

## Verify dotenv file has been sourced properly (or ENV are otherwise set)
## Verify docker (for docker-compose) & docker-machine are installed

## Verify launchpad is active/set launchpad as active
docker-machine status launchpad # should output "Running"

# Set the local docker instance to the remote machine
eval $(docker-machine env launchpad)

docker-machine active # Should output "launchpad"

# Sync Kratos config
docker-machine ssh launchpad mkdir -p /launchpad/services/auth/kratos
docker-machine scp ./services/auth/kratos/production/ launchpad:/launchpad/services/auth/kratos/ --recursive --delta

# Sync Oathkeeper config
docker-machine ssh launchpad mkdir -p /launchpad/services/auth/oathkeeper
docker-machine scp ./services/auth/oathkeeper/production/ launchpad:/launchpad/services/auth/oathkeeper/ --recursive --delta

# Sync Postgres initialization command
## Copy over postgres/init-db.sql sed'ing secret => $POSTGRES_PASSWORD
docker-machine ssh launchpad mkdir -p /launchpad/postgres-data
docker-machine ssh launchpad mkdir -p /launchpad/services/data/postgres
docker-machine scp ./services/data/postgres/init-db.sql launchpad:/launchpad/services/data/postgres/init-db.sql --delta

## Run docker-compose
just deploy

# TODO: Handle error `failed to create endpoint angry_keldysh on network bridge: adding interface veth6fd0dc1 to bridge docker0 failed: could not find bridge docker0: route ip+net: no such network interface`
# docker-machine ssh launchpad systemctl restart docker

## TODO: Consider running launchpad-blue & launchpad-green and doing blue-green deploys. Where to store active state?

## Clean up dangling resources to prevent Docker cache from exceeding available disk
docker system prune --all --filter "until=24h" --filter "label!=production"
# docker volume prune --filter "label!=production"

## End deploy-machine.sh logic ##
msg "${RED}Read parameters:${NOFORMAT}"
msg "- flag: ${flag}"
msg "- param: ${param}"
msg "- arguments: ${args[*]-}"