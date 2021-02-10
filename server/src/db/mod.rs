use rocket::config::Environment;
use rocket::Rocket;
use rocket_contrib::databases::diesel::PgConnection;

pub(crate) mod password;
pub(crate) mod schema;
pub(crate) mod user;
pub(crate) mod user_settings;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("postgres")]
pub struct Postgres(PgConnection);

pub(crate) fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = Postgres::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            // TODO (observability): Use trace-style logging everywhere
            println!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

/// Build the Postgres database URL based on the current environment.
pub(crate) fn database_url() -> String {
    let env = Environment::active().expect("No environment found");

    let pg_user = env_default_for_dev("ROCKET_POSTGRES_USER", "user", &env);
    let pg_password = env_default_for_dev("ROCKET_POSTGRES_PASSWORD", "password", &env);
    let pg_location = std::env::var("ROCKET_POSTGRES_LOCATION").unwrap_or("localhost".into());
    let pg_port = std::env::var("ROCKET_POSTGRES_PORT").unwrap_or("5432".into());

    let pg_schema = std::env::var("ROCKET_POSTGRES_SCHEMA").unwrap_or("launchpad".into());
    // let pg_schema = match cfg!(test) {
    //     // TODO: Change _testing -> _test and create a setup process
    //     true => format!("{}_testing", pg_schema),
    //     false => pg_schema,
    // };

    format!(
        "postgres://{user}:{password}@{location}:{port}/{schema}",
        user = pg_user,
        password = pg_password,
        location = pg_location,
        port = pg_port,
        schema = pg_schema
    )
}

/// Fetch the environment variable. If it's missing, use the default value in Development and panic in Production.
fn env_default_for_dev(value: &str, default_value: &str, env: &Environment) -> String {
    match env {
        Environment::Development => {
            std::env::var(value).unwrap_or(default_value.into())
        },
        Environment::Staging | Environment::Production => {
            std::env::var(value).expect(&format!("Missing environment value for {}. Secure values must be set for non-developement builds.", value))
        }
    }
}

// TODO (testing): Restructure so these tests aren't consistently flaky. `stress --runs 1000 -- cargo test database_url_postgres -j 1` shows a 35% failure rate.
#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    
    #[test]
    fn database_url_postgres_none_set() {
        let expected_value = "postgres://user:password@localhost:5432/launchpad";
        assert_eq!(database_url(), expected_value);
    }

    #[test]
    #[ignore = "Depends on environmental variables, causing race conditions when run in parallel"]
    fn database_url_postgres_all_set() {
        // setup environment variables
        env::set_var("ROCKET_POSTGRES_USER", "other_user");
        env::set_var("ROCKET_POSTGRES_PASSWORD", "123456");
        env::set_var("ROCKET_POSTGRES_LOCATION", "127.0.0.1");
        env::set_var("ROCKET_POSTGRES_PORT", "1234");
        env::set_var("ROCKET_POSTGRES_SCHEMA", "another_schema");

        let expected_value = "postgres://other_user:123456@127.0.0.1:1234/another_schema";
        assert_eq!(database_url(), expected_value);

        cleanup_env();
    }

    #[test]
    #[ignore = "Depends on environmental variables, causing race conditions when run in parallel"]
    fn database_url_postgres_some_set() {
        // setup environment variables
        env::set_var("ROCKET_POSTGRES_USER", "other_user");
        env::set_var("ROCKET_POSTGRES_SCHEMA", "another_schema");

        let expected_value = "postgres://other_user:password@localhost:5432/another_schema";
        assert_eq!(database_url(), expected_value);

        cleanup_env();
    }

    fn cleanup_env() {
        env::remove_var("ROCKET_POSTGRES_USER");
        env::remove_var("ROCKET_POSTGRES_PASSWORD");
        env::remove_var("ROCKET_POSTGRES_LOCATION");
        env::remove_var("ROCKET_POSTGRES_PORT");
        env::remove_var("ROCKET_POSTGRES_SCHEMA");
    }
}
