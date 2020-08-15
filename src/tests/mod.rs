use super::rocket;
use rocket::http::{ContentType, Cookie, Status};
use rocket::local::Client;
use rocket::Response;

fn user_id_cookie(response: &Response) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

fn login(client: &Client, user: &str, pass: &str) -> Option<Cookie<'static>> {
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!(
            "username={username}&password={password}",
            username = user,
            password = pass
        ))
        .dispatch();

    user_id_cookie(&response)
}

fn register(
    client: &Client,
    user: &str,
    email: &str,
    name: &str,
    pass: &str,
) -> Option<Cookie<'static>> {
    let response = client
        .post("/register")
        .header(ContentType::Form)
        .body(format!(
            "username={username}&email={email}&preferred-name={preferred_name}&password={password}",
            username = user,
            email = email,
            preferred_name = name,
            password = pass
        ))
        .dispatch();

    user_id_cookie(&response)
}

#[test]
fn redirect_on_index() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn can_login() {
    let client = Client::new(rocket()).unwrap();

    let mut response = client.get("/login").dispatch();
    let body = response.body_string().unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(body.contains("Please login to continue."));
}

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
        .post("/logout")
        .cookie(registration_cookie)
        .dispatch();
    let cookie = user_id_cookie(&response).expect("logout cookie");
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
        .post("/logout")
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
    let response = client.post("/logout").cookie(login_cookie).dispatch();
    let cookie = user_id_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());
}
