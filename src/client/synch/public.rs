// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Public API implementations of [Client].

use super::super::common::{deserialize, serialize};
use super::Client;
use crate::types::*;
use crate::{Error, Result};
use bee_message::prelude::{Address, Message, MessageId, OutputId, TransactionId};

/// Public APIs (except constructors).
impl Client {
    /// Get health statuses of the nodes. (`/health`)
    ///
    /// A node considers itself healthy if its solid milestone is at most two delta away from the
    /// latest known milestone, has at least one ongoing gossip stream and the latest known
    /// milestone is newer than 5 minutes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let node_healths = client.get_nodes_health();
    ///
    /// client.nodes
    ///     .iter()
    ///     .zip(node_healths.iter())
    ///     .for_each(|(url, result)| match result {
    ///         Ok(health) => {
    ///             println!("{}: {}", url, if *health { "healthy" } else { "unhealthy" });
    ///         }
    ///         Err(err) => {
    ///             println!("{}: {}", url, err);
    ///         }
    ///     });
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_nodes_health(&self) -> Vec<Result<bool>> {
        self.nodes
            .iter()
            .map(|node| {
                let url = match node.join("/health") {
                    Ok(url) => url,
                    Err(err) => return Err(Error::InvalidInput(Box::new(err))),
                };

                let resp = match self.agent.request_url("GET", &url).call() {
                    Ok(resp) => resp,
                    Err(err) => return Err(Error::NetworkError(Box::new(err))),
                };

                let status = resp.status();
                let status_text = resp.status_text();

                match status {
                    200 => Ok(true),
                    503 => Ok(false),
                    _ => Err(Error::NodeError {
                        code: status,
                        reason: status_text.to_string(),
                    }),
                }
            })
            .collect()
    }

    /// Get general information about the nodes. (`/api/v1/info`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let node_info = client.get_nodes_information();
    ///
    /// client.nodes
    ///     .iter()
    ///     .zip(node_info.iter())
    ///     .for_each(|(url, result)| match result {
    ///         Ok(info) => {
    ///             println!("{}: {:?}", url, info);
    ///         }
    ///         Err(err) => {
    ///             println!("{}: {}", url, err);
    ///         }
    ///     });
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_nodes_information(&self) -> Vec<Result<InfoResponse>> {
        self.http_get_from_all_nodes("/api/v1/info")
            .into_iter()
            .map(|result| {
                result
                    .and_then(|body| deserialize::<SuccessBody<InfoResponse>>(&body))
                    .map(|succ| succ.data)
            })
            .collect()
    }

    /// Get tips that are ideal for attaching a message. (`/api/v1/tips`)
    ///
    /// The tips can be considered as _non-lazy_ and are therefore ideal for attaching a message.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let tips = client.get_tips().unwrap();
    /// tips.iter().for_each(|tip| println!("{}", tip));
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_tips(&self) -> Result<Vec<MessageId>> {
        self.http_get_from_any_node("/api/v1/tips")
            .and_then(|resp| deserialize::<SuccessBody<TipsResponse>>(&resp))
            .map(|body| body.data.tip_message_ids)
    }

    /// Submit a message. (`/api/v1/messages`)
    ///
    /// The node takes care of missing fields and tries to auto-fill the following fields:
    ///
    /// - `network_id`
    /// - `parents`
    /// - `nonce`
    ///
    /// If `payload` is `None`, the message will be built without a payload. On success, the message
    /// will be stored in the Tangle.
    ///
    /// # Example
    #[cfg(feature = "rest-apis")]
    pub fn send_message(&self, msg: &Message) -> Result<MessageId> {
        self.http_post_to_any_node("/api/v1/messages", &serialize(msg)?)
            .and_then(|resp| deserialize::<SuccessBody<SubmitMessageResponse>>(&resp))
            .map(|body| body.data.message_id)
    }

    /// Search for messages matching a given indexation key. (`/api/v1/messages`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.find_messages("example".as_bytes()).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn find_messages(&self, idx: &[u8]) -> Result<MessagesFindResponse> {
        self.http_get_from_any_node(&format!("/api/v1/messages?index={}", &hex::encode(idx)))
            .and_then(|resp| deserialize::<SuccessBody<MessagesFindResponse>>(&resp))
            .map(|body| body.data)
    }

    /// Find a message by its identifer, getting a structural response.
    /// (`/api/v1/messages/{messageId}`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_message(
    ///     &"f532a53545103276b46876c473846d98648ee418468bce76df4868648dd73e5d".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_message(&self, _id: &MessageId) -> Result<Message> {
        todo!()
    }

    /// Find the metadata of a given message. (`/api/v1/messages/{messageId}/metadata`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_message_metadata(
    ///     &"f532a53545103276b46876c473846d98648ee418468bce76df4868648dd73e5d".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_message_metadata(&self, _id: &MessageId) -> Result<MessageMetadataResponse> {
        todo!()
    }

    /// Find a message by its identifer, getting a raw response.
    /// (`/api/v1/messages/{messageId}/raw`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_message_raw(
    ///     &"f532a53545103276b46876c473846d98648ee418468bce76df4868648dd73e5d".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_message_raw(&self, _id: &MessageId) -> Result<Vec<u8>> {
        todo!()
    }

    /// Find a message by its identifier, getting children of the message.
    /// (`/api/v1/messages/{messageId}/children`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_message_children(
    ///     &"f532a53545103276b46876c473846d98648ee418468bce76df4868648dd73e5d".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_message_children(&self, _id: &MessageId) -> Result<MessageChildrenResponse> {
        todo!()
    }

    /// Find an output by its identifier. (`/api/v1/outputs/{outputId}`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_output(
    ///     &"1ee46e19f4219ee65afc10227d0ca22753f76ef32d1e922e5cbe3fbc9b5a5298".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_output(&self, _id: &OutputId) -> Result<OutputResponse> {
        todo!()
    }

    /// Get the balance of an address.
    ///
    /// This is actually `/api/v1/addresses/ed25519/{address}`, as one must first convert to an
    /// [Address] regardless of the type of address, during which the address has been validated.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::{Client, Address, Ed25519Address};
    ///
    /// let client = Client::new();
    ///
    /// // Convert from a Bech32 address
    /// let _addr_bech32: Address =
    ///     "iota1qrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqgyzyx"
    ///     .parse()
    ///     .unwrap();
    ///
    /// // Convert from a hex-encoded Ed25519 address
    /// let _addr_ed25519: Address =
    ///     "efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a3"
    ///     .parse::<Ed25519Address>()
    ///     .unwrap()
    ///     .into();
    ///
    /// // Both of the above are identical and can be used in this method
    /// // let response = client.get_balance(&_addr_bech32).unwrap();
    /// let response = client.get_balance(&_addr_ed25519).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_balance(&self, _addr: &Address) -> Result<BalanceAddressResponse> {
        todo!()
    }

    /// Get all outputs that use a given address.
    ///
    /// If [OutputsAddressResponse::count] equals [OutputsAddressResponse::max_results], then there
    /// might be more outputs available but those were skipped for performance reasons. User should
    /// sweep the address to reduce the amount of outputs.
    ///
    /// This is actually `/api/v1/addresses/ed25519/{address}/outputs`, as one must first convert to
    /// an [Address] regardless of the type of address, during which the address has been validated.
    ///
    /// # Example
    #[cfg(feature = "rest-apis")]
    pub fn get_outputs(
        &self,
        _addr: &Address,
        _include_spent: Option<bool>,
        _type: Option<OutputType>,
    ) -> Result<OutputsAddressResponse> {
        todo!()
    }

    /// Get all stored receipts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_receipts().unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_receipts(&self) -> Result<Vec<ReceiptTuple>> {
        todo!()
    }

    /// Get all stored receipts for a given migration index. (`/api/v1/receipts/{migratedAt}`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_receipts_at_index(1000).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_receipts_at_index(&self, _idx: u64) -> Result<Vec<ReceiptTuple>> {
        todo!()
    }

    /// Get information about the treasury. (`/api/v1/treasury`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_treasury().unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_treasury(&self) -> Result<TreasuryResponse> {
        todo!()
    }

    /// Get the included message of a transaction.
    /// (`/api/v1/transactions/{transactionId}/included-message`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_message_in_transaction(
    ///     &"af7579fb57746219561072c2cc0e4d0fbb8d493d075bd21bf25ae81a450c11ef".parse().unwrap()
    /// ).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_message_in_transaction(&self, _id: &TransactionId) -> Result<Message> {
        todo!()
    }

    /// Get a milestone by a given milestone index. (`/api/v1/milestones/{index}`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_milestone(154862).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_milestone(&self, _idx: u64) -> Result<MilestoneResponse> {
        todo!()
    }

    /// Get all UTXO changes of a given milestone. (`/api/v1/milestones/{index}/utxo-changes`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use iota_client::Client;
    ///
    /// let client = Client::new();
    /// let response = client.get_milestone_utxo_changes(154862).unwrap();
    /// ```
    #[cfg(feature = "rest-apis")]
    pub fn get_milestone_utxo_changes(&self, _idx: u64) -> Result<UtxoChangesResponse> {
        todo!()
    }

    #[cfg(feature = "rest-apis")]
    pub fn get_peers(&self) -> Result<()> {
        todo!()
    }

    #[cfg(feature = "rest-apis")]
    pub fn add_peer(&self, _peer: ()) -> Result<()> {
        todo!()
    }

    #[cfg(feature = "rest-apis")]
    pub fn get_peer_information(&self, _id: &str) -> Result<()> {
        todo!()
    }

    #[cfg(feature = "rest-apis")]
    pub fn delete_peer(&self, _id: &str) -> Result<()> {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_latest_milestones<F>(&self, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_confirmed_milestones<F>(&self, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_messages<F>(&self, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_referenced_messages<F>(&self, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_messages_by_index<F>(&self, _idx: &[u8], _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_message_metadata<F>(&self, _id: &MessageId, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_output<F>(&self, _id: &OutputId, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_transaction_included_message<F>(
        &self,
        _id: &TransactionId,
        _fn: F,
    ) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }

    #[cfg(feature = "event-apis")]
    pub fn subscribe_outputs_by_address<F>(&self, _addr: &Address, _fn: F) -> Result<()>
    where
        F: FnMut(),
    {
        todo!()
    }
}
