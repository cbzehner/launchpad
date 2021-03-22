use rocket::response::content;

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Hello, world!"
}

#[get("/healthz")]
pub(crate) fn health_check() -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
