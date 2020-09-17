use rocket_contrib::databases::diesel::prelude::*;

use super::schema::users;

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
    // pub preferred_name: &'a str,
    pub email: &'a str,
}

pub fn create(
    conn: &diesel::PgConnection,
    email: &str,
) -> Result<QueryableUser, diesel::result::Error> {
    let new_user = InsertableUser { email };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<QueryableUser>(conn)
}
