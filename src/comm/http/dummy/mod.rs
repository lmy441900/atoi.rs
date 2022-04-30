//! Dummy HTTP client that returns fake data when invoked.

mod response;

use self::response::respond_dummy;
use super::{AsyncHttpClient, HttpClient};
use crate::types::{Auth, Result};
use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct DummyHttpClient {}

impl HttpClient for DummyHttpClient {
    fn get(&self, url: &str, _auth: Option<&Auth>) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }

    fn post(&self, url: &str, _auth: Option<&Auth>, _body: &[u8]) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }

    fn delete(&self, url: &str, _auth: Option<&Auth>) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }
}

#[async_trait]
impl AsyncHttpClient for DummyHttpClient {
    async fn get(&self, url: &str, _auth: Option<&Auth>) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }

    async fn post(&self, url: &str, _auth: Option<&Auth>, _body: &[u8]) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }

    async fn delete(&self, url: &str, _auth: Option<&Auth>) -> Result<Vec<u8>> {
        Ok(respond_dummy(url))
    }
}

impl DummyHttpClient {
    pub fn new() -> Self {
        Self::default()
    }
}
