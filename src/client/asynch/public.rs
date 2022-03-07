use super::{AsyncClient, AsyncClientBuilder};

impl AsyncClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn builder() -> AsyncClientBuilder {
        AsyncClientBuilder::default()
    }
}
