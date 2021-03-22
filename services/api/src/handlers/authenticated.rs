use rocket::response::content;

use crate::models::User;

#[get("/whoami")]
pub(crate) fn whoami(user: User) -> Result<content::Json<String>, String> {
    let json_value = serde_json::to_string(&user);

    match json_value {
        Ok(json_value) => Ok(content::Json(json_value)),
        Err(err) => {
            // TODO: Verify this can never occur.
            Err(err.to_string())
        }
    }
}
