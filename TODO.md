# TODOs

## api
1. Use dotenv instead of hard-coded secret values.
1. Setup authentication verification with Kratos auth service.
1. Write a test verifying protected routes.
1. Default all routes to Protected and write a "Guest" data guard to opt-routes from this behavior.

## services
1. Use dotenv instead of hard-coded secret values.
1. Store Kratos data in PostgreSQL rather than SQLite.
1. Setup email service with Kratos auth service.

## web
1. ~~Use dotenv instead of hard-coded secret values.~~
1. Have an actual login experience on the Home page.
1. Support logout from menu bar.

## deployment
1. Use dotenv instead of hard-coded secret values in Docker.
1. Create a `deployments/containers/` folder and set up a `just` command for running `docker-compose up --remove-orphans`.
1. Deploy an unhardened "production" instance
1. Harden production instance.

## ci/cd
1. Set up GitHub actions to run tests for each service, the api and the web client.
1. Set up GitHub Action to deploy green builds to production

## meta
1. Create a "book" directory and start writing an overview of launchpad.
