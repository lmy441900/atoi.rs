//! Implementation of [the Core REST API][core] for [Client].
//!
//! [core]: https://github.com/iotaledger/tips/pull/57

use crate::types::{InfoResponse, Result};
use crate::Client;

impl Client {
    pub fn health(&self) -> Result<()> {
        self.http_get("/health")?;
        Ok(())
    }

    pub fn info(&self) -> Result<InfoResponse> {
        Ok(serde_json::from_slice(&self.http_get("/api/v2/info")?)?)
    }
}
