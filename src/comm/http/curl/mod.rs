//! HTTP communication backend using libcurl.

mod handler;

use self::handler::CurlHttpClientHandler;
use crate::comm::http::HttpClient;
use crate::types::{Auth, Result};
use curl::easy::Easy2;

/// The default user agent string.
const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " curl/",
    env!("CURL_VERSION"),
);

/// HTTP communication backend using libcurl.
#[derive(Debug, Default)]
pub struct CurlHttpClient {}

impl HttpClient for CurlHttpClient {
    fn get(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<u8>> {
        let mut handle = Easy2::new(CurlHttpClientHandler::new());
        let mut headers = curl::easy::List::new();

        handle.reset();

        handle.get(true)?;
        handle.url(url)?;
        handle.useragent(USER_AGENT)?;

        match auth {
            Some(Auth::Basic { username, password }) => {
                handle.http_auth(curl::easy::Auth::new().basic(true))?;
                handle.username(username)?;
                handle.password(password)?;
            }
            Some(Auth::Jwt(jwt)) => {
                headers.append(&format!("Authorization: Bearer {}", jwt))?;
            }
            None => {}
        }

        handle.http_headers(headers)?;
        handle.verbose(true)?;

        handle.perform()?;

        let handler = handle.get_ref();
        Ok(handler.0.clone())
    }

    fn post(&self, _url: &str, _auth: Option<&Auth>, _body: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    fn delete(&self, _url: &str, _auth: Option<&Auth>) -> Result<Vec<u8>> {
        todo!()
    }
}

impl CurlHttpClient {
    pub fn new() -> Self {
        Default::default()
    }
}
