use rocket_contrib::databases::diesel::prelude::*;

use super::schema::user_settings;

#[derive(Debug, Queryable)]
pub struct QueryableUserSettings {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub preferred_name: String,
    pub verified: bool,
    pub last_login_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "user_settings"]
pub struct InsertableUserSettings<'a> {
    pub user_id: uuid::Uuid,
    pub preferred_name: &'a str,
}

pub fn create(
    conn: &diesel::PgConnection,
    user_id: uuid::Uuid,
    preferred_name: &str,
) -> Result<QueryableUserSettings, diesel::result::Error> {
    let new_user_setting = InsertableUserSettings {
        user_id,
        preferred_name: preferred_name,
    };
    diesel::insert_into(user_settings::table)
        .values(new_user_setting)
        .get_result::<QueryableUserSettings>(conn)
}
