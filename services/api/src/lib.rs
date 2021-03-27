mod fairings;
mod handlers;
mod models;

// TODO: If possible, move this into the tests/ directory.
pub mod mocks;

pub fn server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
