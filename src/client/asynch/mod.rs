//! IOTA client with asynchronous interfaces.

mod private;
mod public;

use crate::comm::http::AsyncHttpClient;
use crate::comm::mqtt::AsyncMqttClient;
use crate::signer::AsyncSigner;
use crate::types::Node;

/// The entry point of IOTA, with asynchronous interfaces.
#[derive(Default)]
pub struct AsyncClient {
    pub nodes: Option<Vec<Node>>,
    pub http: Option<Box<dyn AsyncHttpClient>>,
    pub mqtt: Option<Box<dyn AsyncMqttClient>>,
    pub signer: Option<Box<dyn AsyncSigner>>,
}

impl AsyncClient {
    pub fn new() -> Self {
        Default::default()
    }
}
