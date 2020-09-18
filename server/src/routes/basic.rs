use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::models::{view_context::ViewContext, User};
use crate::routes::auth;

#[get("/")]
pub fn user_index(user: User) -> Template {
    let context = ViewContext {
        user: Some(user),
        ..ViewContext::default()
    };

    Template::render("pages/index", context)
}

#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(auth::login_page))
}
