use std::sync::RwLock;

use url::Url;

#[allow(dead_code)]
mod session;

pub(crate) use session::Session;

lazy_static! {
    // TODO: Avoid hard-coding the auth server public URL!
    static ref BASE_URL: RwLock<Url> = RwLock::new(Url::parse("http://127.0.0.1:4433").unwrap());
}

fn get_base_url() -> Url {
    let base_url = BASE_URL.read().unwrap();
    base_url.clone()
}

// Exposed to allow tests to change the base_url for the Kratos service.
pub fn set_base_url(url: Url) {
    let mut base_url = BASE_URL.write().unwrap();
    *base_url = url;
}
