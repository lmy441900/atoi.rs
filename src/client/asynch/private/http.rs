//! High-level HTTP methods sending request to selected nodes.

use crate::comm::http::AsyncHttpClient;
use crate::types::{Error, Node, Result};
use crate::AsyncClient;
use alloc::vec::Vec;

#[allow(dead_code)]
impl AsyncClient {
    fn http_and_node(&self) -> Result<(&dyn AsyncHttpClient, &Node)> {
        match (&self.http, &self.node) {
            (Some(http), Some(node)) => Ok((http.as_ref(), node)),
            (Some(_), None) => Err(Error::MissingNode),
            (None, Some(_)) | (None, None) => Err(Error::MissingHttpClient),
        }
    }

    pub(crate) async fn http_get(&self, path: &str) -> Result<Vec<u8>> {
        let (http, node) = self.http_and_node()?;
        let url = node.url.clone() + path;

        http.get(&url, node.auth.as_ref()).await
    }
}
