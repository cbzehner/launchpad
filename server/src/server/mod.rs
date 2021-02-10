use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::db;
use crate::routes::{catchers, routes};

pub(crate) mod config;

/// Build Rocket server.
pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::build())
        .attach(Template::fairing())
        .attach(db::Postgres::fairing())
        .attach(AdHoc::on_attach(
            "Database migrations",
            db::run_db_migrations,
        ))
        .mount("/", routes())
        .mount("/", StaticFiles::from("../public/root").rank(20))
        .mount("/public/css", StaticFiles::from("../public/css"))
        .register(catchers())
}
