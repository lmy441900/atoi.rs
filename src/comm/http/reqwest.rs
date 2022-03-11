//! [Reqwest]-supported HTTP communication backend.
//!
//! [Reqwest]: https://github.com/seanmonstar/reqwest

use super::AsyncHttpClient;
use crate::types::{Auth, Error, Node};
use crate::Result;
use async_trait::async_trait;
use log::debug;
use reqwest::{RequestBuilder, Response, StatusCode};

/// The default user agent string.
const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " reqwest/",
    env!("REQWEST_VERSION")
);

/// The maximum bytes of response body we can accept.
///
/// Having this limit helps with preventing malicious DoS attack.
const MAX_BODY_BYTES: u64 = 1024 * 1024;

/// HTTP client with [reqwest].
#[cfg(feature = "reqwest")]
pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

#[async_trait]
impl AsyncHttpClient for ReqwestHttpClient {
    async fn get(&self, node: &Node) -> Result<Vec<u8>> {
        let builder = self.client.get(node.url.clone());
        let builder = self::set_auth(builder, node.auth.as_ref());

        debug!("> GET {}", node.url);
        let response = match builder.send().await {
            Ok(resp) => resp,
            Err(err) => return Err(Error::NetworkError(Box::new(err))),
        };

        let status = response.status();
        debug!("< HTTP {}", status);

        // TODO: See if the body contains an error message; it should take precedence.

        if status != StatusCode::OK {
            return Err(Error::NodeError {
                code: status.as_u16(),
                reason: status
                    .canonical_reason()
                    .unwrap_or("unexpected status code")
                    .to_string(),
            });
        }

        self::read_body(response).await
    }

    async fn post(&self, node: &Node, body: &[u8]) -> Result<Vec<u8>> {
        let builder = self.client.post(node.url.clone());
        let builder = self::set_auth(builder, node.auth.as_ref());
        let builder = builder.body(body.to_vec());

        debug!("> POST {}", node.url);
        let response = match builder.send().await {
            Ok(resp) => resp,
            Err(err) => return Err(Error::NetworkError(Box::new(err))),
        };

        let status = response.status();
        debug!("< HTTP {}", status);

        // TODO: See if the body contains an error message; it should take precedence.

        if status != StatusCode::NO_CONTENT {
            return Err(Error::NodeError {
                code: status.as_u16(),
                reason: status
                    .canonical_reason()
                    .unwrap_or("unexpected status code")
                    .to_string(),
            });
        }

        self::read_body(response).await
    }

    async fn delete(&self, node: &Node) -> Result<Vec<u8>> {
        let builder = self.client.delete(node.url.clone());
        let builder = self::set_auth(builder, node.auth.as_ref());

        debug!("> DELETE {}", node.url);
        let response = match builder.send().await {
            Ok(resp) => resp,
            Err(err) => return Err(Error::NetworkError(Box::new(err))),
        };

        let status = response.status();
        debug!("< HTTP {}", status);

        // TODO: See if the body contains an error message; it should take precedence.

        if status != StatusCode::NO_CONTENT {
            return Err(Error::NodeError {
                code: status.as_u16(),
                reason: status
                    .canonical_reason()
                    .unwrap_or("unexpected status code")
                    .to_string(),
            });
        }

        self::read_body(response).await
    }
}

impl ReqwestHttpClient {
    /// Construct a new [ReqwestHttpClient] with defaults.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Set [Auth] from [Node] to [RequestBuilder].
fn set_auth(builder: RequestBuilder, auth: Option<&Auth>) -> RequestBuilder {
    match auth {
        Some(Auth::Basic { username, password }) => builder.basic_auth(username, Some(password)),
        Some(Auth::Jwt(token)) => builder.bearer_auth(token),
        None => builder,
    }
}

/// Read the HTTP payload (body) out from a [Response].
async fn read_body(mut response: Response) -> Result<Vec<u8>> {
    let bytes_to_read = if let Some(cl) = response.content_length() {
        cl
    } else {
        MAX_BODY_BYTES
    };

    let mut buffer = Vec::with_capacity(bytes_to_read.try_into().unwrap());

    loop {
        let chunk = match response.chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break,
            Err(err) => return Err(Error::NetworkError(Box::new(err))),
        };

        debug!("read {} bytes chunk", chunk.len());
        buffer.extend_from_slice(&chunk);
    }

    debug!("read {} bytes in total", buffer.len());
    buffer.shrink_to_fit();

    Ok(buffer)
}
