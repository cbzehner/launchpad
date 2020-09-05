use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::models::user::User;
use crate::routes::auth;

#[get("/")]
pub fn user_index(user: User) -> Template {
    Template::render("pages/index", &user)
}

#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(auth::login_page))
}
