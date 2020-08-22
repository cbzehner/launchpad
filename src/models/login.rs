use crate::models::password::Password;

#[derive(FromForm)]
pub struct Login<'r> {
    pub username: String,
    pub password: Password<'r>,
}
