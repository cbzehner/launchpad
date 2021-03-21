# API

The primary application API server. This is intended to be developed in a monolithic-style, with the majority of application functionality and business logic contained within this server.

## Overview

TODO

## When should something be a service vs. including in the API server?

The primary criteria for extraction of functionality into new `services/` are:
1. The functionality is unrelated to the application or has a clearly defined interface to the rest of the application.
1. The functionality may need to scale independently of the primary API (example: message queues and async workers).
1. The functionality is provided by a third-party as an idependent service or container.
