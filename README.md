# launchpad

The vision for the project is encapsulated in our tagline:
> A starter kit for quickly building secure & performant modern applications. Ease of use, performance, flexibility, choose three.

## Overview

Launchpad is a template for building modern software applications.

There's no magic. No hidden configurations. You have full control over your code from the moment you take the hot seat in Mission Control.

The launchpad template is designed to be production ready from Day One and provide a foundation for common tasks in setting up software applications.

The components are modular, empowering you to swap out solutions that don't make sense for your application as it grows.

## Getting Started
<details>
  <summary>Expand for details...</summary>

### Install Dependencies

#### Base Dependencies
- Rust
- Postgres
- Docker
- NodeJS
- Yarn

#### Development Tools

This project depends on a few Rust packages
- `cargo-chef` which builds Rust projects in Docker with automatic caching.
- `just` is a `make`-style task runner that doesn't require tabs.
- `watchexec` is a filewatcher for easy development.
- `mdbook` is a command to create an online book from markdown files.
- `mdbook-linkcheck` is a linter for verifying the mdbook links are valid.

Run `cargo install cargo-chef just watchexec mdkook mdbook-linkcheck` to install the dependencies.
</details>
<br/>

## Architecture

<img width="1179" alt="Project architecture diagram showing services and dependencies" src="https://user-images.githubusercontent.com/3886290/116010365-23ff6a00-a5e4-11eb-89ec-258e57ec1b52.png">

[Edit](https://whimsical.com/launchpad-Y4AZpD16s4S7exbrayWQNZ)

### Frontend

The front-end is a simple [Create React App](https://create-react-app.dev/) using [Tailwind UI](https://tailwindui.com/).

It's easily swappable for [Vue](https://vuejs.org/), [Svelte](https://svelte.dev/), [Elm](https://elm-lang.org/) or your custom front-end application.

It's built for production and served by a base Nginx container.

### Backend

The backend consists of a series of services, each of which is responsible for a single concern. 

#### API
The primary service is simply named "API" and is a [Rocket](rocket.rs/) service with pre-configured hooks into the authentication service.

#### Authentication
Authentication is handled by [ORY Kratos](https://www.ory.sh/kratos/), a backend service for managing identities.

#### Database
Postgres is the primary database. Two different databases are run inside a single Postgres instance, `kratos` for user identity management and `api` for use by the API service.

### Security

TLS encryption of public internet traffic is terminated at an Nginx reverse-proxy. All other services communicate via an internal network and do not expose ports to the internet.

[ORY Oathkeeper](https://github.com/ory/oathkeeper), provides access control and routing for traffic recieved from the reverse proxy. Oathkeeper will ensure that requests are routed to the appropriate service and confirm access against the identity server [ORY Kratos](https://github.com/ory/kratos).

By default, unauthenticated users ("guests") will be redirected to the `/auth/login` route if attempting to access any other route.

## Who should use this?

⚠️ This is currently pre-alpha software and isn't recommended for production use by anyone!

## Contributing

All contributions are welcome! This code is meant for you so feel free to suggest improvements or features!

Not all features will be accepted, but the maintainer(s) will strive to handle requests transparently.

Please open a GitHub issue if you have questions or need to get in touch with the maintainers.

## Known Issues

- [ ] Need to sign up for security alerts for all major frameworks components. Route them to `security-alerts@launchpad.rs` and perfom maintenence/upgrades as needed.
- [ ] Need to sign up for upgrade alerts for all major frameworks components. Route them to `dependencies@launchpad.rs` and perfom maintenence/upgrades as needed.

# License
Launchpad is licensed under either of the following, at your option:

Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
