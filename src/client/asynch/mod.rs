//! IOTA client with asynchronous interfaces.

mod private;
mod public;

use crate::comm::http::{AsyncHttpClient, ReqwestHttpClient};
use crate::comm::mqtt::{AsyncMqttClient, RumqttcMqttClient};
use crate::signer::AsyncSigner;
use crate::types::PresetNodes;
use derive_builder::Builder;
use url::Url;

/// Stub.
#[cfg(feature = "async")]
#[derive(Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AsyncClient {
    pub nodes: Vec<Url>,
    pub http: Option<Box<dyn AsyncHttpClient>>,
    pub mqtt: Option<Box<dyn AsyncMqttClient>>,
    pub signer: Option<Box<dyn AsyncSigner>>,
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self {
            nodes: PresetNodes::Mainnet.into(),
            http: Some(Box::new(ReqwestHttpClient::new())),
            mqtt: Some(Box::new(RumqttcMqttClient::new())),
            signer: None,
        }
    }
}
