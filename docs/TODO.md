# TODOs

## api
1. ~~Use dotenv instead of hard-coded secret values.~~
1. ~~Setup authentication verification with Kratos auth service.~~
1. ~~Write a test verifying protected routes.~~
1. Simple integration with Postgres & sqlx as an example.
1. Default all routes to Protected and write a "Guest" data guard to opt-routes from this behavior.

## services

### auth
#### ory oathkeeper
1. Configure TLS on all connections to Oathkeeper
1. Set up a local domain dev.launchpad.rs redirect to 127.0.0.1:4455
1. Configure Let's Encrypt with Oathkeeper directly (or throw it all behind Traefik?)

### data
#### postgres
1. ~~Store Kratos data in Postgres rather than SQLite.~~
1. Change default password for ROLE `kratos` in `init-db.sql`.
1. Periodically backup the Postgres database to a remote service. [Hint](https://davejansen.com/how-to-set-up-and-use-postgres-using-docker/)
1. Investigate having a `login` role rather than exposing LOGIN to `kratos` and `api` roles.

### deployments
#### docker
1. ~~Use dotenv instead of hard-coded secret values in Docker.~~
1. ~~Create a `deployments/containers/` folder and set up a `just` command for running `docker-compose up --remove-orphans`.~~
1. Healthchecks for all services.
1. Deploy an unhardened "production" instance
1. Harden production instance.
1. Periodically [rebuild without caching](https://pythonspeed.com/articles/docker-cache-insecure-images/).

#### ci/cd
1. Set up GitHub actions to run tests for each service, the api and the web client.
1. Set up GitHub Action to deploy green builds to production

## web
1. ~~Use dotenv instead of hard-coded secret values.~~
1. Have an actual login experience on the Home page.
1. Support logout from menu bar.
1. Allow the user to edit their own identity settings.
1. Show off an integration with the application. Consider adding a second section to the settings page for app-settings.

## docs
1. Create documentation walking through the setup and maintenence of a Launchpad project.
1. Continue to flesh out the startup guide.
1. Update the README.md to include high-level descriptions of the different services and point to more complete documentation.
1. Create a book walking through some "twitter-clone" style apps using Launchpad.
