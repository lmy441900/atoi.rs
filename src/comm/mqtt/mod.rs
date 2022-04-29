//! Communication backends speaking MQTT.

use async_trait::async_trait;

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
