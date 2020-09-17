use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;

use crate::db::Postgres;
use crate::models::{Cache, Login, Registration, Session};
use crate::routes::auth;
use crate::routes::basic;

#[post("/api/auth/login", data = "<login>")]
pub fn login(
    cookies: Cookies,
    login: Form<Login>,
    cache: State<Cache>,
) -> Result<Redirect, Flash<Redirect>> {
    let error_msg = "Incorrect email or password";

    // TODO (performance): Both fetches in a single database query
    // Fetch user from the database
    // Fetch user's password digest from the database
    // Compare the fetched password to the login password provided
    // match login.password.verify_digest(&user.password_digest) {
    //         Some(()) => {
    //             Session::from(user).set_cookie(cookies);
    //             Ok(Redirect::to(uri!(basic::index)))
    //         }
    //         None => Err(Flash::error(
    //             Redirect::to(uri!(auth::login_page)),
    //             error_msg,
    //         )),
    //     }
    Err(Flash::error(
        Redirect::to(uri!(auth::login_page)),
        error_msg,
    ))
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
