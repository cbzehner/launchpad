use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use rocket::{
    http::{Cookie, Status},
    request::{FromRequest, Outcome, Request},
    try_outcome, State,
};
use url::Url;
use uuid::Uuid;

use crate::configs::KratosClient;

const SESSION_COOKIE_NAME: &str = "ory_kratos_session";

#[derive(serde::Deserialize)]
pub(crate) struct Session {
    id: Uuid,
    active: bool,
    expires_at: DateTime<Utc>,
    authenticated_at: DateTime<Utc>,
    issued_at: DateTime<Utc>,
    pub(crate) identity: Identity,
}

#[derive(serde::Deserialize)]
pub(crate) struct Identity {
    pub(crate) id: Uuid,
    schema_id: SchemaId,
    schema_url: Url,
    pub(crate) traits: Traits,
    verifiable_addresses: Vec<VerifiedAddress>,
    recovery_addresses: Vec<RecoveryAddress>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum SchemaId {
    Default,
}

#[derive(serde::Deserialize)]
pub(crate) struct Traits {
    pub(crate) name: Name,
    pub(crate) email: EmailAddress,
}

#[derive(serde::Deserialize)]
pub(crate) struct Name {
    pub(crate) full: String,
    pub(crate) preferred: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct VerifiedAddress {
    id: Uuid,
    value: EmailAddress,
    verified: bool,
    via: VerificationMethod,
    status: VerificationStatus,
    verified_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum VerificationMethod {
    Email,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum VerificationStatus {
    Pending,
    Confirmed, // TODO: Verify this status
}

#[derive(serde::Deserialize)]
pub(crate) struct RecoveryAddress {
    id: Uuid,
    value: EmailAddress,
    via: VerificationMethod,
}

impl<'r> Session {
    // TODO: Write a test mocking the value returned by reqwest (including invalid values)
    async fn validate(kratos_url: Url, cookie: &Cookie<'r>) -> Result<Self, reqwest::Error> {
        // TODO: Move client into Rocket State
        let session = reqwest::Client::new()
            // TODO: Avoid hard-coding the auth server public URL!
            .get(kratos_url.join("sessions/whoami").unwrap())
            .header("Cookie", format!("{}", cookie))
            .send()
            .await?
            .json::<Session>()
            .await?;

        Ok(session)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    // TODO: Convert errors back to something meaningful
    // type Error = String; // TODO: Better error handling
    type Error = ();

    // TODO: Use `local_cache_async` to cache session guard executions...
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract the authentication session.
        let cookie = match req.cookies().get(SESSION_COOKIE_NAME) {
            Some(cookie) => cookie,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    (), // "No session cookie provided".into(),
                ));
            }
        };

        let state: State<KratosClient> = try_outcome!(req.guard::<State<KratosClient>>().await);
        // TODO: Clean this up!
        let kratos_url = (*state.inner()).clone();
        // Confirm the session against the authentication server.
        let session = match Session::validate(kratos_url, cookie).await {
            Ok(session) => session,
            Err(_err) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    (), /* err.to_string() */
                ));
            }
        };

        if !session.active {
            return Outcome::Failure((
                Status::Unauthorized,
                (), /* "Inactive session".into() */
            ));
        }

        if session.expires_at < Utc::now() {
            return Outcome::Failure((
                Status::Unauthorized,
                (), /* "Expired session".into() */
            ));
        }

        Outcome::Success(session)
    }
}

#[cfg(test)]
mod tests {
    use bear::fixture;

    use super::*;

    #[test]
    fn models_kratos_session_deserialize() {
        let data = fixture("kratos/session.json");
        let result = serde_json::from_str::<Session>(&data);
        assert!(result.is_ok())
    }
}
