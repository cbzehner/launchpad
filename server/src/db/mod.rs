use rocket_contrib::databases::diesel;

pub(crate) mod password;
pub(crate) mod schema;
pub(crate) mod user;
pub(crate) mod user_settings;

#[database("postgres")]
pub struct Postgres(diesel::PgConnection);
