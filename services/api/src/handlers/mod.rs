use rocket::{Catcher, Route};

pub(crate) fn routes() -> Vec<Route> {
    routes![hello_world]
}

pub(crate) fn catchers() -> Vec<Catcher> {
    catchers![]
}

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}
