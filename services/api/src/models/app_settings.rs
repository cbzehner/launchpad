use rocket_contrib::databases::diesel::prelude::*;
use uuid::Uuid;

use crate::db::{
    schema::app_settings::{self, dsl},
    Postgres,
};

#[derive(Debug, serde::Serialize, diesel::Queryable)]
pub struct AppSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AppSettings {
    /// Load the AppSettings for a user from the database. Each user has a single associated AppSettings row.
    /// Attempts to find an existing AppSettings row for the given user and if one does not exist, creates it.
    pub(crate) async fn from_user(
        user_id: Uuid,
        conn: &Postgres,
    ) -> Result<AppSettings, diesel::result::Error> {
        match AppSettings::find(user_id, conn).await {
            Ok(app_settings) => Ok(app_settings),
            Err(diesel::NotFound) => AppSettings::create(user_id, conn).await,
            Err(err) => Err(err),
        }
    }

    /// Create a new AppSettings row for a given user id.
    async fn create(user_id: Uuid, conn: &Postgres) -> Result<AppSettings, diesel::result::Error> {
        conn.run(move |c| {
            diesel::insert_into(app_settings::table)
                .values(dsl::user_id.eq(user_id))
                .on_conflict(dsl::user_id)
                .do_nothing()
                .get_result::<AppSettings>(c)
        })
        .await
    }

    /// Find an existing AppSettings row for a given user id.
    async fn find(user_id: Uuid, conn: &Postgres) -> Result<AppSettings, diesel::result::Error> {
        conn.run(move |c| {
            dsl::app_settings
                .filter(dsl::user_id.eq(user_id))
                .first::<AppSettings>(c)
        })
        .await
    }
}
