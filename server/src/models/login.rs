use crate::models::password::Password;

#[derive(FromForm)]
pub struct Login<'r> {
    pub email: String,
    pub password: Password<'r>,
}
