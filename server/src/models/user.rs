use std::convert::{identity, TryFrom};

use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::request::{Outcome, Request};
use rocket::State;
use serde::Serialize;

use super::session::Session;
use crate::models::Cache;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub preferred_name: String,
}

impl User {
    pub fn lookup_user_by_id(id: uuid::Uuid, cache: &Cache) -> Option<User> {
        match cache.users.lock() {
            Ok(users) => match users.iter().find(|user| user.id == id) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn lookup_user_by_credentials(email: String, cache: State<Cache>) -> Option<User> {
        match cache.users.lock() {
            Ok(users) => match users.iter().find(|user| user.email == email) {
                Some(user) => Some((*user).clone()),
                None => None,
            },
            Err(_) => None,
        }
    }
}

// TODO (api ergonomics): Write a request guard for valid sessions. Otherwise redirect to the login screen.

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, !> {
        let cache = request.guard::<State<Cache>>().unwrap();

        request
            .cookies()
            .get_private("session")
            .and_then(|cookie| Session::try_from(cookie).ok())
            .map(|session| User::lookup_user_by_id(session.user_id, cache.inner()))
            .and_then(identity) // This is the same as the experimental Option::flatten()
            .or_forward(())
    }
}

// TODO (DRY): How to share user model building code between Registration and Login?
// Maybe have a function that takes user_row and user_settings row and builds the User...
// impl<'r> std::convert::TryFrom<models::Registration<'r>> for User {
//     type Error = &'static str;
//     fn try_from(registration: models::Registration) -> Result<Self, Self::Error> {
//         ...how to access DB connection?
//     }
// }
