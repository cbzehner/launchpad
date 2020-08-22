use rocket::{Catcher, Route};

mod api;
mod auth;
mod basic;
// mod template;
// mod error;

pub fn routes() -> Vec<Route> {
    routes![
        api::auth::login,
        api::auth::register,
        auth::loggedin_user,
        auth::login_page,
        api::auth::logout,
        auth::registered_user,
        auth::registration_page,
        basic::index,
        basic::user_index,
    ]
}

pub fn catchers() -> Vec<Catcher> {
    // catchers![error::not_found]
    catchers![]
}
