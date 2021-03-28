use url::Url;

mod clients;
mod fairings;
mod handlers;
mod models;

use clients::KratosClient;

pub fn server(kratos_url: impl Into<Option<Url>>) -> rocket::Rocket {
    let kratos_url = match kratos_url.into() {
        Some(url) => url,
        None => Url::parse("http://127.0.0.1:4433").unwrap(),
    };

    rocket::ignite()
        .manage(KratosClient::new(kratos_url))
        .mount("/", handlers::routes())
        .register(handlers::catchers())
}
