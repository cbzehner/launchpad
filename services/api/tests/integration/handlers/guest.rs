use rocket::http::Status;
use rocket::local::blocking::Client;

use api::server;

#[test]
fn index() {
    let client = Client::tracked(server()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn health_check() {
    let client = Client::tracked(server()).unwrap();
    let response = client.get("/healthz").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("{\"status\":\"ok\"}".into()));
}
