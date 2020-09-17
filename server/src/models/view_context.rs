use serde::Serialize;

use crate::models::User;

#[derive(Default, Serialize)]
pub(crate) struct ViewContext {
    pub flash: Option<String>,
    pub form: Option<Form>,
    pub user: Option<User>,
    pub debug: bool, // TODO: Calculate this based on Rocket env. Maybe add a Header to all requests in development?
}

#[derive(Serialize)]
pub(crate) struct Form {
    pub action: String,
    pub method: String, // TODO: Stronger types
    pub submit_text: String,
    pub cancel_text: Option<String>,
    pub secondary: Option<Link>,
    pub rows: Vec<FormRow>,
}

#[derive(Serialize)]
pub(crate) struct FormRow {
    pub label: String,
    pub r#type: String, // TODO: Stronger types
    pub placeholder: String,
}

#[derive(Serialize)]
pub(crate) struct Link {
    pub url: String, // TODO: Use Rocket's typed URI
    pub text: String,
}
