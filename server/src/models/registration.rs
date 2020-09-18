use crate::db;
use crate::models::{Password, User};

#[derive(FromForm)]
pub struct Registration<'a> {
    pub email: String,
    #[form(field = "preferred-name")]
    pub preferred_name: String,
    pub password: Password<'a>,
}

impl<'a> Registration<'a> {
    pub fn create_user(
        &mut self,
        conn: &diesel::PgConnection,
    ) -> Result<User, diesel::result::Error> {
        // TODO: Open a transaction
        let user_row = db::user::create(conn, &self.email)?;
        db::password::create(conn, user_row.id, &mut self.password)?;
        let settings_row = db::user_settings::create(conn, user_row.id, &self.preferred_name)?;
        // TODO: Close the transaction

        Ok(User::from_db(user_row, settings_row))
    }
}
