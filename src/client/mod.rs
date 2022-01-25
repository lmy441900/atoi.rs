// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The entry point of IOTA (a.k.a. the Tangle).
//!
//! A client connects to one or a few nodes, who foresee the Tangle they are running, and accept
//! requests performing actions on the Tangle (e.g. sending messages) or on the nodes themselves
//! (e.g. adding peers to the node). If one node failed to respond, the client automatically retries
//! on another node, until all nodes have been tried.
//!
//! There are two versions of clients:
//!
//! - [Client] with synchronous interfaces (`fn`)
//! - [AsyncClient] with asynchronous interfaces (`async fn`)
//!
//! Application developers may choose one of them depending on their preferences. Depending on the
//! switched-on features, they may exist or not. By default, both of them are enabled. If only one
//! of them is preferred in the application, one may disable the default feature set and enable only
//! the required features manually in their `Cargo.toml`:
//!
//! ```toml
//! [dependencies.iota_client]
//! version = "2.0.0"
//! default-features = false
//! features = ["sync-rest-apis"]
//! ```
//!
//! See the comments in the `Cargo.toml` of this library for more information.
//!
//! ## Use
//!
//! To create a new [Client] or [AsyncClient], use [Client::new()] or [AsyncClient::new()], which
//! automatically applies a set of defaults for out-of-box usability. A default client:
//!
//! - Connects to the [mainnet] nodes maintained by the IOTA Foundation.
//!
//! To customize options for a client (e.g. if one wants to connect to a [devnet] node), use
//! [Client::builder()] or [AsyncClient::builder()], which returns a [ClientBuilder] or an
//! [AsyncClientBuilder], allowing one to tweak individual options in an ergonomic way (i.e.
//! [the Builder pattern]).
//!
//! # Examples
//!
//! # See Also
//!
//! - [The REST API Specification](https://github.com/iotaledger/tips/blob/main/tips/TIP-0013/tip-0013.md)
//! - [The Event API Specification](https://github.com/iotaledger/tips/blob/main/tips/TIP-0016/tip-0016.md)
//!
//! [mainnet]: https://wiki.iota.org/chrysalis-docs/mainnet
//! [devnet]: https://wiki.iota.org/chrysalis-docs/devnet
//! [the Builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

#[cfg(feature = "async")]
mod asynch;
#[cfg(any(feature = "sync", feature = "async"))]
mod common;
#[cfg(feature = "sync")]
mod synch;

#[cfg(feature = "async")]
pub use self::asynch::{AsyncClient, AsyncClientBuilder};
#[cfg(feature = "sync")]
pub use self::synch::{Client, ClientBuilder};
