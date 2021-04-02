use std::env;
use url::Url;

pub(crate) struct PostgresProvider(Url);

impl PostgresProvider {
    pub(crate) fn new(url: impl Into<Option<Url>>) -> Self {
        match url.into() {
            Some(url) => Self(url),
            None => match env::var("POSTGRES_URL") {
                Ok(url) => Self(Url::parse(&url).expect("Invalid URL for POSTGRES_URL")),
                Err(_) => Self::default(),
            },
        }
    }
}

impl Default for PostgresProvider {
    fn default() -> Self {
        #[cfg(not(test))]
        return Self(Url::parse("postgres://api:secret@localhost:5432/api").unwrap());
        #[cfg(test)]
        return Self(Url::parse("postgres://api:secret@localhost:5432/api_test").unwrap());
    }
}

impl std::ops::Deref for PostgresProvider {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
