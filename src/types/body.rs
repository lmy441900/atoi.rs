// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wrapper types used in the responses from node software.
//!
//! These are mainly used for deserialization.

use serde::{Deserialize, Serialize};

/// A wrapper on a successful response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessBody<T> {
    pub data: T,
}

/// A wrapper on an erroneous response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorBody<T> {
    pub error: T,
}
