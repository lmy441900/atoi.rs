// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The official client library for interacting with IOTA, a.k.a. the Tangle.
//!
//! A client library is for applications to integrate themselves into the Tangle. _iota.rs_
//! (iota-client in Rust) is a fully-featured client library developed and supported by the IOTA
//! Foundation. _iota.rs_ enables an application to talk with the Tangle, via node software (e.g.
//! [Hornet], [Bee]) over [the REST APIs] and [the Event APIs], in a more native and ergonomic way.
//!
//! This API documentation is mainly for application developers. For basic concepts of IOTA and the
//! Tangle, please visit [the IOTA Wiki] for more background information.
//!
//! To start, use [Client] or [AsyncClient].
//!
//! ## Features
//!
//! There are a few tweakable Cargo features:
//!
//! - `sync-rest-apis`: enables synchronous REST APIs
//! - `async-rest-apis`: enables asynchronous REST APIs
//! - `sync-event-apis`: enables synchronous Event APIs
//! - `async-event-apis`: enables asynchronous Event APIs
//!
//! By default, all of the above are enabled. If only a subset of the APIs are needed, one can
//! remove other features by first disabling the default feature set and then enabling the desired
//! feature. See the [client] module-level documentaion or `Cargo.toml` for more information.
//!
//! [Hornet]: https://github.com/gohornet/hornet
//! [Bee]: https://github.com/iotaledger/bee/
//! [the REST APIs]: https://github.com/iotaledger/tips/blob/main/tips/TIP-0013/tip-0013.md
//! [the Event APIs]: https://github.com/iotaledger/tips/blob/main/tips/TIP-0016/tip-0016.md
//! [the IOTA Wiki]: https://wiki.iota.org/
//! [Mainnet]: https://wiki.iota.org/chrysalis-docs/mainnet

// #![warn(missing_docs)]

// `pub mod client` is mainly for exposing the module-level documentation.
pub mod client;
mod error;
mod presets;
mod types;

// Re-export types that are parts of the public APIs.
pub use bee_message::address::{Address, Ed25519Address};
pub use bee_message::{Message, MessageBuilder, MessageId};

// Public API exports.
#[cfg(feature = "async")]
pub use self::client::{AsyncClient, AsyncClientBuilder};
#[cfg(feature = "sync")]
pub use self::client::{Client, ClientBuilder};
pub use self::error::Error;
pub use self::presets::PresetNodes;
pub use self::types::*;

/// The canonical [Result] type used across the library, with [Error] as the error type.
///
/// [Result]: core::result::Result
/// [Error]: enum@self::Error
pub type Result<T> = std::result::Result<T, self::error::Error>;

/// The version string of iota-client.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
