# Reverse Proxy

Use Nginx as a reverse-proxy to provide external connections on ports `80` (http) & `443` (https) for inbound traffic. This is the only publically accessibly entrypoint into the cluster.

## Setup

The first time you set up a new domain, it should be accessible over http immediately. However, secure connections will require slightly more work.

Make sure the `init-letsencrypt.sh` script has run and created the `./data/certbot` folders.

## Local Development

Local development currently only supports http traffic.

## Credit
Setup inspired by [Nginx and Let’s Encrypt with Docker in Less Than 5 Minutes](https://pentacent.medium.com/nginx-and-lets-encrypt-with-docker-in-less-than-5-minutes-b4b8a60d3a71)