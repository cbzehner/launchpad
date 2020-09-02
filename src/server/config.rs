use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

/// Build Rocket config from environment set by dotenv.
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("ROCKET_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("ROCKET_PORT environmental variable should parse to an integer");

    let mut databases = HashMap::new();
    let postgres_config = configure_postgres();
    databases.insert("postgres", Value::from(postgres_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

// TODO: Match on environment to use a different database for testing
fn configure_postgres<'a>() -> HashMap<&'a str, Value> {
    let pg_user = env::var("ROCKET_POSTGRES_USER").unwrap_or("user".into());
    let pg_password = env::var("ROCKET_POSTGRES_PASSWORD").unwrap_or("password".into());
    let pg_location = env::var("ROCKET_POSTGRES_LOCATION").unwrap_or("localhost".into());
    let pg_port = env::var("ROCKET_POSTGRES_PORT").unwrap_or("5432".into());
    let pg_schema = env::var("ROCKET_POSTGRES_SCHEMA").unwrap_or("launchpad".into());

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
