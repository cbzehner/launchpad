use rocket_contrib::databases::{database, diesel::PgConnection};

pub(crate) mod initialize;
pub(crate) mod schema;

#[database("postgres")]
pub(crate) struct Postgres(PgConnection);
