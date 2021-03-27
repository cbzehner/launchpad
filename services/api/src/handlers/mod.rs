use rocket::{Catcher, Route};

mod authenticated;
mod guest;

pub(crate) fn routes() -> Vec<Route> {
    rocket::routes![guest::index, guest::health_check, authenticated::whoami]
}

pub(crate) fn catchers() -> Vec<Catcher> {
    rocket::catchers![]
}
