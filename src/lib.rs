//! An experimental client library for interacting with IOTA, a.k.a. the Tangle.
//!
//! A client library is for applications to integrate themselves into the Tangle. atoi.rs
//! (iota-client in Rust) is a fully-featured client library. atoi.rs enables an application to
//! talk with the Tangle via node software (e.g. [Hornet], [Bee]).
//!
//! This API documentation is mainly for application developers. For basic concepts of IOTA and the
//! Tangle, please visit the [IOTA Wiki] for more background information.
//!
//! To start, use `Client` or `AsyncClient`.
//!
//! ## Supported API Specifications
//!
//! - [TIP-25 Core REST API](https://github.com/iotaledger/tips/pull/57)
//! - [TIP-26 UTXO Indexer REST API](https://github.com/iotaledger/tips/pull/62)
//! - [TIP-16 Event MQTT API](https://github.com/iotaledger/tips/blob/main/tips/TIP-0016/tip-0016.md)
//! - [Hornet Plugin - Faucet REST APT](#)
//!
//! ## Features
//!
//! Various modules can be switched on or off via Cargo. Feature gates are defined in `Cargo.toml`,
//! but some of them are for internal use. Here is a brief explanation to the usable ones:
//!
//! - `sync`: turns on `Client` with synchronous interfaces.
//! - `async`: turns on `AsyncClient` with asynchronous interfaces.
//! - `tls-webpki`: bundle the Web PKI CA certificates and use it for TLS connections.
//! - `tls-native`: use the CA certificates available on the running system.
//!
//! [Hornet]: https://github.com/gohornet/hornet
//! [Bee]: https://github.com/iotaledger/bee/
//! [IOTA Wiki]: https://wiki.iota.org/

#![no_std]

extern crate alloc;

pub mod client;
pub mod comm;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod types;

// Re-exports.
#[cfg(feature = "async")]
pub use self::client::AsyncClient;
#[cfg(feature = "sync")]
pub use self::client::Client;
