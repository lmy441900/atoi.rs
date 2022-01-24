// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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

    /// An error related to the computer network (e.g. a loss of connection) happened.
    #[error("network error: {0}")]
    NetworkError(Box<dyn std::error::Error>),

    /// An error was returned by the node software.
    #[error("the node responded with an error: {code} {reason}")]
    NodeError { code: u16, reason: String },

    /// No node is currently available for use.
    #[error("no node is currently available")]
    NoAvailableNode,
}
