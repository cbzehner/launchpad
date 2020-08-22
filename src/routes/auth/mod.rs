use std::collections::HashMap;

use rocket::{request::FlashMessage, response::Redirect};
use rocket_contrib::templates::Template;

use crate::models::user::User;
use crate::routes::basic;

#[get("/login")]
pub fn loggedin_user(_user: User) -> Redirect {
    Redirect::to(uri!(basic::index))
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[get("/register")]
pub fn registered_user(_user: User) -> Redirect {
    Redirect::to(uri!(basic::index))
}

#[get("/register", rank = 2)]
pub fn registration_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("register", &context)
}
