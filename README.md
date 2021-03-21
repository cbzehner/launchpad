# launchpad

The vision for the project is encapsulated in our tagline:
> A starter kit for quickly building secure & performant modern applications. Ease of use, performance, flexibility, choose three.

## Overview

Launchpad is a template for building modern software applications.

There's no magic. No hidden configurations. You have full control over your code from the moment you take the hot seat in Mission Control.

The launchpad template is designed to be production ready from Day One and provide a foundation for common tasks in setting up software applications.

The components are modular, empowering you to swap out solutions that don't make sense for your application as it grows.

## Getting Started

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

## Architecture

![diagram](https://user-images.githubusercontent.com/3886290/107551504-34787a80-6b87-11eb-9299-92cebe454176.png)
[Edit](https://whimsical.com/launchpad-Y4AZpD16s4S7exbrayWQNZ)

### Frontend

The frontend consists of three separate components:
- Auth: The login/sign-up flow and other self-management settings. Interacts with the Auth service.
- App: The traditional application frontend.
- Admin: A frontend for internal users and application administrators.

### Backend

The backend consists of a primary API endpoint and secondary services, each of which handles a specific task.

Postgres is used as the primary database.

### Security

The reverse proxy, [Traefik](https://github.com/traefik/traefik) and/or [ORY Oathkeeper](https://github.com/ory/oathkeeper), will ensure that requests are routed to the appropriate frontend and confirm access against the identity server [ORY Kratos](https://github.com/ory/kratos).

Unauthenticated users will be redirected to the `auth` subdomain and after logging in they'll be redirected back to their original destination. If they don't have access (example: a non-admin user attempting to access the `admin` subdomain they will encounter a `404 Not Found` error).

### Ports

Port   | Service        | Description
------ | ------         |----------
`4433` | ORY Kratos     | Administrative API endpoints
`4433` | ORY Kratos     | Public API endpoints
`4435` | React Web App  | Direct access, prefer going through Oathkeeper
`4436` | Mailslurper    | ?
`4437` | Mailslurper    | ?
`4438` | Backend API    | $
`4455` | ORY Oathkeeper | Public endpoint for accessing
`4456` | ORY Oathkeeper | $
`5432` | Postgres       | $


## Who should use this?

This is currently pre-alpha software and isn't recommended for production use.

## Contributing

All contributions are welcome! This code is meant for you so feel free to suggest improvements or features! Not all features will be accepted, but the maintainers will strive to handle requests transparently.

Please open a GitHub issue if you have questions or need to get in touch with the maintainers.

## Known Issues

- [ ] Need to sign up for security alerts for all major frameworks components. Route them to `security-alerts@launchpad.rs` and perfom maintenence/upgrades as needed.
- [ ] Need to sign up for upgrade alerts for all major frameworks components. Route them to `dependencies@launchpad.rs` and perfom maintenence/upgrades as needed.

# License
Launchpad is licensed under either of the following, at your option:

Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
