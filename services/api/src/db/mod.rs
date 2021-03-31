use rocket_contrib::databases;

pub(crate) mod initialize;
pub(crate) mod schema;

#[rocket_contrib::database("postgres")]
pub(crate) struct Postgres(databases::diesel::PgConnection);
