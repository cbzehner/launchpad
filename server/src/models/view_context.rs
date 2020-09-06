use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ViewContext {
    pub flash: Option<String>,
    pub form: Option<Form>,
    pub debug: bool, // TODO: Calculate this based on Rocket env. Maybe add a Header to all requests in development?
}

#[derive(Serialize)]
pub(crate) struct Form {
    pub rows: Vec<FormRow>,
    pub primary_cta: String,
    pub secondary_cta: Option<String>,
    pub action: String,
    pub method: String, // TODO: Stronger types
}

#[derive(Serialize)]
pub(crate) struct FormRow {
    pub label: String,
    pub r#type: String, // TODO: Stronger types
    pub placeholder: String,
}
