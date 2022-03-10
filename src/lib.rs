//! An experimental client library for interacting with IOTA, a.k.a. the Tangle.
//!
//! A client library is for applications to integrate themselves into the Tangle. _atoi.rs_
//! (iota-client in Rust) is a fully-featured client library. _atoi.rs_ enables an application to
//! talk with the Tangle, via node software (e.g. [Hornet], [Bee]) over [the Core REST APIs] and
//! [the Event APIs], in a more native and ergonomic way. Plugin APIs (e.g.
//! [the UTXO Indexer REST API]) are also supported.
//!
//! This API documentation is mainly for application developers. For basic concepts of IOTA and the
//! Tangle, please visit [the IOTA Wiki] for more background information.
//!
#![cfg_attr(feature = "client", doc = "To start, use ")]
#![cfg_attr(feature = "sync", doc = "[Client]")]
#![cfg_attr(all(feature = "sync", feature = "async"), doc = " or ")]
#![cfg_attr(feature = "async", doc = "[AsyncClient]")]
#![cfg_attr(feature = "client", doc = ".")]
//!
//! ## Features
//!
//! To do.
//!
//! [Hornet]: https://github.com/gohornet/hornet
//! [Bee]: https://github.com/iotaledger/bee/
//! [the Core REST APIs]: #
//! [the Event APIs]: https://github.com/iotaledger/tips/blob/main/tips/TIP-0016/tip-0016.md
//! [the UTXO Indexer REST API]: #
//! [the IOTA Wiki]: https://wiki.iota.org/
//! [Mainnet]: https://wiki.iota.org/chrysalis-docs/mainnet

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "comm")]
pub mod comm;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "signer")]
pub mod signer;
pub mod types;

// Re-exports.
#[cfg(feature = "async")]
pub use self::client::{AsyncClient, AsyncClientBuilder};
#[cfg(feature = "sync")]
pub use self::client::{Client, ClientBuilder};

/// The version string of iota-client.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The canonical [Result] type used across the library, with [Error] as the error type.
///
/// [Result]: std::result::Result
/// [Error]: self::types::Error
pub type Result<T> = std::result::Result<T, self::types::Error>;
