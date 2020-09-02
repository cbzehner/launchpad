use diesel::pg::types::sql_types::Timestamptz;
use uuid::Uuid;

use crate::db::schema::users;

#[derive(Debug, Queryable)]
pub struct User<'a> {
    pub id: Uuid,
    pub preferred_name: &'a str,
    pub email: &'a str,
    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub preferred_name: &'a str,
    pub email: &'a str,
}
