# Reverse Proxy

Use Nginx as a reverse-proxy to provide external connections on ports `80` (http) & `443` (https) for inbound traffic. This is the only publically accessibly entrypoint into the cluster.

## Setup

The first time you set up a new domain, it should be accessible over http immediately. However, secure connections will require slightly more work.

Upon first run, login to the docker container by ssh'ing into the host machine and running `docker exec -it reverse-proxy /bin/bash`. Then run `./register.sh <domain.tld>`

The setup was inspired by [this guide](https://techsparx.com/software-development/docker/damp/nginx-cron-ssl.html).

## Let's Encrypt

