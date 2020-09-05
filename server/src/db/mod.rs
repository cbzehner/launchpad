use rocket_contrib::databases::diesel::PgConnection;

pub mod models;
pub mod schema;

#[database("postgres")]
pub struct Conn(PgConnection);
