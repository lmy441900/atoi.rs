//! The error type used across the library.

use alloc::string::String;

/// The error type used across the library.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Error {
    /// No node has been configured during client construction.
    MissingNode,

    /// No HTTP client has been configured during client construction.
    MissingHttpClient,

    /// An error was returned by the node software.
    NodeError {
        url: String,
        code: u16,
        reason: String,
    },

    /// An invalid / malformed response was received from a node.
    ResponseError(String),

    /// An error was returned by the HTTP client.
    HttpClientError(String),
}

impl From<serde_json::Error> for Error {
    fn from(serde_json_error: serde_json::Error) -> Self {
        Self::ResponseError(serde_json_error.to_string())
    }
}

#[cfg(feature = "curl")]
impl From<curl::Error> for Error {
    fn from(curl_error: curl::Error) -> Self {
        Self::HttpClientError(curl_error.to_string())
    }
}
