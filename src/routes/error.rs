use rocket::response::Redirect;
use rocket::Request;

use crate::routes::auth;

/// Redirect all "401 Unauthorized" errors to the login page.
/// The current response isn't conformant to RFC 7235 due to
/// the omission of a "WWW-Authenticate" header in the response.
#[catch(401)]
pub fn unauthorized(_req: &Request) -> Redirect {
    Redirect::to(uri!(auth::login_page))
}

// #[catch(404)]
// pub fn not_found(req: &Request) -> Template {
//     let mut map = HashMap::new();
//     map.insert("path", req.uri().path());
//     Template::render("error/404", &map)
// }
