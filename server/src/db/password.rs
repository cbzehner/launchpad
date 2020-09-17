use rocket_contrib::databases::diesel::prelude::*;

use super::schema::passwords;
use crate::models::Password;

#[derive(Debug, Queryable)]
pub struct QueryablePassword {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub digest: String,
    pub salt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "passwords"]
pub struct InsertablePassword<'a> {
    pub user_id: uuid::Uuid,
    pub digest: &'a str,
    pub salt: &'a str,
}

pub fn create(
    conn: &diesel::PgConnection,
    user_id: uuid::Uuid,
    password: &mut Password,
) -> Result<QueryablePassword, diesel::result::Error> {
    let password_digest = password
        .digest()
        .expect("Password from registration request has been consumed.");
    let new_password = InsertablePassword {
        user_id,
        digest: &password_digest,
        // TODO (security): Use an actual salt
        salt: "SALT",
    };
    diesel::insert_into(passwords::table)
        .values(new_password)
        .get_result::<QueryablePassword>(conn)
}
