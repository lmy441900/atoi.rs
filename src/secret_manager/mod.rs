//! Secret managers.

use async_trait::async_trait;

/// Interfaces for a synchronous secret manager.
pub trait SecretManager {
    fn gen_addrs(&self);
    fn sig_unlock(&self);
}

/// Interfaces for an asynchronous secret manager.
#[async_trait]
pub trait AsyncSecretManager {
    async fn gen_addrs(&self);
    async fn sig_unlock(&self);
}
