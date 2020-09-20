use rocket_contrib::databases::diesel::prelude::*;

use super::schema::passwords::{self, dsl};
use crate::models::Password;

#[derive(Debug, Queryable)]
pub struct QueryablePassword {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub digest: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "passwords"]
pub struct InsertablePassword<'a> {
    pub user_id: uuid::Uuid,
    pub digest: &'a str,
}

pub fn create(
    conn: &diesel::PgConnection,
    user_id: uuid::Uuid,
    password: &mut Password,
) -> Result<usize, diesel::result::Error> {
    let password_digest = password.digest();
    let new_password = InsertablePassword {
        user_id,
        digest: &password_digest,
    };
    diesel::insert_into(passwords::table)
        .values(new_password)
        .execute(conn)
}

pub fn from_user_id(
    conn: &diesel::PgConnection,
    user_id: uuid::Uuid,
) -> Result<QueryablePassword, diesel::result::Error> {
    dsl::passwords.filter(dsl::user_id.eq(user_id)).first(conn)
}
