use std::convert::TryFrom;

use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::request::{Outcome, Request};

use super::session::Session;
use crate::db;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub preferred_name: String,
}

impl User {
    pub fn from_db(
        user: db::user::QueryableUser,
        settings: db::user_settings::QueryableUserSettings,
    ) -> Self {
        User {
            id: user.id,
            email: user.email,
            preferred_name: settings.preferred_name,
        }
    }
}

// Note: I'm actually reconsidering this TODO because this creates the need to create a blacklist or whitelist
// of all currently active (or blacklisted) sessions. Right now I'm leaning towards the approach of "fat" sessions
// that store most of the information in private cookies on the user's device and try to minimize the amount of trips
// to the database ("chatty app" syndrome).
// In the future it may make sense to store a bloom filter or more advanced data structure with just session ids that
// can be used for server-side confirmation of the sessions, but I'm hoping to keep the database out of it.
// TODO (api ergonomics): Write a request guard for valid sessions. Otherwise redirect to the login screen.

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, !> {
        request
            .cookies()
            .get_private("session")
            .and_then(|cookie| Session::try_from(cookie).ok())
            .map(|session| session.user)
            .or_forward(())
    }
}
