use rocket::response::content;

#[rocket::get("/")]
pub(crate) fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/healthz")]
pub(crate) fn health_check() -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
