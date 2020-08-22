#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::convert;
use std::convert::TryFrom;
use std::sync::Mutex;

use rocket::http::RawStr;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, Form, FromFormValue, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use serde::Serialize;

struct AppState {
    users: Mutex<HashSet<User>>,
}

#[derive(FromForm)]
struct Login<'r> {
    username: String,
    password: Password<'r>,
}

#[derive(FromForm)]
struct Registration<'r> {
    username: String,
    email: String,
    #[form(field = "preferred-name")]
    preferred_name: String,
    password: Password<'r>,
}

#[derive(Copy, Clone)]
struct Password<'v>(&'v str);

impl<'v> std::fmt::Debug for Password<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

impl<'v> std::fmt::Display for Password<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

impl<'v> std::ops::Deref for Password<'v> {
    type Target = &'v str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'v> std::ops::DerefMut for Password<'v> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'v> FromFormValue<'v> for Password<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        // TODO: Validate password strength (see examples/form_validation)
        // Reject weak passwords with https://github.com/magiclen/passwords or https://github.com/shssoichiro/zxcvbn-rs
        Ok(Password(v.as_str()))
    }
}

impl<'a> Password<'a> {
    fn digest(&mut self) -> Option<String> {
        let raw_password = self.0;

        if raw_password.is_empty() {
            return None;
        }

        // TODO: Investigate whether there is a clever way to implement this.
        // Allow a password to be read once-and-only-once by replacing it with an empty string.
        self.0 = "";
        Some(self.securely_hash(raw_password))
    }

    fn securely_hash(self, password: &str) -> String {
        let params = ScryptParams::new(15, 8, 1).unwrap();
        // TODO (post-rocket 0.5): Provide the ROCKET_SECRET as a salt to scrypt.
        scrypt_simple(password, &params).expect("OS RNG should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn test_opaque_debug() {
        let password = Password("NeverLeakAPassword");
        assert_eq!(format!("{:?}", password), "Password {..}");
    }

    #[test]
    fn test_opaque_display() {
        let password = Password("NeverLeakAPassword");
        assert_eq!(format!("{}", password), "Password {..}");
    }

    #[test]
    fn test_digest_once() {
        let mut password = Password("NeverLeakAPassword");
        assert_eq!(password.digest().is_some(), true);
        assert_eq!(password.digest().is_none(), true);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
    preferred_name: String,
    password_digest: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        let app_state = request.guard::<State<AppState>>().unwrap();

        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User::lookup_user_by_id(id, app_state.inner()))
            .and_then(convert::identity) // This is the same as the experimental Option::flatten()
            .or_forward(())
    }
}

impl std::convert::TryFrom<&mut Registration<'_>> for User {
    type Error = &'static str;

    fn try_from(registration: &mut Registration) -> Result<Self, Self::Error> {
        match registration.password.digest() {
            Some(password_digest) => {
                let user_id = uuid::Uuid::new_v4();
                Ok(User {
                    id: user_id,
                    username: registration.username.clone(),
                    email: registration.email.clone(),
                    preferred_name: registration.preferred_name.clone(),
                    password_digest,
                })
            }
            None => Err("Password from registration request has been consumed."),
        }
    }
}

impl User {
    fn lookup_user_by_id(id: uuid::Uuid, app_state: &AppState) -> Option<User> {
        match app_state.users.lock() {
            Ok(users) => match users.iter().find(|user| user.id == id) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    fn lookup_user_by_credentials(username: String, app_state: State<AppState>) -> Option<User> {
        match app_state.users.lock() {
            Ok(users) => match users.iter().find(|user| user.username == username) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }
}

#[post("/login", data = "<login>")]
fn login(
    mut cookies: Cookies,
    login: Form<Login>,
    app_state: State<AppState>,
) -> Result<Redirect, Flash<Redirect>> {
    let error_msg = "Incorrect username or password";
    match User::lookup_user_by_credentials(login.username.clone(), app_state) {
        Some(user) => match scrypt_check(login.password.0, &user.password_digest) {
            Ok(()) => {
                cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                Ok(Redirect::to(uri!(index)))
            }
            Err(_) => Err(Flash::error(Redirect::to(uri!(login_page)), error_msg)),
        },
        None => Err(Flash::error(Redirect::to(uri!(login_page)), error_msg)),
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
fn loggedin_user(_user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[post("/register", data = "<registration>")]
fn register(
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
                            Redirect::to(uri!(registration_page)),
                            error_msg,
                        ));
                    }
                    user_id = new_user.id;
                    match users.insert(new_user) {
                        true => (),
                        false => {
                            return Err(Flash::error(
                                Redirect::to(uri!(registration_page)),
                                error_msg,
                            ))
                        }
                    }
                }
                Err(_) => {
                    return Err(Flash::error(
                        Redirect::to(uri!(registration_page)),
                        error_msg,
                    ))
                }
            }
        }

        Err(_) => {
            return Err(Flash::error(
                Redirect::to(uri!(registration_page)),
                error_msg,
            ))
        }
    }

    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
    Ok(Flash::success(
        Redirect::to(uri!(index)),
        format!("Successfully registered as {}", username),
    ))
}

#[get("/register")]
fn registered_user(_user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/register", rank = 2)]
fn registration_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("register", &context)
}

#[get("/")]
fn user_index(user: User) -> Template {
    Template::render("index", &user)
}

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to(uri!(login_page))
}

fn rocket() -> rocket::Rocket {
    let state = AppState {
        users: Mutex::new(HashSet::new()),
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                loggedin_user,
                login,
                login_page,
                logout,
                register,
                registered_user,
                registration_page,
                user_index,
            ],
        )
        .manage(state)
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
