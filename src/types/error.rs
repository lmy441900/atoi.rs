//! The error type used across the library.

use alloc::string::String;

/// The error type used across the library.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Error {
    /// No node has been configured during client construction.
    MissingNode,

    /// No HTTP client has been configured during client construction.
    MissingHttpClient,

    /// No MQTT client has been configured during client construction.
    MissingMqttClient,

    /// No signer has been configured during client construction.
    MissingSigner,

    /// AN error was returned by the client or its backend.
    ClientError(String),

    /// An error was returned by the node software.
    NodeError {
        url: String,
        code: u16,
        reason: String,
    },

    /// An invalid / malformed response was received from a node.
    ResponseError(String),

    /// An computer network error (e.g. a loss of connection) happened.
    NetworkError(String),
}

#[cfg(feature = "curl")]
impl From<curl::Error> for Error {
    fn from(curl_error: curl::Error) -> Self {
        Self::ClientError(curl_error.to_string())
    }
}
