use chrono::{DateTime, Duration, Utc};
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::request::{Outcome, Request};

use crate::models::User;
use std::convert::TryFrom;

/// A session cookie that stores common information about the user.  Prefer storing
/// static, non-sensitive information in the session since it can be accessed by each
/// request without additional load on the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub user: User,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl<'a> Session {
    pub fn set_cookie(self, mut cookies: Cookies) -> () {
        let json_cookie = serde_json::to_string(&self).unwrap();
        cookies.add_private(Cookie::new("session", json_cookie));
    }

    pub fn remove_cookie(self, mut cookies: Cookies) -> () {
        let json_cookie = serde_json::to_string(&self).unwrap();
        cookies.remove_private(Cookie::new("session", json_cookie));
    }
}

/// Implement a [Request Guard](https://rocket.rs/v0.4/guide/requests/#request-guards) for loading the Session stored in the client-side cookies
impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Session, !> {
        request
            .cookies()
            .get_private("session")
            .and_then(|cookie| Session::try_from(cookie).ok())
            .or_forward(())
    }
}

/// Convert a User into a Session
impl std::convert::From<User> for Session {
    fn from(user: User) -> Self {
        Session {
            id: uuid::Uuid::new_v4(),
            user,
            issued_at: Utc::now(),
            expires_at: Utc::now() + Duration::weeks(1), // TODO (DRY): Better align this with Rocket's default Cookie expires setting.
        }
    }
}

/// Try to convert a cookie into a Session
impl<'a> std::convert::TryFrom<Cookie<'a>> for Session {
    type Error = std::io::Error;

    fn try_from(cookie: Cookie) -> Result<Self, Self::Error> {
        let session: Session = serde_json::from_str(cookie.value())?;
        // TODO (security): Verify that the session isn't expired before allowing conversion.
        Ok(session)
    }
}
