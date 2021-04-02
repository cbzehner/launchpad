#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use url::Url;

mod configs;
mod db;
mod fairings;
mod handlers;
mod models;

use configs::{KratosClient, PostgresProvider};

pub fn server(
    kratos_url: impl Into<Option<Url>>,
    postgres_url: impl Into<Option<Url>>,
) -> rocket::Rocket {
    let config = rocket::Config::figment().merge((
        "databases.postgres.url",
        PostgresProvider::new(postgres_url).to_string(),
    ));

    rocket::custom(config)
        .attach(db::Postgres::fairing())
        .attach(AdHoc::on_attach(
            "Postgres migrations",
            db::initialize::run_migrations,
        ))
        .manage(KratosClient::new(kratos_url))
        .mount("/", handlers::routes())
        .register("/", handlers::catchers())
}
