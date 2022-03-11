//! Communication backends speaking HTTP.

#[cfg(feature = "reqwest")]
mod reqwest;

// Exports.
#[cfg(feature = "reqwest")]
pub use self::reqwest::ReqwestHttpClient;

use crate::types::Node;
use crate::Result;
use async_trait::async_trait;

/// Interfaces of a synchronous HTTP client.
pub trait HttpClient {
    fn get(&self, node: &Node) -> Result<Vec<u8>>;
    fn post(&self, node: &Node, body: &[u8]) -> Result<Vec<u8>>;
    fn delete(&self, node: &Node) -> Result<Vec<u8>>;
}

/// Interfaces of an asynchronous HTTP client.
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(&self, node: &Node) -> Result<Vec<u8>>;
    async fn post(&self, node: &Node, body: &[u8]) -> Result<Vec<u8>>;
    async fn delete(&self, node: &Node) -> Result<Vec<u8>>;
}
