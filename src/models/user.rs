use std::convert;

use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::request::{Outcome, Request};
use rocket::State;
use serde::Serialize;

use crate::models::app::AppState;
use crate::models::registration::Registration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    // TODO: Remove this from the model and verification in the DB
    pub password_digest: String,
}

impl User {
    pub fn lookup_user_by_id(id: uuid::Uuid, app_state: &AppState) -> Option<User> {
        match app_state.users.lock() {
            Ok(users) => match users.iter().find(|user| user.id == id) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn lookup_user_by_credentials(
        username: String,
        app_state: State<AppState>,
    ) -> Option<User> {
        match app_state.users.lock() {
            Ok(users) => match users.iter().find(|user| user.username == username) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, !> {
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
