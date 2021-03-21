use crate::rocket;
use rocket::http::Status;

#[test]
fn test_hello() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}
