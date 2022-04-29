//! Communication backends speaking HTTP.

use crate::types::Auth;
use crate::Result;
use async_trait::async_trait;

/// Interfaces of a synchronous HTTP client.
pub trait HttpClient {
    fn get(&self, url: &str, auth: Option<&Auth>) -> Result<Box<[u8]>>;
    fn post(&self, url: &str, auth: Option<&Auth>, body: &[u8]) -> Result<Box<[u8]>>;
    fn delete(&self, url: &str, auth: Option<&Auth>) -> Result<Box<[u8]>>;
}

/// Interfaces of an asynchronous HTTP client.
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(&self, url: &str, auth: Option<&Auth>) -> Result<Box<[u8]>>;
    async fn post(&self, url: &str, auth: Option<&Auth>, body: &[u8]) -> Result<Box<[u8]>>;
    async fn delete(&self, url: &str, auth: Option<&Auth>) -> Result<Box<[u8]>>;
}
