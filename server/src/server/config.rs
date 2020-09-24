use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

/// Build the application configuration.
pub fn build() -> Config {
    let env = Environment::active().expect("No environment found");

    // Use the presence or absence of CARGO_BIN_EXE_<name> to detect whether a test run is in progress and set the schema accordingly. ([source](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates))
    match std::env::var(format!("CARGO_BIN_EXE_{}", std::env!("CARGO_PKG_NAME"))) {
        Ok(_) => for_testing(env),
        Err(_) => from_env(env),
    }
}

/// Build Rocket config from environment set by dotenv.
fn from_env(env: Environment) -> Config {
    let port = std::env::var("ROCKET_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("ROCKET_PORT environmental variable should parse to an integer");

    let mut databases = HashMap::new();
    let postgres_config = configure_postgres(env, false);
    databases.insert("postgres", Value::from(postgres_config));

    Config::build(env)
        .environment(env)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

/// Build Rocket config for local testing.
fn for_testing(env: Environment) -> Config {
    let mut databases = HashMap::new();
    let postgres_config = configure_postgres(env, true);
    databases.insert("postgres", Value::from(postgres_config));

    Config::build(env)
        .environment(env)
        .port(0) // "Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application." ([source](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/#4-5-2-choosing-a-random-port))
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

fn configure_postgres<'a>(env: Environment, test_env: bool) -> HashMap<&'a str, Value> {
    let pg_user = env_default_for_dev("ROCKET_POSTGRES_USER", "user", env);
    let pg_password = env_default_for_dev("ROCKET_POSTGRES_PASSWORD", "password", env);
    let pg_location = std::env::var("ROCKET_POSTGRES_LOCATION").unwrap_or("localhost".into());
    let pg_port = std::env::var("ROCKET_POSTGRES_PORT").unwrap_or("5432".into());

    let pg_schema = std::env::var("ROCKET_POSTGRES_SCHEMA").unwrap_or("launchpad".into());
    let pg_schema = match test_env {
        true => format!("{}_testing", pg_schema),
        false => pg_schema,
    };

    let database_url = format!(
        "postgres://{user}:{password}@{location}:{port}/{schema}",
        user = pg_user,
        password = pg_password,
        location = pg_location,
        port = pg_port,
        schema = pg_schema
    );

    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from(database_url));

    database_config
}

/// Fetch the environment variable. If it's missing, use the default value in Development and panic in Production.
fn env_default_for_dev(value: &str, default_value: &str, env: Environment) -> String {
    match env {
        Environment::Development => {
            std::env::var(value).unwrap_or(default_value.into())
        },
        Environment::Staging | Environment::Production => {
            std::env::var(value).expect(&format!("Missing environment value for {}. Secure values must be set for non-developement builds.", value))
        }
    }
}

// TODO: Restructure so these tests aren't consistently flaky. `stress --runs 1000 -- cargo test postgres -j 1` shows a 35% failure rate.
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configure_postgres_none_set() {
        cleanup_env();

        let config = configure_postgres();
        let result = config.get("url").expect("postgres url should be present");
        let expected_value = "\"postgres://user:password@localhost:5432/launchpad\"";
        let result = assert_eq!(result.to_string(), expected_value);

        result
    }

    #[test]
    fn configure_postgres_all_set() {
        // setup environment variables
        env::set_var("ROCKET_POSTGRES_USER", "other_user");
        env::set_var("ROCKET_POSTGRES_PASSWORD", "123456");
        env::set_var("ROCKET_POSTGRES_LOCATION", "127.0.0.1");
        env::set_var("ROCKET_POSTGRES_PORT", "1234");
        env::set_var("ROCKET_POSTGRES_SCHEMA", "another_schema");

        let config = configure_postgres();
        let result = config.get("url").expect("postgres url should be present");
        let expected_value = "\"postgres://other_user:123456@127.0.0.1:1234/another_schema\"";
        let result = assert_eq!(result.to_string(), expected_value);

        result
    }

    #[test]
    fn configure_postgres_some_set() {
        cleanup_env();

        // setup environment variables
        env::set_var("ROCKET_POSTGRES_USER", "other_user");
        env::set_var("ROCKET_POSTGRES_SCHEMA", "another_schema");

        let config = configure_postgres();
        let result = config.get("url").expect("postgres url should be present");
        let expected_value = "\"postgres://other_user:password@localhost:5432/another_schema\"";
        let result = assert_eq!(result.to_string(), expected_value);

        result
    }

    // TODO: Why is calling `configure_postgres` necessary? Overly agressive inlining by the compiler?
    fn cleanup_env() {
        configure_postgres().get("url").unwrap().to_string();
        env::remove_var("ROCKET_POSTGRES_USER");
        env::remove_var("ROCKET_POSTGRES_PASSWORD");
        env::remove_var("ROCKET_POSTGRES_LOCATION");
        env::remove_var("ROCKET_POSTGRES_PORT");
        env::remove_var("ROCKET_POSTGRES_SCHEMA");
        configure_postgres().get("url").unwrap().to_string();
    }
}
*/
