// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Preset values that can make our life easier.

use std::convert::From;
use url::Url;

/// Preset public IOTA nodes.
///
/// Each of the option here represents either a selected node or a group of nodes running the
/// corresponding Tangle. They can be easily converted into an [Url] object or a `Vec<Url>` array,
/// which is often required by this library, with an [into()] call.
///
/// ```
/// use url::Url;
/// use iota_client::PresetNodes;
///
/// let mainnet: Vec<Url> = PresetNodes::Mainnet.into();
/// let devnet: Url = PresetNodes::DevnetIotaFoundation0.into();
/// let comnet: Url = PresetNodes::Comnet.into();
///
/// assert_eq!(mainnet, vec![
///     "https://chrysalis-nodes.iota.org".try_into().unwrap(),
///     "https://chrysalis-nodes.iota.cafe".try_into().unwrap(),
/// ]);
/// assert_eq!(devnet, "https://api.lb-0.h.chrysalis-devnet.iota.cafe".try_into().unwrap());
/// assert_eq!(comnet, PresetNodes::ComnetTangleBay.into());
/// ```
///
/// [into()]: std::convert::Into::into()
#[derive(Clone, Debug)]
pub enum PresetNodes {
    /// Node(s) to the mainnet, the primary public Tangle.
    ///
    /// This is currently a synonym to [PresetNodes::MainnetIotaFoundation].
    Mainnet,
    /// Node(s) maintained by the IOTA Foundation to the mainnet.
    ///
    /// When converting into a [Url], this is currently a synonym to
    /// [PresetNodes::MainnetIotaFoundationOrg].
    MainnetIotaFoundation,
    /// `https://chrysalis-nodes.iota.org` to the mainnet, maintained by the IOTA Foundation.
    MainnetIotaFoundationOrg,
    /// `https://chrysalis-nodes.iota.cafe` to the mainnet, maintained by the IOTA Foundation.
    MainnetIotaFoundationCafe,
    /// `https://mainnet-node.tanglebay.com` to the mainnet, maintained by
    /// [Tangle Bay](https://tanglebay.com).
    MainnetTangleBay,
    /// Node(s) to the devnet, the public Tangle for development and testing purposes.
    ///
    /// This is currently a synonym to [PresetNodes::DevnetIotaFoundation].
    Devnet,
    /// Node(s) maintained by the IOTA Foundation to the devnet.
    ///
    /// When converting into a [Url], this is currently a synonym to
    /// [PresetNodes::DevnetIotaFoundation0].
    DevnetIotaFoundation,
    /// `https://api.lb-0.h.chrysalis-devnet.iota.cafe` to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation0,
    /// `https://api.lb-1.h.chrysalis-devnet.iota.cafe` to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation1,
    /// Node(s) to the comnet, the community-driven Tangle.
    ///
    /// This is currently a synonym to [PresetNodes::ComnetTangleBay].
    Comnet,
    /// Node(s) maintained by [Tangle Bay](https://tanglebay.com) to the comnet.
    ///
    /// This is currently `https://comnet-node.tanglebay.com`.
    ComnetTangleBay,
}

impl From<PresetNodes> for Url {
    fn from(node: PresetNodes) -> Url {
        match node {
            PresetNodes::Mainnet
            | PresetNodes::MainnetIotaFoundation
            | PresetNodes::MainnetIotaFoundationOrg => {
                "https://chrysalis-nodes.iota.org".try_into().unwrap()
            }
            PresetNodes::MainnetIotaFoundationCafe => {
                "https://chrysalis-nodes.iota.cafe".try_into().unwrap()
            }
            PresetNodes::MainnetTangleBay => {
                "https://mainnet-node.tanglebay.com".try_into().unwrap()
            }
            PresetNodes::Devnet
            | PresetNodes::DevnetIotaFoundation
            | PresetNodes::DevnetIotaFoundation0 => "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
                .try_into()
                .unwrap(),
            PresetNodes::DevnetIotaFoundation1 => "https://api.lb-1.h.chrysalis-devnet.iota.cafe"
                .try_into()
                .unwrap(),
            PresetNodes::Comnet | PresetNodes::ComnetTangleBay => {
                "https://comnet-node.tanglebay.com".try_into().unwrap()
            }
        }
    }
}

impl From<PresetNodes> for Vec<Url> {
    fn from(node: PresetNodes) -> Vec<Url> {
        match node {
            PresetNodes::Mainnet | PresetNodes::MainnetIotaFoundation => {
                vec![
                    PresetNodes::MainnetIotaFoundationOrg.into(),
                    PresetNodes::MainnetIotaFoundationCafe.into(),
                ]
            }
            PresetNodes::MainnetIotaFoundationOrg => {
                vec![PresetNodes::MainnetIotaFoundationOrg.into()]
            }
            PresetNodes::MainnetIotaFoundationCafe => {
                vec![PresetNodes::MainnetIotaFoundationCafe.into()]
            }
            PresetNodes::MainnetTangleBay => {
                vec![PresetNodes::MainnetTangleBay.into()]
            }
            PresetNodes::Devnet | PresetNodes::DevnetIotaFoundation => {
                vec![
                    PresetNodes::DevnetIotaFoundation0.into(),
                    PresetNodes::DevnetIotaFoundation1.into(),
                ]
            }
            PresetNodes::DevnetIotaFoundation0 => {
                vec![PresetNodes::DevnetIotaFoundation0.into()]
            }
            PresetNodes::DevnetIotaFoundation1 => {
                vec![PresetNodes::DevnetIotaFoundation1.into()]
            }
            PresetNodes::Comnet | PresetNodes::ComnetTangleBay => {
                vec![PresetNodes::ComnetTangleBay.into()]
            }
        }
    }
}
