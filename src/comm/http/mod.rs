//! Communication backends speaking HTTP.

use crate::types::Node;
use crate::Result;
use async_trait::async_trait;

/// Interfaces of a synchronous HTTP client.
pub trait HttpClient {
    fn get(&self, node: &Node, path: &str) -> Result<Box<[u8]>>;
    fn post(&self, node: &Node, path: &str, body: &[u8]) -> Result<Box<[u8]>>;
    fn delete(&self, node: &Node, path: &str) -> Result<Box<[u8]>>;
}

/// Interfaces of an asynchronous HTTP client.
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(&self, node: &Node, path: &str) -> Result<Box<[u8]>>;
    async fn post(&self, node: &Node, path: &str, body: &[u8]) -> Result<Box<[u8]>>;
    async fn delete(&self, node: &Node, path: &str) -> Result<Box<[u8]>>;
}
