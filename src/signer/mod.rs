//! Cryptographic signers.

use async_trait::async_trait;

/// Interfaces for a synchronous signer.
pub trait Signer {
    fn gen_addrs(&self);
    fn unlock(&self);
}

/// Interfaces for an asynchronous signer.
#[async_trait]
pub trait AsyncSigner {
    async fn gen_addrs(&self);
    async fn unlock(&self);
}
