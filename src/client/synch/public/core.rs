//! Implementation of [the Core REST API][core] for [Client].
//!
//! [core]: https://github.com/iotaledger/tips/pull/57

use crate::types::{Error, InfoResponse, Result};
use crate::Client;
use alloc::string::ToString;

impl Client {
    pub fn health(&self) -> Result<()> {
        self.http_get("/health").map(|_| ())
    }

    pub fn info(&self) -> Result<InfoResponse> {
        self.http_get("/api/v2/info").and_then(|slice| {
            serde_json::from_slice(&slice).map_err(|err| Error::ResponseError(err.to_string()))
        })
    }
}
