//! IOTA client with synchronous interfaces.

mod private;
mod public;

use derive_builder::Builder;

/// The entry point of IOTA.
#[derive(Default, Builder)]
#[builder(pattern = "owned", setter(strip_option), default)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}
