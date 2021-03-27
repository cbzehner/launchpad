use bear::fixture;
use rocket::http::{Cookie, Status};
use rocket::local::asynchronous::Client;
use url::Url;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

use api::models::kratos;
use api::server;

#[tokio::test]
async fn whoami() {
    let _mock_server = mock_kratos(Status::Ok, &fixture("kratos/session.json"), 1.into()).await;

    let client = Client::tracked(server()).await.unwrap();
    let response = client
        .get("/whoami")
        .cookie(Cookie::new("ory_kratos_session", "MTYxNjM1ODg2NXxEdi1CQkFFQ180SUFBUkFCRUFBQVJfLUNBQUVHYzNSeWFXNW5EQThBRFhObGMzTnBiMjVmZEc5clpXNEdjM1J5YVc1bkRDSUFJRkE0T1VwaFUzVm5UMlpUYmt4TU1rSktPVkExYkVkWmQyRkRjVmhQWjJKbXyKNqk1ax4XXUA9kcc67NA_KYYFw5CNOczCpIaegt7fTQ=="))
        .dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("{\"id\":\"53354449-9e03-40fc-bc90-fc499e6d44e3\",\"email\":\"cbzehner@test.com\",\"full_name\":\"Christopher Zehner\",\"preferred_name\":\"Chris\"}".into())
    );
}

#[tokio::test]
async fn whoami_missing_cookie() {
    let _mock_server = mock_kratos(Status::Ok, &fixture("kratos/session.json"), 0.into()).await;

    let client = Client::tracked(server()).await.unwrap();
    let response = client.get("/whoami").dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
    assert_ne!(
        response.into_string().await,
        Some("{\"id\":\"53354449-9e03-40fc-bc90-fc499e6d44e3\",\"email\":\"cbzehner@test.com\",\"full_name\":\"Christopher Zehner\",\"preferred_name\":\"Chris\"}".into())
    );

    drop(_mock_server);
}

#[tokio::test]
async fn whoami_session_inactive() {
    let _mock_server = mock_kratos(
        Status::Ok,
        &fixture("kratos/session_inactive.json"),
        1.into(),
    )
    .await;

    let client = Client::tracked(server()).await.unwrap();
    let response = client
        .get("/whoami")
        .cookie(Cookie::new("ory_kratos_session", "MTYxNjM1ODg2NXxEdi1CQkFFQ180SUFBUkFCRUFBQVJfLUNBQUVHYzNSeWFXNW5EQThBRFhObGMzTnBiMjVmZEc5clpXNEdjM1J5YVc1bkRDSUFJRkE0T1VwaFUzVm5UMlpUYmt4TU1rSktPVkExYkVkWmQyRkRjVmhQWjJKbXyKNqk1ax4XXUA9kcc67NA_KYYFw5CNOczCpIaegt7fTQ=="))
        .dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
    assert_ne!(
        response.into_string().await,
        Some("{\"id\":\"53354449-9e03-40fc-bc90-fc499e6d44e3\",\"email\":\"cbzehner@test.com\",\"full_name\":\"Christopher Zehner\",\"preferred_name\":\"Chris\"}".into())
    );

    drop(_mock_server);
}

#[tokio::test]
async fn whoami_session_expired() {
    let _mock_server = mock_kratos(
        Status::Ok,
        &fixture("kratos/session_expired.json"),
        1.into(),
    )
    .await;

    let client = Client::tracked(server()).await.unwrap();
    let response = client
        .get("/whoami")
        .cookie(Cookie::new("ory_kratos_session", "MTYxNjM1ODg2NXxEdi1CQkFFQ180SUFBUkFCRUFBQVJfLUNBQUVHYzNSeWFXNW5EQThBRFhObGMzTnBiMjVmZEc5clpXNEdjM1J5YVc1bkRDSUFJRkE0T1VwaFUzVm5UMlpUYmt4TU1rSktPVkExYkVkWmQyRkRjVmhQWjJKbXyKNqk1ax4XXUA9kcc67NA_KYYFw5CNOczCpIaegt7fTQ=="))
        .dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
    assert_ne!(
        response.into_string().await,
        Some("{\"id\":\"53354449-9e03-40fc-bc90-fc499e6d44e3\",\"email\":\"cbzehner@test.com\",\"full_name\":\"Christopher Zehner\",\"preferred_name\":\"Chris\"}".into())
    );

    drop(_mock_server);
}

/// Mock the ORY Kratos service.
async fn mock_kratos(status: Status, body: &str, expect: wiremock::Times) -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(status.code).set_body_string(body))
        .expect(expect)
        .mount(&mock_server)
        .await;
    kratos::set_base_url(Url::parse(&mock_server.uri()).unwrap());

    mock_server
}
