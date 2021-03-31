#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use url::Url;

mod clients;
mod db;
mod fairings;
mod handlers;
mod models;

use clients::KratosClient;

pub fn server(kratos_url: impl Into<Option<Url>>) -> rocket::Rocket {
    // TODO: Clean up the input configs into a proper Figment config object...
    let kratos_url = match kratos_url.into() {
        Some(url) => url,
        None => Url::parse("http://127.0.0.1:4433").unwrap(),
    };

    rocket::ignite()
        .attach(db::Postgres::fairing())
        .attach(AdHoc::on_attach(
            "Postgres migrations",
            db::initialize::run_migrations,
        ))
        .manage(KratosClient::new(kratos_url))
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
