use rocket::Rocket;

use super::Postgres;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!(/* ./migrations */);

pub(crate) async fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    Postgres::get_one(&rocket)
        .await
        .expect("Postgres connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                eprintln!("Failed to run Postgres migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}
