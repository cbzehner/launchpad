use std::convert::TryFrom;

use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;

use crate::models::cache::Cache;
use crate::models::login::Login;
use crate::models::registration::Registration;
use crate::models::session::Session;
use crate::models::user::User;
use crate::routes::auth;
use crate::routes::basic;

#[post("/api/auth/login", data = "<login>")]
pub fn login(
    cookies: Cookies,
    login: Form<Login>,
    cache: State<Cache>,
) -> Result<Redirect, Flash<Redirect>> {
    let error_msg = "Incorrect username or password";
    match User::lookup_user_by_credentials(login.username.clone(), cache) {
        Some(user) => match login.password.verify_digest(&user.password_digest) {
            Some(()) => {
                Session::from(user).set_cookie(cookies);
                Ok(Redirect::to(uri!(basic::index)))
            }
            None => Err(Flash::error(
                Redirect::to(uri!(auth::login_page)),
                error_msg,
            )),
        },
        None => Err(Flash::error(
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
    cache: State<Cache>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    // TODO: Guard against attempts to register existing usernames or emails with a RequestGuard
    let error_msg = "An error occurred on our end while trying to sign you up. Please try again!";
    let username = registration.username.clone();
    match cache.users.lock() {
        Ok(mut users) => {
            let new_user = User::try_from(&mut registration.into_inner());
            match new_user {
                Ok(new_user) => {
                    // TODO (performance): Optimize this solution which detects attempts to register
                    //      new users with usernames or passwords that conflict with existing users.
                    if users
                        .clone()
                        .into_iter()
                        .find(|user| {
                            user.email == new_user.email || user.username == new_user.username
                        })
                        .is_some()
                    {
                        return Err(Flash::error(
                            Redirect::to(uri!(auth::registration_page)),
                            error_msg,
                        ));
                    }
                    match users.insert(new_user.clone()) {
                        true => Session::from(new_user).set_cookie(cookies),
                        false => {
                            return Err(Flash::error(
                                Redirect::to(uri!(auth::registration_page)),
                                error_msg,
                            ))
                        }
                    }
                }
                Err(_) => {
                    return Err(Flash::error(
                        Redirect::to(uri!(auth::registration_page)),
                        error_msg,
                    ))
                }
            }
        }

        Err(_) => {
            return Err(Flash::error(
                Redirect::to(uri!(auth::registration_page)),
                error_msg,
            ))
        }
    }

    Ok(Flash::success(
        Redirect::to(uri!(basic::index)),
        format!("Successfully registered as {}", username),
    ))
}
