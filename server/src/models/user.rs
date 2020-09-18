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
