//! Types returned by the Core REST API.

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub index: u64,
    pub timestamp: u64,
    pub milestone_id: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub is_healthy: bool,
    pub latest_milestone: Milestone,
    pub confirmed_milestone: Milestone,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub messages_per_second: f32,
    pub referenced_messages_per_second: f32,
    pub referenced_rate: f32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RentStructure {
    pub v_byte_cost: u64,
    pub v_byte_factor_data: u64,
    pub v_byte_factor_key: u64,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub network_name: String,
    #[serde(rename = "bech32HRP")]
    pub bech32_hrp: String,
    pub token_supply: String,
    pub protocol_version: u64,
    #[serde(rename = "minPoWScore")]
    pub min_pow_score: f32,
    pub rent_structure: RentStructure,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseToken {
    pub name: String,
    pub ticker_symbol: String,
    pub unit: String,
    pub decimals: u64,
    pub subunit: Option<String>,
    pub use_metric_prefix: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub status: Status,
    pub metrics: Metrics,
    pub protocol: Protocol,
    pub base_token: BaseToken,
    pub features: Vec<String>,
    pub plugins: Vec<String>,
}
