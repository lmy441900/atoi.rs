use super::{AsyncSigner, Signer};
use async_trait::async_trait;

/// Signer that makes use of [Stronghold.rs].
///
/// [Stronghold.rs]: https://github.com/iotaledger/stronghold.rs
pub struct StrongholdSigner {}

impl Signer for StrongholdSigner {
    fn generate_addresses(&self) {
        todo!()
    }

    fn signature_unlock(&self) {
        todo!()
    }
}

#[async_trait]
impl AsyncSigner for StrongholdSigner {
    async fn generate_addresses(&self) {
        todo!()
    }

    async fn signature_unlock(&self) {
        todo!()
    }
}
