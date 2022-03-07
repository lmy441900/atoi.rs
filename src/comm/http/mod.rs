//! Communication backends talking HTTP.

#[cfg(feature = "reqwest")]
mod reqwest;

// Exports.
#[cfg(feature = "reqwest")]
pub use self::reqwest::ReqwestHttpClient;

use crate::Result;
use async_trait::async_trait;
use url::Url;

/// Interfaces of a synchronous HTTP client.
pub trait SyncHttpClient {
    fn get(&self, url: &Url) -> Result<Vec<u8>>;
    fn put(&self, url: &Url, body: &[u8]) -> Result<Vec<u8>>;
    fn post(&self, url: &Url, body: &[u8]) -> Result<Vec<u8>>;
    fn delete(&self, url: &Url) -> Result<Vec<u8>>;
}

/// Interfaces of an asynchronous HTTP client.
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(&self, url: &Url) -> Result<Vec<u8>>;
    async fn put(&self, url: &Url, body: &[u8]) -> Result<Vec<u8>>;
    async fn post(&self, url: &Url, body: &[u8]) -> Result<Vec<u8>>;
    async fn delete(&self, url: &Url) -> Result<Vec<u8>>;
}
