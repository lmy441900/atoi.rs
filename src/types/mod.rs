// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! User-facing types used across the public APIs.

mod body;
mod response;

pub use body::*;
pub use response::*;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LedgerInclusionState {
    Included,
    Conflicting,
    NoTransaction,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum ConflictReason {
    AlreadySpent = 1,
    AlreadySpentWhileConfirmingThisMilestone = 2,
    NotFound = 3,
    SumsDoesNotMatch = 4,
    InvalidUnblockBlock = 5,
    UnsupportedOutputType = 6,
    UnsupportedAddressType = 7,
    InvalidDustAllowance = 8,
    SemanticValidationFailed = 9,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum OutputType {
    SigLockedSingleOutput = 0,
    SigLockedDustAllowanceOutput = 1,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum AddressType {
    Ed25519 = 0,
}
