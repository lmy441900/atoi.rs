//! Implementation of [the Core REST API][core] for [AsyncClient].
//!
//! [core]: https://github.com/iotaledger/tips/pull/57

use crate::types::{InfoResponse, Result};
use crate::AsyncClient;

impl AsyncClient {
    pub async fn health(&self) -> Result<()> {
        self.http_get("/health").await?;
        Ok(())
    }

    pub async fn info(&self) -> Result<InfoResponse> {
        Ok(serde_json::from_slice(
            &self.http_get("/api/v2/info").await?,
        )?)
    }
}
