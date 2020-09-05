# Launchpad

A starter kit for quickly building modern, secure applications. Ease of use, performance, flexibility, choose three.

## Overview

Launchpad provides a template for the basic functionality of web applications. Setting up a database, simple caching, user registration and authentication come pre-configured.

There's no magic. No hidden configurations. You have full control over your code from the moment you take over Mission Control.

## Getting Started

Clone this repository with `git clone git@github.com:cbzehner/launchpad.git` and remove the existing commit history with `cd launchpad && rm -rf .git/ && git init`.

Build for local development by running `cargo make serve`.

## Who should use this

This is currently pre-alpha software and isn't recommended for production use.

## Technologies

Launchpad is built with [Rocket 🚀](http://rocket.rs/) and stores data in [PostgreSQL](https://www.postgresql.org/) with a caching layer via [Redis](https://redis.io/).

The front-end is rendered via [Tera templates](https://tera.netlify.app/) using [Tailwind](https://tailwindcss.com/) for styling.

As the project evolves it will also include recommendations and preconfigured defaults for simple deployments and other standard infrastructure.

## Frequently Asked Questions (F.A.Q.)

Q: Why these technologies?

A: These are technologies I believe will stand the test of time. They have vibrant communities and plentiful resources for getting additional help.

Q: Where is the front-end? Can I only use this for developing APIs?

A: While there is an eventual goal to provide seamless support for \<front-end-framework-of-choice>, the initial project focus is on providing well-configured defaults for a small number of technologies.

SPAs are often unnecessary for projects that are starting out and thus the projects the author initially used Launchpad for are primarily server-side rendered using [Tera templates](https://tera.netlify.app/docs/) with only a small amount of Javascript.
