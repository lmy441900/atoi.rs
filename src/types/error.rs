//! The error type used across the library.

/// The error type used across the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An invalid / malformed input was given by a user.
    #[error("invalid input: {0}")]
    InvalidInput(Box<dyn std::error::Error>),

    /// An invalid / malformed response was received from a node.
    #[error("invalid response received: {0}")]
    InvalidResponse(Box<dyn std::error::Error>),

    /// An computer network error (e.g. a loss of connection) happened.
    #[error("network error: {0}")]
    NetworkError(Box<dyn std::error::Error>),

    /// An error was returned by the node software.
    #[error("the node responded with an error: {code} {reason}")]
    NodeError { code: u16, reason: String },

    /// No node is currently available for use. An array of errors encountered is included.
    #[error("no node is currently available")]
    NoAvailableNode(Vec<Self>),

    /// No node has been configured during client construction.
    #[error("cannot perform any network operation: no node has been configured")]
    NoNode,

    /// No HTTP client has been configured during client construction.
    #[error("cannot send any http request: no http client has been configured")]
    NoHttpClient,

    /// No MQTT client has been configured during client construction.
    #[error("cannot subscribe to any topic: no mqtt client has been configured")]
    NoMqttClient,

    /// No signer has been configured during client construction.
    #[error("cannot sign on any message: no signer has been configured")]
    NoSigner,
}
