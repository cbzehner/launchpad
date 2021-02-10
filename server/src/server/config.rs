use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

use crate::db;

/// Build the application configuration.
pub fn build() -> Config {
    let env = Environment::active().expect("No environment found");
    let port = match cfg!(test) {
        true => 0, // "Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application." ([source](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/#4-5-2-choosing-a-random-port))
        false => std::env::var("ROCKET_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .expect("ROCKET_PORT environmental variable should parse to an integer"),
    };

    let mut databases = HashMap::new();
    databases.insert("postgres", Value::from(configure_postgres()));

    Config::build(env)
        .environment(env)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

/// Set up the Postgres configuration based on the current environment.
fn configure_postgres<'a>() -> HashMap<&'a str, Value> {
    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from(db::database_url()));

    database_config
}
