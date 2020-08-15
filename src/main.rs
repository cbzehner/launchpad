#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::convert;
use std::sync::Mutex;

use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FlashMessage, Form, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::{outcome::IntoOutcome, State};
use rocket_contrib::templates::Template;
use serde::Serialize;

struct AppState {
    users: Mutex<HashSet<User>>,
}

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

#[derive(FromForm)]
struct Registration {
    username: String,
    email: String,
    #[form(field = "preferred-name")]
    preferred_name: String,
    password: String, // TODO: Reject weak passwords with https://github.com/magiclen/passwords or https://github.com/shssoichiro/zxcvbn-rs
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
    preferred_name: String,
    password: String, // TODO: Change to password digest and implement hashing
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

    fn lookup_user_by_credentials(
        username: String,
        password: String,
        app_state: State<AppState>,
    ) -> Option<User> {
        match app_state.users.lock() {
            Ok(users) => match users
                .iter()
                .find(|user| user.username == username && user.password == password)
            {
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
    match User::lookup_user_by_credentials(
        login.username.clone(),
        login.password.clone(),
        app_state,
    ) {
        Some(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Ok(Redirect::to(uri!(index)))
        }
        None => Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid username/password.",
        )),
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
    let user_id = uuid::Uuid::new_v4();
    let error_msg = "An error occurred on our end while trying to sign you up. Please try again!";
    match app_state.users.lock() {
        Ok(mut users) => {
            let new_user = User {
                id: user_id,
                username: registration.username.clone(),
                email: registration.email.clone(),
                preferred_name: registration.preferred_name.clone(),
                password: registration.password.clone(), // TODO: Implement password hashing here
            };
            if users.contains(&new_user) {
                return Err(Flash::error(
                    Redirect::to(uri!(registration_page)),
                    error_msg,
                ));
            }
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

    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
    Ok(Flash::success(
        Redirect::to(uri!(index)),
        format!("Successfully registered as {}", registration.username),
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
