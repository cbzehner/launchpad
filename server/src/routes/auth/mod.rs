use std::collections::HashMap;

use rocket::{request::FlashMessage, response::Redirect};
use rocket_contrib::templates::Template;

use crate::models::user::User;
use crate::models::view_context::{Form, FormRow, ViewContext};
use crate::routes::basic;

#[get("/login")]
pub fn loggedin_user(_user: User) -> Redirect {
    Redirect::to(uri!(basic::index))
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    let flash = match flash {
        Some(flash) => Some(flash.msg().to_string()),
        None => None,
    };

    let context = ViewContext {
        flash,
        form: Some(Form {
            primary_cta: "Sign In".into(),
            secondary_cta: None,
            action: "/api/auth/login".into(),
            method: "post".into(),
            rows: vec![
                FormRow {
                    label: "Username".into(),
                    r#type: "text".into(),
                    placeholder: "cbzehner".into(),
                },
                FormRow {
                    label: "Password".into(),
                    r#type: "password".into(),
                    placeholder: "******************".into(),
                },
            ],
        }),
        debug: false,
    };

    Template::render("pages/login", &context)
}

#[get("/register")]
pub fn registered_user(_user: User) -> Redirect {
    Redirect::to(uri!(basic::index))
}

#[get("/register", rank = 2)]
pub fn registration_page(flash: Option<FlashMessage>) -> Template {
    let flash = match flash {
        Some(flash) => Some(flash.msg().to_string()),
        None => None,
    };

    let context = ViewContext {
        flash,
        form: Some(Form {
            action: "/api/auth/register".into(),
            method: "post".into(),
            secondary_cta: None,
            primary_cta: "Register".into(),
            rows: vec![
                FormRow {
                    label: "Username".into(),
                    r#type: "text".into(),
                    placeholder: "cbzehner".into(),
                },
                FormRow {
                    label: "Preferred name".into(),
                    r#type: "text".into(),
                    placeholder: "Chris".into(),
                },
                FormRow {
                    label: "Email".into(),
                    r#type: "text".into(),
                    placeholder: "cbzehner@gmail.com".into(),
                },
                FormRow {
                    label: "Password".into(),
                    r#type: "password".into(),
                    placeholder: "******************".into(),
                },
            ],
        }),
        debug: false,
    };

    Template::render("pages/register", &context)
}
