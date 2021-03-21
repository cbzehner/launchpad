use rocket::{response::content, Catcher, Route};

pub(crate) fn routes() -> Vec<Route> {
    routes![index, health_check]
}

pub(crate) fn catchers() -> Vec<Catcher> {
    catchers![]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/healthz")]
fn health_check() -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
