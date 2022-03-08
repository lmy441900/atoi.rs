//! Preset public IOTA nodes.

use super::node::Node;
use std::convert::From;
use url::Url;

/// Preset public IOTA nodes.
///
/// Each of the option here represents either a selected node or a group of nodes running the
/// corresponding Tangle. They can be easily converted into an [Url] object, a `Vec<Url>` array,
/// a [Node] object, or a `Vec<Node>` array (which is often required by this library), with just an
/// [into()] call.
///
/// ```
/// use url::Url;
/// use iota_client::types::{Node, PresetNode};
///
/// let mainnet: Vec<Url> = PresetNode::Mainnet.into();
/// let devnet: Url = PresetNode::DevnetIotaFoundation0.into();
/// let comnet: Node = PresetNode::Comnet.into();
///
/// assert_eq!(mainnet, vec![
///     "https://chrysalis-nodes.iota.org".try_into().unwrap(),
///     "https://chrysalis-nodes.iota.cafe".try_into().unwrap(),
/// ]);
/// assert_eq!(devnet, "https://api.lb-0.h.chrysalis-devnet.iota.cafe".try_into().unwrap());
/// assert_eq!(comnet, PresetNode::ComnetTangleBay.into());
/// ```
///
/// [into()]: std::convert::Into::into()
#[derive(Clone, Debug)]
pub enum PresetNode {
    /// Node(s) to the mainnet, the primary public Tangle.
    ///
    /// This is currently a synonym to [PresetNode::MainnetIotaFoundation].
    Mainnet,
    /// Node(s) maintained by the IOTA Foundation to the mainnet.
    ///
    /// When converting into a [Url], this is currently a synonym to
    /// [PresetNode::MainnetIotaFoundationOrg].
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
    /// This is currently a synonym to [PresetNode::DevnetIotaFoundation].
    Devnet,
    /// Node(s) maintained by the IOTA Foundation to the devnet.
    ///
    /// When converting into a [Url], this is currently a synonym to
    /// [PresetNode::DevnetIotaFoundation0].
    DevnetIotaFoundation,
    /// `https://api.lb-0.h.chrysalis-devnet.iota.cafe` to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation0,
    /// `https://api.lb-1.h.chrysalis-devnet.iota.cafe` to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation1,
    /// Node(s) to the comnet, the community-driven Tangle.
    ///
    /// This is currently a synonym to [PresetNode::ComnetTangleBay].
    Comnet,
    /// Node(s) maintained by [Tangle Bay](https://tanglebay.com) to the comnet.
    ///
    /// This is currently `https://comnet-node.tanglebay.com`.
    ComnetTangleBay,
}

impl From<PresetNode> for Url {
    fn from(node: PresetNode) -> Url {
        match node {
            PresetNode::Mainnet
            | PresetNode::MainnetIotaFoundation
            | PresetNode::MainnetIotaFoundationOrg => {
                "https://chrysalis-nodes.iota.org".try_into().unwrap()
            }
            PresetNode::MainnetIotaFoundationCafe => {
                "https://chrysalis-nodes.iota.cafe".try_into().unwrap()
            }
            PresetNode::MainnetTangleBay => {
                "https://mainnet-node.tanglebay.com".try_into().unwrap()
            }
            PresetNode::Devnet
            | PresetNode::DevnetIotaFoundation
            | PresetNode::DevnetIotaFoundation0 => "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
                .try_into()
                .unwrap(),
            PresetNode::DevnetIotaFoundation1 => "https://api.lb-1.h.chrysalis-devnet.iota.cafe"
                .try_into()
                .unwrap(),
            PresetNode::Comnet | PresetNode::ComnetTangleBay => {
                "https://comnet-node.tanglebay.com".try_into().unwrap()
            }
        }
    }
}

impl From<PresetNode> for Vec<Url> {
    fn from(node: PresetNode) -> Vec<Url> {
        match node {
            PresetNode::Mainnet | PresetNode::MainnetIotaFoundation => {
                vec![
                    PresetNode::MainnetIotaFoundationOrg.into(),
                    PresetNode::MainnetIotaFoundationCafe.into(),
                ]
            }
            PresetNode::MainnetIotaFoundationOrg => {
                vec![PresetNode::MainnetIotaFoundationOrg.into()]
            }
            PresetNode::MainnetIotaFoundationCafe => {
                vec![PresetNode::MainnetIotaFoundationCafe.into()]
            }
            PresetNode::MainnetTangleBay => {
                vec![PresetNode::MainnetTangleBay.into()]
            }
            PresetNode::Devnet | PresetNode::DevnetIotaFoundation => {
                vec![
                    PresetNode::DevnetIotaFoundation0.into(),
                    PresetNode::DevnetIotaFoundation1.into(),
                ]
            }
            PresetNode::DevnetIotaFoundation0 => {
                vec![PresetNode::DevnetIotaFoundation0.into()]
            }
            PresetNode::DevnetIotaFoundation1 => {
                vec![PresetNode::DevnetIotaFoundation1.into()]
            }
            PresetNode::Comnet | PresetNode::ComnetTangleBay => {
                vec![PresetNode::ComnetTangleBay.into()]
            }
        }
    }
}

impl From<PresetNode> for Node {
    fn from(node: PresetNode) -> Node {
        Self {
            url: node.into(),
            auth: None,
        }
    }
}

impl From<PresetNode> for Vec<Node> {
    fn from(node: PresetNode) -> Vec<Node> {
        let urls: Vec<Url> = node.into();

        urls.into_iter()
            .map(|url| Node { url, auth: None })
            .collect()
    }
}
