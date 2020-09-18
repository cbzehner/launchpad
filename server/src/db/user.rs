use rocket_contrib::databases::diesel::prelude::*;

use super::schema::users::{self, dsl};

#[derive(Debug, Queryable)]
pub struct QueryableUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub email: &'a str,
}

pub fn create(
    conn: &diesel::PgConnection,
    email: &str,
) -> Result<QueryableUser, diesel::result::Error> {
    let new_user = InsertableUser { email: email };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<QueryableUser>(conn)
}

pub fn from_id(
    conn: &diesel::PgConnection,
    user_id: uuid::Uuid,
) -> Result<QueryableUser, diesel::result::Error> {
    dsl::users.find(user_id).first(conn)
}

pub fn from_email(
    conn: &diesel::PgConnection,
    email: &str,
) -> Result<QueryableUser, diesel::result::Error> {
    dsl::users.filter(dsl::email.eq(email)).first(conn)
}
