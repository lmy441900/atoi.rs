//! Implementation of [the Core REST API][core] for [AsyncClient].
//!
//! [core]: https://github.com/iotaledger/tips/pull/57

use crate::types::{Error, InfoResponse, Result};
use crate::AsyncClient;
use alloc::string::ToString;

impl AsyncClient {
    pub async fn health(&self) -> Result<()> {
        self.http_get("/health").await.map(|_| ())
    }

    pub async fn info(&self) -> Result<InfoResponse> {
        self.http_get("/api/v2/info").await.and_then(|slice| {
            serde_json::from_slice(&slice).map_err(|err| Error::ResponseError(err.to_string()))
        })
    }
}
