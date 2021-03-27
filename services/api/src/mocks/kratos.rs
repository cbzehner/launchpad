use std::sync::RwLock;

use lazy_static::lazy_static;
use rocket::http::Status;

use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

pub struct KratosService(Option<MockServer>);

impl KratosService {
    pub fn new() -> Self {
        Self(None)
    }

    #[cfg(test)]
    pub async fn init(&mut self, status: Status, body: &str, expect: wiremock::Times) {
        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(status.code).set_body_string(body))
            .expect(expect)
            .mount(&mock_server)
            .await;

        self.0 = Some(mock_server);
    }
}

impl std::ops::Deref for KratosService {
    type Target = Option<MockServer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

lazy_static! {
    pub static ref MOCK_KRATOS_SERVER: RwLock<KratosService> = RwLock::new(KratosService::new());
}
