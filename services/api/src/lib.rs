mod fairings;
mod handlers;
mod models;

pub fn server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
