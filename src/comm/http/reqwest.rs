//! [Reqwest]-supported HTTP communication backend.
//!
//! [Reqwest]: https://github.com/seanmonstar/reqwest

use super::AsyncHttpClient;
use crate::Result;
use async_trait::async_trait;
use url::Url;

/// The default user agent string.
const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " reqwest/",
    env!("REQWEST_VERSION")
);

/// HTTP client with [reqwest].
#[cfg(feature = "reqwest")]
pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

#[async_trait]
impl AsyncHttpClient for ReqwestHttpClient {
    async fn get(&self, _url: &Url) -> Result<Vec<u8>> {
        todo!()
    }

    async fn put(&self, _url: &Url, _body: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    async fn post(&self, _url: &Url, _body: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    async fn delete(&self, _url: &Url) -> Result<Vec<u8>> {
        todo!()
    }
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Default::default()
    }
}
