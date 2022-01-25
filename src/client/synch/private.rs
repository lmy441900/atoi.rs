// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Private API implementations of [Client].

use super::super::common::{deserialize, MAX_CONTENT_LENGTH};
use super::Client;
use crate::types::*;
use crate::{Error, Result};
use log::warn;
use std::io::Read;
use url::Url;

/// Private APIs.
impl Client {
    /// The default `User-Agent` string used in the HTTP client.
    pub(super) const DEFAULT_USER_AGENT: &'static str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
        " (ureq/",
        env!("UREQ_VERSION"),
        ")"
    );

    /// Perform a HTTP GET to (get from) `url` using [Self::agent].
    pub(super) fn http_get(&self, url: &Url) -> Result<Vec<u8>> {
        let resp = match self.agent.request_url("GET", url).call() {
            Ok(resp) => resp,
            Err(err) => match err {
                // We want to keep processing on HTTP errors returned by the node software.
                ureq::Error::Status(_, resp) => resp,
                ureq::Error::Transport(tnsp) => return Err(Error::NetworkError(Box::new(tnsp))),
            },
        };

        // The HTTP header.
        let status = resp.status();
        let status_text = resp.status_text().to_string();

        // Always read the body first; it has the final saying on an error.
        //
        // NOTE: The node won't necessarily return `content-length. If there would be one,
        // [ureq::Response::into_reader()] would obey it. We just allocate a maximum allowed buffer,
        // read all octets on wire, then shrink the buffer.
        let mut body = Vec::with_capacity(MAX_CONTENT_LENGTH);
        let bytes_read = match resp
            .into_reader()
            .take(MAX_CONTENT_LENGTH.try_into().unwrap())
            .read_to_end(&mut body)
        {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(Error::NetworkError(Box::new(err)));
            }
        };

        // If we happen to have read `MAX_CONTENT_LENGTH` bytes, then it's possible that the
        // returned content have exceeded our limit.
        if bytes_read >= MAX_CONTENT_LENGTH {
            warn!("response may have exceeded the maximum allowed content length");
        }

        // If the node returns an error, we deserialize and return the error body.
        if status != 200 {
            return match deserialize::<ErrorBody<ErrorResponse>>(&body) {
                Ok(err_body) => Err(Error::NodeError {
                    code: err_body.error.code,
                    reason: err_body.error.message,
                }),
                Err(err) => {
                    // An error during body deserialization doesn't break the flow - we just fall
                    // back to the HTTP status.
                    warn!("invalid error body: {}", err);
                    Err(Error::NodeError {
                        code: status,
                        reason: status_text,
                    })
                }
            };
        }

        // Save some memory.
        body.shrink_to_fit();

        Ok(body)
    }

    /// Perform HTTP GETs to (get from) all the nodes.
    pub(super) fn http_get_from_all_nodes(&self, path: &str) -> Vec<Result<Vec<u8>>> {
        self.nodes
            .iter()
            .map(|node| {
                let url = match node.join(path) {
                    Ok(url) => url,
                    Err(err) => return Err(Error::InvalidInput(Box::new(err))),
                };

                self.http_get(&url)
            })
            .collect()
    }

    /// Get a raw response from any of the stored nodes.
    pub(super) fn http_get_from_any_node(&self, path: &str) -> Result<Vec<u8>> {
        for node in &self.nodes {
            let url = match node.join(path) {
                Ok(url) => url,
                // An error from the user explodes early.
                Err(err) => return Err(Error::InvalidInput(Box::new(err))),
            };

            match self.http_get(&url) {
                Ok(resp) => return Ok(resp),
                Err(_) => continue,
            };
        }

        // If the loop ends before a return, then all nodes have failed.
        Err(Error::NoAvailableNode)
    }

    /// Perform a synchronous HTTP POST request to any of the stored node.
    pub(super) fn http_post_to_any_node(&self, _path: &str, _payload: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    // /// Performe a synchronous HTTP DELETE request to any of the stored node.
    // pub(super) fn http_delete(&self, _path: &str) -> Result<()> {
    //     todo!()
    // }
}
