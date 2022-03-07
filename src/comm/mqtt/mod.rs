//! Communication backends talking MQTT.

#[cfg(feature = "rumqttc")]
mod rumqttc;

use async_trait::async_trait;

// Exports.
#[cfg(feature = "rumqttc")]
pub use self::rumqttc::RumqttcMqttClient;

/// Interfaces for a synchronous MQTT client.
pub trait SyncMqttClient {}

/// Interfaces for an asynchronous MQTT client.
#[async_trait]
pub trait AsyncMqttClient {}
