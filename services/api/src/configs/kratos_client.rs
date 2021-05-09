use std::env;
use url::Url;

pub(crate) struct KratosClient(Url);

impl KratosClient {
    pub(crate) fn new(url: impl Into<Option<Url>>) -> Self {
        match url.into() {
            Some(url) => Self(url),
            None => match env::var("KRATOS_URL") {
                Ok(url) => Self(Url::parse(&url).expect("Invalid URL for KRATOS_URL")),
                Err(_) => Self::default(),
            },
        }
    }
}

impl Default for KratosClient {
    fn default() -> Self {
        Self(Url::parse("http://localhost:4433").unwrap())
    }
}

impl std::ops::Deref for KratosClient {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
