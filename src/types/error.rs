//! The error type used across the library.

/// The error type used across the library.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// No node has been configured during client construction.
    #[cfg_attr(
        feature = "std",
        error("cannot perform any network operation: no node has been configured")
    )]
    MissingNode,

    /// No HTTP client has been configured during client construction.
    #[cfg_attr(
        feature = "std",
        error("cannot send any http request: no http client has been configured")
    )]
    MissingHttpClient,

    /// No MQTT client has been configured during client construction.
    #[cfg_attr(
        feature = "std",
        error("cannot subscribe to any topic: no mqtt client has been configured")
    )]
    MissingMqttClient,

    /// No signer has been configured during client construction.
    #[cfg_attr(
        feature = "std",
        error("cannot sign on any message: no signer has been configured")
    )]
    MissingSigner,

    /// An error was returned by the node software.
    #[cfg_attr(
        feature = "std",
        error("node {url} responded with an error: {code} {reason}")
    )]
    NodeError {
        url: String,
        code: u16,
        reason: String,
    },

    /// An invalid / malformed response was received from a node.
    #[cfg_attr(feature = "std", error("invalid response: {0}"))]
    ResponseError(String),

    /// An computer network error (e.g. a loss of connection) happened.
    #[cfg_attr(feature = "std", error("network error: {0}"))]
    NetworkError(String),
}
