use crate::models::password::Password;

#[derive(FromForm)]
pub struct Registration<'r> {
    pub username: String,
    pub email: String,
    #[form(field = "preferred-name")]
    pub preferred_name: String,
    pub password: Password<'r>,
}
