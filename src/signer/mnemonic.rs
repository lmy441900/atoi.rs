use super::{AsyncSigner, Signer};
use async_trait::async_trait;

/// Signer that makes use of a [BIP-39] mnemonic string.
///
/// [BIP-39]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
pub struct MnemonicSigner {}

impl Signer for MnemonicSigner {
    fn generate_addresses(&self) {
        todo!()
    }

    fn signature_unlock(&self) {
        todo!()
    }
}

#[async_trait]
impl AsyncSigner for MnemonicSigner {
    async fn generate_addresses(&self) {
        todo!()
    }

    async fn signature_unlock(&self) {
        todo!()
    }
}
