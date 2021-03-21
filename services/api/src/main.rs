#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod fairings;
mod handlers;
mod models;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
