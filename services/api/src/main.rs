#[rocket::launch]
fn launch() -> rocket::Rocket {
    api::server(None, None)
}
