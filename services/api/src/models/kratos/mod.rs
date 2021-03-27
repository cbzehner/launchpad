use url::Url;

#[allow(dead_code)]
mod session;

pub(crate) use session::Session;

#[cfg(test)]
use crate::mocks;

fn base_url() -> Result<Url, url::ParseError> {
    // TODO: Avoid hard-coding the auth server public URL!
    let url = Url::parse("http://127.0.0.1:4433");

    #[cfg(not(test))]
    return url;
    #[cfg(test)]
    return match mocks::kratos::MOCK_KRATOS_SERVER.read().unwrap().as_ref() {
        Some(mock_server) => Url::parse(&mock_server.uri()),
        None => url,
    };
}
