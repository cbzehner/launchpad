use rocket::http::Status;
use rocket::local::asynchronous::Client;

use api::server;

#[tokio::test]
async fn index() {
    let client = Client::tracked(server(None, None)).await.unwrap();
    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await, Some("Hello, world!".into()));
}

#[tokio::test]
async fn health_check() {
    let client = Client::tracked(server(None, None)).await.unwrap();
    let response = client.get("/healthz").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("{\"status\":\"ok\"}".into())
    );
}
