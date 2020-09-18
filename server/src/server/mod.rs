use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::db;
use crate::routes::{catchers, routes};

mod config;

/// Build Rocket server.
pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .attach(Template::fairing())
        .attach(db::Postgres::fairing())
        .mount("/", routes())
        .mount("/", StaticFiles::from("../public/root").rank(20))
        .mount("/public/css", StaticFiles::from("../public/css"))
        .register(catchers())
}
