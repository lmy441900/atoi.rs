//! Communication backends for [clients] to connect to the node software.
//!
//! These are pluggable and reusable modules wrapping specific libraries that perform network
//! requests.
//!
//! [clients]: crate::client

pub mod http;
