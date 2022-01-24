// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementaion of [AsyncClient].
//!
//! The name of this module (_asynch_) is chosen to avoid the reserved keyword `async`. It could be
//! (by referring to as `r#async`), but cbindgen complains about this name in code, so we just move
//! away from it.

mod private;
mod public;

use crate::PresetNodes;
use derive_builder::Builder;
use url::Url;

/// The entry point of IOTA, with asynchronous interfaces.
///
/// See the [module-level documentation][mod-doc] for more information.
///
/// [mod-doc]: super
#[derive(Clone, Debug, Builder)]
#[builder(default)]
pub struct AsyncClient {
    /// URLs of IOTA nodes to connect to.
    pub nodes: Vec<Url>,
}

impl AsyncClient {
    /// Create a new [AsyncClient] with default options.
    ///
    /// # Example
    ///
    /// ```
    /// let client = iota_client::AsyncClient::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new [AsyncClientBuilder] to tweak options of a [AsyncClient].
    ///
    /// See [PresetNodes] for a list of preset public nodes that are most often used.
    ///
    /// # Example
    ///
    /// ```
    /// use iota_client::{AsyncClient, PresetNodes};
    ///
    /// let client = AsyncClient::builder()
    ///     .nodes(PresetNodes::Devnet.into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> AsyncClientBuilder {
        AsyncClientBuilder::default()
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self {
            nodes: PresetNodes::Mainnet.into(),
        }
    }
}
