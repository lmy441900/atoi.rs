// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types from the responses of node software.
//!
//! Compared to the [bee_message] and [bee_rest_api] crates, types here are _checked_ by using
//! concrete types putting constraints on the shape of data it holds. For example,
//! [InfoResponse::version] has type [Version] instead of [String]. This ensures that the data
//! received from a node is always valid and not malformed.
//!
//! These types don't include their successful / erroneous wrappers; the wrappers are defined in
//! module [body].
//!
//! These types are not necessarily parts of the public APIs of [Client] and [AsyncClient]; they
//! are mainly for correctly deserializing responses.
//!
//! # See Also
//!
//! - [The REST API Specification](https://github.com/iotaledger/tips/blob/main/tips/TIP-0013/tip-0013.md)
//! - [The Event API Specification](https://github.com/iotaledger/tips/blob/main/tips/TIP-0016/tip-0016.md)
//!
//! [Client]: crate::Client
//! [AsyncClient]: crate::AsyncClient
//! [body]: super::body

use super::{AddressType, ConflictReason, LedgerInclusionState};
use bee_message::prelude::{
    Address, MessageId, MilestoneId, Output, OutputId, TailTransactionHash, TransactionId,
    TreasuryInput, TreasuryOutput,
};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub name: String,
    pub version: Version,
    pub is_healthy: bool,
    pub network_id: String,
    #[serde(rename = "bech32HRP")]
    pub bech32_hrp: String,
    #[serde(rename = "minPoWScore")]
    pub min_pow_score: f64,
    pub messages_per_second: f64,
    pub referenced_messages_per_second: f64,
    pub referenced_rate: f64,
    pub latest_milestone_timestamp: u64,
    pub latest_milestone_index: u64,
    pub confirmed_milestone_index: u64,
    pub pruning_index: u64,
    pub features: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipsResponse {
    pub tip_message_ids: Vec<MessageId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitMessageResponse {
    pub message_id: MessageId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesFindResponse {
    pub index: Vec<u8>,
    pub max_results: u64,
    pub count: u64,
    pub message_ids: Vec<MessageId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetadataResponse {
    pub message_id: MessageId,
    pub parent_message_ids: Vec<MessageId>,
    pub is_solid: bool,
    pub referenced_by_milestone_index: Option<u64>,
    pub milestone_index: Option<u64>,
    pub ledger_inclusion_state: LedgerInclusionState,
    pub conflict_reason: ConflictReason,
    pub should_promote: bool,
    pub should_reattach: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageChildrenResponse {
    pub message_id: MessageId,
    pub max_results: u64,
    pub count: u64,
    pub children_message_ids: Vec<MessageId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputResponse {
    pub message_id: MessageId,
    pub transaction_id: TransactionId,
    pub output_index: u64,
    pub is_spent: bool,
    pub output: Output,
    pub ledger_index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAddressResponse {
    pub address_type: AddressType,
    pub adderss: Address,
    pub balance: u64,
    pub dust_allowed: bool,
    pub ledger_index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputsAddressResponse {
    pub address_type: AddressType,
    pub adderss: Address,
    pub max_results: u64,
    pub count: u64,
    pub output_ids: Vec<OutputId>,
    pub ledger_index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigratedFundsEntry {
    pub tail_transaction_hash: TailTransactionHash,
    pub address: Address,
    pub deposit: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryTransactionPayload {
    pub input: TreasuryInput,
    pub output: TreasuryOutput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptPayload {
    pub migrated_at: u64,
    pub r#final: bool,
    pub funds: Vec<MigratedFundsEntry>,
    pub transaction: TreasuryTransactionPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptTuple {
    pub receipt: ReceiptPayload,
    pub milestone_index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptsResponse {
    pub receipts: Vec<ReceiptTuple>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryResponse {
    pub milesone_id: MilestoneId,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneResponse {
    pub index: u64,
    pub message_id: MessageId,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoChangesResponse {
    pub index: u64,
    pub created_outputs: Vec<MessageId>,
    pub consumed_outputs: Vec<MessageId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}
