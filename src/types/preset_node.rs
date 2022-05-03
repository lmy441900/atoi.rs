//! Preset public IOTA nodes.

use super::node::Node;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use core::convert::From;

/// Preset public IOTA nodes.
///
/// Each of the option here represents either a selected node or a group of nodes running the
/// corresponding Tangle. They can be easily converted into several types with an invocation of
/// [`into()`]:
///
/// - [`&str`] and [`Vec<&str>`]
/// - [`String`] and [`Vec<String>`]
///
/// ```
/// use atoi::types::PresetNode;
///
/// let mainnet: Vec<&str> = PresetNode::Mainnet.into();
/// let devnet: String = PresetNode::DevnetIotaFoundation0.into();
///
/// assert_eq!(
///     mainnet,
///     vec![
///         "https://chrysalis-nodes.iota.org",
///         "https://chrysalis-nodes.iota.cafe",
///     ]
/// );
/// assert_eq!(
///     devnet,
///     String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
/// );
/// ```
///
/// [`into()`]: core::convert::Into::into()
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PresetNode {
    /// Node(s) to the mainnet, the primary public Tangle.
    ///
    /// This is currently a synonym to [`MainnetIotaFoundation`].
    ///
    /// [`MainnetIotaFoundation`]: Self::MainnetIotaFoundation
    Mainnet,
    /// Node(s) maintained by the IOTA Foundation to the mainnet.
    MainnetIotaFoundation,
    /// <https://chrysalis-nodes.iota.org> to the mainnet, maintained by the IOTA Foundation.
    MainnetIotaFoundationOrg,
    /// <https://chrysalis-nodes.iota.cafe> to the mainnet, maintained by the IOTA Foundation.
    MainnetIotaFoundationCafe,
    /// <https://mainnet-node.tanglebay.com> to the mainnet, maintained by
    /// [Tangle Bay](https://tanglebay.com).
    MainnetTangleBay,
    /// Node(s) to the devnet, the public Tangle for development and testing purposes.
    ///
    /// This is currently a synonym to [`DevnetIotaFoundation`].
    ///
    /// [`DevnetIotaFoundation`]: Self::DevnetIotaFoundation
    Devnet,
    /// Node(s) maintained by the IOTA Foundation to the devnet.
    DevnetIotaFoundation,
    /// <https://api.lb-0.h.chrysalis-devnet.iota.cafe> to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation0,
    /// <https://api.lb-1.h.chrysalis-devnet.iota.cafe> to the devnet, maintained by the IOTA
    /// Foundation.
    DevnetIotaFoundation1,
}

impl From<PresetNode> for &str {
    fn from(node: PresetNode) -> &'static str {
        match node {
            PresetNode::Mainnet
            | PresetNode::MainnetIotaFoundation
            | PresetNode::MainnetIotaFoundationOrg => "https://chrysalis-nodes.iota.org",
            PresetNode::MainnetIotaFoundationCafe => "https://chrysalis-nodes.iota.cafe",
            PresetNode::MainnetTangleBay => "https://mainnet-node.tanglebay.com",
            PresetNode::Devnet
            | PresetNode::DevnetIotaFoundation
            | PresetNode::DevnetIotaFoundation0 => "https://api.lb-0.h.chrysalis-devnet.iota.cafe",
            PresetNode::DevnetIotaFoundation1 => "https://api.lb-1.h.chrysalis-devnet.iota.cafe",
        }
    }
}

impl From<PresetNode> for Vec<&str> {
    fn from(node: PresetNode) -> Vec<&'static str> {
        match node {
            PresetNode::Mainnet | PresetNode::MainnetIotaFoundation => vec![
                PresetNode::MainnetIotaFoundationOrg.into(),
                PresetNode::MainnetIotaFoundationCafe.into(),
            ],
            PresetNode::MainnetIotaFoundationOrg => {
                vec![PresetNode::MainnetIotaFoundationOrg.into()]
            }
            PresetNode::MainnetIotaFoundationCafe => {
                vec![PresetNode::MainnetIotaFoundationCafe.into()]
            }
            PresetNode::MainnetTangleBay => vec![PresetNode::MainnetTangleBay.into()],
            PresetNode::Devnet | PresetNode::DevnetIotaFoundation => vec![
                PresetNode::DevnetIotaFoundation0.into(),
                PresetNode::DevnetIotaFoundation1.into(),
            ],
            PresetNode::DevnetIotaFoundation0 => vec![PresetNode::DevnetIotaFoundation0.into()],
            PresetNode::DevnetIotaFoundation1 => vec![PresetNode::DevnetIotaFoundation1.into()],
        }
    }
}

impl From<PresetNode> for String {
    fn from(node: PresetNode) -> String {
        let str: &str = node.into();
        str.into()
    }
}

impl From<PresetNode> for Vec<String> {
    fn from(node: PresetNode) -> Vec<String> {
        let strs: Vec<&str> = node.into();
        strs.into_iter().map(String::from).collect()
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
        let urls: Vec<String> = node.into();
        urls.into_iter()
            .map(|url| Node { url, auth: None })
            .collect()
    }
}
