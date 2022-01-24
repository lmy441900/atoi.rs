// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Common utilities shared between [Client] and [AsyncClient].
//!
//! [Client]: crate::Client
//! [AsyncClient]: crate::AsyncClient

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// The maximum size of a response body allowed: 1 MiB.
pub const MAX_CONTENT_LENGTH: usize = 1024 * 1024;

/// Serialize
pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    serde_json::to_vec(value).map_err(|err| Error::InvalidInput(Box::new(err)))
}

/// Deserialize
pub fn deserialize<'a, T>(slice: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    serde_json::from_slice::<T>(slice).map_err(|err| Error::InvalidResponse(Box::new(err)))
}
