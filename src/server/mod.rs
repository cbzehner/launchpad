use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

// use dotenv::dotenv;
use rocket_contrib::templates::Template;

use crate::models::cache::Cache;
use crate::routes::{catchers, routes};

// mod config;

/// Build Rocket server.
pub fn rocket() -> rocket::Rocket {
    let state = Cache {
        sessions: Mutex::new(HashMap::new()),
        users: Mutex::new(HashSet::new()),
    };

    rocket::ignite()
        .attach(Template::fairing())
        .manage(state)
        .mount("/", routes())
        .register(catchers())
}
