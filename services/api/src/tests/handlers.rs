use crate::rocket;
use rocket::http::Status;

#[test]
fn route_index() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn route_health_check() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/healthz").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("{\"status\":\"ok\"}".into()));
}
