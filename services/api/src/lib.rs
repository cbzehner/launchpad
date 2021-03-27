mod fairings;
mod handlers;
pub mod models;

pub fn server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
