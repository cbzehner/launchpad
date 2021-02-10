use rocket::http::{ContentType, Cookie, Status};
use rocket::local::Client;
use rocket::Response;
use rocket_contrib::databases::diesel::PgConnection;
// use diesel::pg::PgConnectionPgConnection;

use crate::server;
use crate::db;

/// Extract a session cookie from a response.
fn session_cookie<'a>(response: &'a Response) -> Option<Cookie<'a>> {
    response
        .cookies()
        .into_iter()
        .find(|cookie| cookie.name() == "session")
}

fn cleanup_registration() {
    let conn = PgConnection::establish
    // Open a diesel connection
    // DELETE FROM users WHERE email='kjohnson@test.com'
    // Close connection
}

#[test]
fn redirect_on_index() {
    let client = Client::new(server::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn registration_succeeds() {
    let client = Client::new(server::rocket()).unwrap();
    client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&preferred-name={preferred_name}&password={password}",
            email = "kjohnson@test.com",
            preferred_name = "Katherine Johnson",
            password = "test123"
        ))
        .dispatch();

    // Ensure we're logged in.
    let mut response = client
        .get("/")
        .dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    println!("{:?}", response);
    assert!(body.contains("Logged in with user ID"))
}

#[ignore]
#[test]
fn duplicate_registration_security() {
    let client = Client::new(server::rocket()).unwrap();
    client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&preferred-name={preferred_name}&password={password}",
            email = "kjohnson@test.com",
            preferred_name = "Katherine Johnson",
            password = "test123"
        ))
        .dispatch();

    // Ensure we're logged in.
    let mut response = client.get("/").dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Logged in with user ID"));

    let malicious_password_change_attempt = client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&preferred-name={preferred_name}&password={password}",
            email = "kjohnson@test.com",
            preferred_name = "Katherine Johnson",
            password = "youhavebeenpwned"
        ))
        .dispatch();
    let malicious_registration_cookie = session_cookie(&malicious_password_change_attempt);
    assert!(malicious_registration_cookie.is_none());
}

#[ignore]
#[test]
fn logout_succeeds() {
    let client = Client::new(server::rocket()).unwrap();
    client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&preferred-name={preferred_name}&password={password}",
            email = "kjohnson@test.com",
            preferred_name = "Katherine Johnson",
            password = "test123"
        ))
        .dispatch();

    let response = client.post("/api/auth/logout").dispatch();
    let cookie = session_cookie(&response).expect("Logout cookie");
    assert!(cookie.value().is_empty());
}

#[ignore]
#[test]
fn login_logout_succeeds() {
    let client = Client::new(server::rocket()).unwrap();

    // register a new user and then log them out
    client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&preferred-name={preferred_name}&password={password}",
            email = "kjohnson@test.com",
            preferred_name = "Katherine Johnson",
            password = "test123"
        ))
        .dispatch();
    client
        .post("/api/auth/logout")
        .dispatch();

    // Now login as the user.
    client
        .post("/api/auth/login")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&password={password}",
            email = "kjohnson@test.com",
            password = "test123"
        ))
        .dispatch();

    // Ensure we're logged in.
    let mut response = client.get("/").dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Logged in with user ID"));

    // One more.
    let response = client.get("/login").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));

    // Logout.
    let response = client
        .post("/api/auth/logout")
        .dispatch();
    let cookie = session_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());
}
