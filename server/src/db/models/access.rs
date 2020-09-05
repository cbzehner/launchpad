use diesel::pg::types::sql_types::Timestamptz;
use uuid::Uuid;

use crate::db::schema::access;

#[derive(Debug, Queryable)]
pub struct Access<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_type: &'a str,
    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

#[derive(Debug, Insertable)]
#[table_name = "access"]
pub struct InsertAccess<'a> {
    pub user_id: Uuid,
    pub access_type: &'a str,
    pub access_digest: &'a str,
}
