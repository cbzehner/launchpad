use std::convert::TryFrom;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;

use crate::models::app::AppState;
use crate::models::login::Login;
use crate::models::registration::Registration;
use crate::models::user::User;
use crate::routes::auth;
use crate::routes::basic;

#[post("/login", data = "<login>")]
pub fn login(
    mut cookies: Cookies,
    login: Form<Login>,
    app_state: State<AppState>,
) -> Result<Redirect, Flash<Redirect>> {
    let error_msg = "Incorrect username or password";
    match User::lookup_user_by_credentials(login.username.clone(), app_state) {
        Some(user) => match login.password.verify_digest(&user.password_digest) {
            Some(()) => {
                cookies.add_private(Cookie::new("user_id", user.id.to_string()));
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

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(
        Redirect::to(uri!(auth::login_page)),
        "Successfully logged out.",
    )
}

#[post("/register", data = "<registration>")]
pub fn register(
    mut cookies: Cookies,
    registration: Form<Registration>,
    app_state: State<AppState>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    // TODO: Guard against attempts to register existing usernames or emails with a RequestGuard
    let error_msg = "An error occurred on our end while trying to sign you up. Please try again!";
    let username = registration.username.clone();
    let mut user_id: uuid::Uuid = uuid::Uuid::default();
    match app_state.users.lock() {
        Ok(mut users) => {
            let new_user = User::try_from(&mut registration.into_inner());
            match new_user {
                Ok(new_user) => {
                    if users.contains(&new_user) {
                        return Err(Flash::error(
                            Redirect::to(uri!(auth::registration_page)),
                            error_msg,
                        ));
                    }
                    user_id = new_user.id;
                    match users.insert(new_user) {
                        true => (),
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

    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
    Ok(Flash::success(
        Redirect::to(uri!(basic::index)),
        format!("Successfully registered as {}", username),
    ))
}
