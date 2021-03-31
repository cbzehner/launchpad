use email_address::EmailAddress;
use rocket::{
    request::{self, FromRequest, Outcome, Request},
    try_outcome,
};
use uuid::Uuid;

use crate::db::Postgres;

use super::{kratos, AppSettings};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) email: EmailAddress,
    pub(crate) full_name: String,
    pub(crate) preferred_name: String,
}

impl User {
    /// Load the AppSettings for a user from the database.
    pub(crate) async fn load_app_settings(&self, conn: &Postgres) -> Result<AppSettings, ()> {
        match AppSettings::from_user(self.id, conn).await {
            Ok(app_settings) => Ok(app_settings),
            Err(err) => {
                eprintln!("{}", err.to_string());
                Err(())
            }
        }
    }
}

impl From<kratos::Session> for User {
    fn from(session: kratos::Session) -> Self {
        Self {
            id: session.identity.id,
            email: session.identity.traits.email,
            full_name: session.identity.traits.name.full,
            preferred_name: session.identity.traits.name.preferred,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = (); // TODO: Better error handling?

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session = try_outcome!(req.guard::<kratos::Session>().await);
        Outcome::Success(session.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bear::fixture;

    use super::*;

    #[test]
    fn models_user_deserialize_from_session() {
        let expected = User {
            id: Uuid::parse_str("53354449-9e03-40fc-bc90-fc499e6d44e3").unwrap(),
            email: EmailAddress::from_str("cbzehner@test.com").unwrap(),
            full_name: "Christopher Zehner".to_owned(),
            preferred_name: "Chris".to_owned(),
        };

        let data = fixture("kratos/session.json");
        let session = serde_json::from_str::<kratos::Session>(&data).unwrap();
        let result = User::from(session);
        assert_eq!(expected, result);
    }
}
