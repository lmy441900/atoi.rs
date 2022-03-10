use super::{AsyncSigner, Signer};
use async_trait::async_trait;

/// Signer that makes use of a [Ledger] hardware wallet.
///
/// [Ledger]: https://www.ledger.com/
#[derive(Default)]
pub struct LedgerSigner {}

impl Signer for LedgerSigner {
    fn generate_addresses(&self) {
        todo!()
    }

    fn signature_unlock(&self) {
        todo!()
    }
}

#[async_trait]
impl AsyncSigner for LedgerSigner {
    async fn generate_addresses(&self) {
        todo!()
    }

    async fn signature_unlock(&self) {
        todo!()
    }
}
