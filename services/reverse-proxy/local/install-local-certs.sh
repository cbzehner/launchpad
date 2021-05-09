#!/usr/bin/env bash
set -Eeuo pipefail

# Check for an existing certbot installation
if ! command -v certbot > /dev/null; then
  # install with homebrew on OSX
  if [[ "$OSTYPE" =~ ^darwin ]]; then
    brew install certbot
  fi
fi

if test -f services/reverse-proxy/local/certs/fullchain.pem && test -f services/reverse-proxy/local/certs/privkey.pem; then
    sudo certbot renew
else
  mkdir -p services/reverse-proxy/local/certs
  # Use the DNS Challenge Method to authenticate certificates for the domain
  sudo certbot certonly \
      --manual \
      --preferred-challenges dns \
      --agree-tos \
      -m admin@launchpad.rs \
      -d local.launchpad.rs,www.local.launchpad.rs
  # For help with debugging failures, visit https://letsdebug.net
fi

# Assuming we succeed, copy the files over
sudo cp -r /etc/letsencrypt/live/local.launchpad.rs/ services/reverse-proxy/local/certs
sudo chown -R $(whoami) services/reverse-proxy/local/certs