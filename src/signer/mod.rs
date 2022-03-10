//! Cryptographic signers.

#[cfg(feature = "ledger")]
mod ledger;
#[cfg(feature = "mnemonic")]
mod mnemonic;
#[cfg(feature = "stronghold")]
mod stronghold;

// Exports.
#[cfg(feature = "ledger")]
pub use self::ledger::LedgerSigner;
#[cfg(feature = "mnemonic")]
pub use self::mnemonic::MnemonicSigner;
#[cfg(feature = "stronghold")]
pub use self::stronghold::StrongholdSigner;

use async_trait::async_trait;

/// Interfaces for a synchronous signer.
pub trait Signer {
    fn generate_addresses(&self);
    fn signature_unlock(&self);
}

/// Interfaces for an asynchronous signer.
#[async_trait]
pub trait AsyncSigner {
    async fn generate_addresses(&self);
    async fn signature_unlock(&self);
}
