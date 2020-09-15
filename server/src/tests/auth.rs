use rocket::http::{ContentType, Cookie, Status};
use rocket::local::Client;
use rocket::Response;

use crate::server::rocket;

fn session_cookie(response: &Response) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("session"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

fn login(client: &Client, email: &str, password: &str) -> Option<Cookie<'static>> {
    let response = client
        .post("/api/auth/login")
        .header(ContentType::Form)
        .body(format!(
            "email={email}&password={password}",
            email = email,
            password = password
        ))
        .dispatch();

    session_cookie(&response)
}

fn register(client: &Client, email: &str, name: &str, password: &str) -> Option<Cookie<'static>> {
    let response = client
        .post("/api/auth/register")
        .header(ContentType::Form)
        .body(format!(
            "email={}&preferred-name={}&password={}",
            email, name, password
        ))
        .dispatch();

    session_cookie(&response)
}

#[test]
fn redirect_on_index() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

// TODO: Consider whether this is testing anything that's not covered
//       by other tests, such as login_logout_succeeds
// #[test]
// fn can_login() {
//     let client = Client::new(rocket()).unwrap();

//     let mut response = client.get("/login").dispatch();
//     let body = response.body_string().unwrap();
//     assert_eq!(response.status(), Status::Ok);
//     assert!(body.contains("Please login to continue."));
// }

#[test]
fn login_fails() {
    let client = Client::new(rocket()).unwrap();
    assert!(login(&client, "unregistered_user", "password").is_none());
    assert!(login(&client, "sergio", "123456").is_none());
}

#[test]
fn registration_succeeds() {
    let client = Client::new(rocket()).unwrap();
    let registration_cookie = register(
        &client,
        "kjohnson",
        "kjohnson@test.com",
        "Katherine Johnson",
        "password",
    )
    .expect("registered new user");

    // Ensure we're logged in.
    let mut response = client
        .get("/")
        .cookie(registration_cookie.clone())
        .dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Logged in with user ID"))
}

#[test]
fn duplicate_registration_security() {
    let client = Client::new(rocket()).unwrap();
    let registration_cookie = register(
        &client,
        "kjohnson",
        "kjohnson@test.com",
        "Katherine Johnson",
        "password",
    )
    .expect("registered new user");

    // Ensure we're logged in.
    let mut response = client
        .get("/")
        .cookie(registration_cookie.clone())
        .dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Logged in with user ID"));

    let malicious_email_change_attempt = register(
        &client,
        "kjohnson",
        "mallory@imposter.test.com",
        "Katherine Johnson",
        "password",
    );
    assert!(malicious_email_change_attempt.is_none());

    let malicious_password_change_attempt = register(
        &client,
        "kjohnson",
        "kjohnson@test.com",
        "Katherine Johnson",
        "youhavebeenpwned",
    );
    assert!(malicious_password_change_attempt.is_none());
}

#[test]
fn logout_succeeds() {
    let client = Client::new(rocket()).unwrap();
    let registration_cookie = register(
        &client,
        "kjohnson",
        "kjohnson@test.com",
        "Katherine Johnson",
        "password",
    )
    .expect("registered new user");

    let response = client
        .post("/api/auth/logout")
        .cookie(registration_cookie)
        .dispatch();
    let cookie = session_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());
}

#[test]
fn login_logout_succeeds() {
    let client = Client::new(rocket()).unwrap();

    // register a new user and then log them out
    let registration_cookie = register(
        &client,
        "kjohnson",
        "kjohnson@test.com",
        "Katherine Johnson",
        "password",
    )
    .expect("registered new user");
    client
        .post("/api/auth/logout")
        .cookie(registration_cookie)
        .dispatch();

    // Now login as the user.
    let login_cookie = login(&client, "kjohnson", "password").expect("logged in");

    // Ensure we're logged in.
    let mut response = client.get("/").cookie(login_cookie.clone()).dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Logged in with user ID"));

    // One more.
    let response = client.get("/login").cookie(login_cookie.clone()).dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));

    // Logout.
    let response = client
        .post("/api/auth/logout")
        .cookie(login_cookie)
        .dispatch();
    let cookie = session_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());
}
