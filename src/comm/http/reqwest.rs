//! [Reqwest]-supported HTTP communication backend.
//!
//! [Reqwest]: https://github.com/seanmonstar/reqwest

use super::AsyncHttpClient;
use crate::Result;
use async_trait::async_trait;
use url::Url;

/// Stub.
#[cfg(feature = "reqwest")]
pub struct ReqwestHttpClient {}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self {}
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
