//! Communication backends speaking MQTT.

#[cfg(feature = "rumqttc")]
mod rumqttc;

use async_trait::async_trait;

// Exports.
#[cfg(feature = "rumqttc")]
pub use self::rumqttc::RumqttcMqttClient;

/// Interfaces for a synchronous MQTT client.
pub trait MqttClient {
    fn subscribe(&self, topic: &str);
    fn unsubscribe(&self, topic: &str);
}

/// Interfaces for an asynchronous MQTT client.
#[async_trait]
pub trait AsyncMqttClient {
    async fn subscribe(&self, topic: &str);
    async fn unsubscribe(&self, topic: &str);
}
