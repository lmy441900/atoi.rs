//! The entry point of IOTA (a.k.a. the Tangle).
//!
//! A client connects to one or a few nodes, who foresee the Tangle they are running, and accept
//! requests performing actions on the Tangle (e.g. sending messages) or on the nodes themselves
//! (e.g. adding peers to the node). If one node failed to respond and the settings allow, then the
//! client automatically retries on another node, until getting a successful response or having
//! tried on all nodes and failed.
//!
//! # Examples

#[cfg(feature = "async")]
mod asynch;

// Exports.
#[cfg(feature = "async")]
pub use self::asynch::{AsyncClient, AsyncClientBuilder};
