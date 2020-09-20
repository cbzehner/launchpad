use crate::db;
use crate::models::{Password, User};

#[derive(FromForm)]
pub struct Login {
    pub email: String,
    pub password: Password,
}

impl Login {
    pub fn retrieve_user(
        &mut self,
        conn: &diesel::PgConnection,
    ) -> Result<User, diesel::result::Error> {
        // TODO: Open a transaction
        match db::user::from_email(conn, &self.email) {
            Ok(user_row) => {
                let password_row = db::password::from_user_id(conn, user_row.id)?;
                match self.password.verify_digest(&password_row.digest) {
                    Ok(_) => {
                        let settings_row = db::user_settings::from_user_id(conn, user_row.id)?;
                        Ok(User::from_db(user_row, settings_row))
                    }
                    // TODO (idioms): Improve error handling for this function
                    Err(_) => Err(diesel::result::Error::NotFound),
                }
            }
            Err(error) => {
                // Avoid leaking the presence or absense of a user by doing the same work as if a user was found in the database and their password needed to be checked.
                Password::do_work();
                Err(error)
            }
        }
        // TODO: Close the transaction
    }
}
