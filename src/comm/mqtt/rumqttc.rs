//! [Rumqttc]-supported MQTT communication backend.
//!
//! [Rumqttc]: https://github.com/bytebeamio/rumqtt

use async_trait::async_trait;

use super::AsyncMqttClient;

/// Stub.
#[cfg(feature = "rumqttc")]
pub struct RumqttcMqttClient {}

impl Default for RumqttcMqttClient {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl AsyncMqttClient for RumqttcMqttClient {}

impl RumqttcMqttClient {
    pub fn new() -> Self {
        Default::default()
    }
}
