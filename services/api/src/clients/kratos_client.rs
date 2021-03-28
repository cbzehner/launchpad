use url::Url;

pub(crate) struct KratosClient(Url);

impl KratosClient {
    pub(crate) fn new(url: Url) -> Self {
        Self(url)
    }
}

impl std::ops::Deref for KratosClient {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
