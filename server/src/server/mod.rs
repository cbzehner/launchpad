use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::db;
use crate::models::cache::Cache;
use crate::routes::{catchers, routes};

mod config;

/// Build Rocket server.
pub fn rocket() -> rocket::Rocket {
    let state = Cache {
        sessions: Mutex::new(HashMap::new()),
        users: Mutex::new(HashSet::new()),
    };

    dotenv().ok();
    rocket::custom(config::from_env())
        .attach(Template::fairing())
        .attach(db::Postgres::fairing())
        .manage(state)
        .mount("/", routes())
        .mount("/", StaticFiles::from("../public/root").rank(20))
        .mount("/public/css", StaticFiles::from("../public/css"))
        .register(catchers())
}
