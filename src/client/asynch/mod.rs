//! IOTA client with asynchronous interfaces.

mod private;
mod public;

use crate::comm::http::{AsyncHttpClient, ReqwestHttpClient};
use crate::comm::mqtt::{AsyncMqttClient, RumqttcMqttClient};
use crate::signer::AsyncSigner;
use crate::types::{Node, PresetNode};
use derive_builder::Builder;

/// Stub.
#[cfg(feature = "async")]
#[derive(Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AsyncClient {
    pub nodes: Option<Vec<Node>>,
    pub http: Option<Box<dyn AsyncHttpClient>>,
    pub mqtt: Option<Box<dyn AsyncMqttClient>>,
    pub signer: Option<Box<dyn AsyncSigner>>,
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self {
            nodes: if cfg!(feature = "comm") {
                Some(PresetNode::Mainnet.into())
            } else {
                None
            },
            http: if cfg!(feature = "reqwest") {
                Some(Box::new(ReqwestHttpClient::new()))
            } else {
                None
            },
            mqtt: if cfg!(feature = "rumqttc") {
                Some(Box::new(RumqttcMqttClient::new()))
            } else {
                None
            },
            signer: None,
        }
    }
}
