//! Communication backends speaking MQTT.

use async_trait::async_trait;

/// Interfaces for a synchronous MQTT client.
pub trait MqttClient {
    fn subscribe(&mut self, topic: &str);
    fn unsubscribe(&mut self, topic: &str);
}

/// Interfaces for an asynchronous MQTT client.
#[async_trait]
pub trait AsyncMqttClient {
    async fn subscribe(&mut self, topic: &str);
    async fn unsubscribe(&mut self, topic: &str);
}
