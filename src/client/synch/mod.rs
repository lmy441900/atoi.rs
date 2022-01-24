// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementaion of [Client].
//!
//! The name of this module (_synch_) is chosen to align with the [asynch] module.
//!
//! [asynch]: super::asynch

mod private;
mod public;

use crate::PresetNodes;
use derive_builder::Builder;
use url::Url;

/// The entry point of IOTA.
///
/// See the [module-level documentation][mod-doc] for more information.
///
/// [mod-doc]: super
#[derive(Clone, Debug, Builder)]
#[builder(default)]
pub struct Client {
    /// URLs of IOTA nodes to connect to.
    pub nodes: Vec<Url>,

    /// The [ureq::Agent] used to initiate HTTP requests.
    #[cfg(feature = "rest-apis")]
    pub agent: ureq::Agent,
}

/// Constructors.
impl Client {
    /// Create a new [Client] with default options.
    ///
    /// # Example
    ///
    /// ```
    /// let client = iota_client::Client::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new [ClientBuilder] to tweak options of a [Client].
    ///
    /// See [PresetNodes] for a list of preset public nodes that are most often used.
    ///
    /// # Example
    ///
    /// ```
    /// use iota_client::{Client, PresetNodes};
    ///
    /// let client = Client::builder()
    ///     .nodes(PresetNodes::Devnet.into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            nodes: PresetNodes::Mainnet.into(),
            agent: ureq::AgentBuilder::new()
                .user_agent(Self::DEFAULT_USER_AGENT)
                .build(),
        }
    }
}
