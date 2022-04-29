//! High-level HTTP methods sending request to selected nodes.

use crate::comm::http::AsyncHttpClient;
use crate::types::{Error, Node};
use crate::{AsyncClient, Result};

#[allow(dead_code)]
impl AsyncClient {
    fn get_http_and_nodes(&self) -> Result<(&dyn AsyncHttpClient, &[Node])> {
        match (&self.http, &self.nodes) {
            (Some(http), Some(nodes)) => Ok((http.as_ref(), nodes.as_ref())),
            (Some(_), None) => Err(Error::MissingNode),
            (None, Some(_)) | (None, None) => Err(Error::MissingHttpClient),
        }
    }

    pub(crate) async fn http_get_from_all_nodes(
        &self,
        path: &str,
    ) -> Result<Box<[Result<Box<[u8]>>]>> {
        let (http, nodes) = self.get_http_and_nodes()?;
        let mut results = Vec::new();

        for node in nodes {
            let url = node.url.clone() + path;
            let result = http.get(&url, node.auth.as_ref()).await;
            results.push(result);
        }

        Ok(results.into_boxed_slice())
    }

    pub(crate) async fn http_get_from_some_nodes(
        &self,
        _path: &str,
    ) -> Result<Box<[Result<Box<[u8]>>]>> {
        todo!()
    }

    pub(crate) async fn http_get_from_any_node(&self, _path: &str) -> Result<Box<[u8]>> {
        todo!()
    }

    pub(crate) async fn http_post_to_any_node(
        &self,
        _path: &str,
        _body: &[u8],
    ) -> Result<Box<[u8]>> {
        todo!()
    }

    pub(crate) async fn http_delete_from_any_node(&self, _path: &str) -> Result<Box<[u8]>> {
        todo!()
    }
}
