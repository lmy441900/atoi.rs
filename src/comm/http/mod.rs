//! Communication backends speaking HTTP.

#[cfg(feature = "curl")]
mod curl;
mod dummy;

#[cfg(feature = "curl")]
pub use self::curl::CurlHttpClient;
pub use self::dummy::DummyHttpClient;

use crate::types::{Auth, Result};
use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;

/// Interfaces of a synchronous HTTP client.
pub trait HttpClient {
    fn get(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<u8>>;
    fn post(&self, url: &str, auth: Option<&Auth>, body: &[u8]) -> Result<Vec<u8>>;
    fn delete(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<u8>>;
}

/// Interfaces of an asynchronous HTTP client.
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<u8>>;
    async fn post(&self, url: &str, auth: Option<&Auth>, body: &[u8]) -> Result<Vec<u8>>;
    async fn delete(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<u8>>;
}
