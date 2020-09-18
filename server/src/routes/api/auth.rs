use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use crate::db::Postgres;
use crate::models::{Login, Registration, Session};
use crate::routes::auth;
use crate::routes::basic;

#[post("/api/auth/login", data = "<login>")]
pub fn login(
    cookies: Cookies,
    login: Form<Login>,
    conn: Postgres,
) -> Result<Redirect, Flash<Redirect>> {
    let error_msg = "Incorrect email or password";

    match login.into_inner().retrieve_user(&conn) {
        Ok(user) => {
            Session::from(user).set_cookie(cookies);
            Ok(Redirect::to(uri!(basic::index)))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(auth::login_page)),
            error_msg,
        )),
    }
}

#[post("/api/auth/logout")]
pub fn logout(session: Session, cookies: Cookies) -> Flash<Redirect> {
    session.remove_cookie(cookies);
    Flash::success(
        Redirect::to(uri!(auth::login_page)),
        "Successfully logged out.",
    )
}

#[post("/api/auth/register", data = "<registration>")]
pub fn register(
    cookies: Cookies,
    registration: Form<Registration>,
    conn: Postgres,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let error_msg = "An error occurred on our end while trying to sign you up. Please try again!";
    let email = registration.email.clone();
    match registration.into_inner().create_user(&conn) {
        Ok(new_user) => {
            Session::from(new_user).set_cookie(cookies);
            Ok(Flash::success(
                Redirect::to(uri!(basic::index)),
                format!("Successfully registered an account with {}", email),
            ))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(auth::registration_page)),
            error_msg,
        )),
    }
}
