# TODOs

## api
1. ~~Use dotenv instead of hard-coded secret values.~~
1. ~~Setup authentication verification with Kratos auth service.~~
1. ~~Write a test verifying protected routes.~~
1. ~~Correctly implement Rocket config using Figment.~~
1. ~Simple integration with Postgres & diesel as an example.~
  - ~~Setup Postgres & Diesel~~
  - ~~Construct a simple generic model (AppSettings)~~
  - ~Update test suite to continue to work as expected. Check for race conditions in test runs (ideal case: allow race conditions if tests all pass for better production veracity).~
1. Convert error handling to something more robust. Either Eyre or Anyhow. Bears further investigation.
1. ~Create some sort of "lint" or test ensuring that migrations are reversible. (CI? Script? Commit hook? Github Action!)~
1. ~Write conditional logic panicing if the localhost database password "secret" is detected in --release runs~
1. Default all routes to Protected and write a "Guest" data guard to opt-routes from this behavior.

## services

### auth
#### ory oathkeeper
1. Configure TLS on all connections to Oathkeeper
1. ~Set up a local domain local.launchpad.rs redirect to localhost~
1. Configure Let's Encrypt with Oathkeeper directly (or throw it all behind Traefik?)
1. Securely set `id_token.jwks.json` for `services/auth/oathkeeper/production`

### data
#### postgres
1. ~~Store Kratos data in Postgres rather than SQLite.~~
1. Move the `.data` folder into `services/data/postgres/` as `data/` and remove the `mkdir` step from the `init-machine.sh` script. Or just use a Docker volume rathar than a bind-mount.
1. Change default password for ROLE `kratos` in `init-db.sql`.
1. Store Kratos as a schema on the API database rather than an entirely separate database.
1. Periodically backup the Postgres database to a remote service. [Hint](https://davejansen.com/how-to-set-up-and-use-postgres-using-docker/)
1. Investigate having a `login` role rather than exposing LOGIN to `kratos` and `api` roles.

### deployments
#### docker
1. ~~Use dotenv instead of hard-coded secret values in Docker.~~
1. ~~Create a `deployments/containers/` folder and set up a `just` command for running `docker-compose up --remove-orphans`.~~
1. Healthchecks for all services.
1. ~Deploy an unhardened "production" instance~
1. Use Alpine as the base image for all production containers.
1. Harden production instance.
1. Periodically [rebuild without caching](https://pythonspeed.com/articles/docker-cache-insecure-images/).
1. Go through the [OWASP Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Docker_Security_Cheat_Sheet.html)
1. Go through `.env.local` and `environment:` keys and make sure everything makes sense and is currently in use.

#### ci/cd
1. ~Set up GitHub actions to run tests for each service, the api and the web client.~
1. ~~Set up GitHub Action to deploy green builds to production via a [DO Container Registry](https://docs.digitalocean.com/products/kubernetes/how-to/deploy-using-github-actions/)~~ Superceded by Watchtower (see below).
1. ~~Set up Watchtower to automatically restart containers.~~
1. Use Watchtower lifecycle hooks and `--rolling-restart` command to enable blue/green deploys.

## web
1. ~~Use dotenv instead of hard-coded secret values.~~
1. ~Dockerize the production build to be a built-image running a static file server~
1. Have an actual login experience on the Home page.
1. Support logout from menu bar.
1. Allow the user to edit their own identity settings.
1. Show off an integration with the application. Consider adding a second section to the settings page for app-settings.

## docs
1. Create documentation walking through the setup and maintenence of a Launchpad project.
1. Continue to flesh out the startup guide.
1. Update the README.md to include high-level descriptions of the different services and point to more complete documentation.
1. Create a book walking through some "twitter-clone" style apps using Launchpad.
