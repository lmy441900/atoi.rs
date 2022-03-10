//! [rumqttc]-supported MQTT communication backend.

use std::cell::RefCell;

use super::AsyncMqttClient;
use crate::types::PresetNode;
use async_trait::async_trait;
use derive_builder::Builder;
use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use tokio::sync::Mutex;

/// The default MQTT client ID.
const DEFAULT_ID: &str = "iota/rumqttc";

/// MQTT client with [rumqttc].
#[cfg(feature = "rumqttc")]
#[derive(Builder)]
#[builder(pattern = "owned", default)]
pub struct RumqttcMqttClient {
    pub client: Mutex<RefCell<AsyncClient>>,
    pub event_loop: Mutex<RefCell<EventLoop>>,
}

impl Default for RumqttcMqttClient {
    fn default() -> Self {
        let options = MqttOptions::new(DEFAULT_ID, PresetNode::Mainnet, 443);
        let cap = 10;
        let (client, event_loop) = AsyncClient::new(options, cap);

        Self {
            client: Mutex::new(RefCell::new(client)),
            event_loop: Mutex::new(RefCell::new(event_loop)),
        }
    }
}

#[async_trait]
impl AsyncMqttClient for RumqttcMqttClient {
    async fn subscribe(&self, _topic: &str) {
        todo!()
    }

    async fn unsubscribe(&self, _topic: &str) {
        todo!()
    }
}

impl RumqttcMqttClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn builder() -> RumqttcMqttClientBuilder {
        RumqttcMqttClientBuilder::default()
    }
}
