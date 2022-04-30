//! High-level HTTP methods sending request to selected nodes.

use crate::comm::http::HttpClient;
use crate::types::{Error, Node, Result};
use crate::Client;
use alloc::vec::Vec;

impl Client {
    fn http_and_node(&self) -> Result<(&dyn HttpClient, &Node)> {
        match (&self.http, &self.node) {
            (Some(http), Some(node)) => Ok((http.as_ref(), node)),
            (Some(_), None) => Err(Error::MissingNode),
            (None, Some(_)) | (None, None) => Err(Error::MissingHttpClient),
        }
    }

    pub(crate) fn http_get(&self, path: &str) -> Result<Vec<u8>> {
        let (http, node) = self.http_and_node()?;
        let url = node.url.clone() + path;

        http.get(&url, node.auth.as_ref())
    }
}
