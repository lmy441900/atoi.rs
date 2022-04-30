//! IOTA client with synchronous interfaces.

mod private;
mod public;

use crate::comm::http::HttpClient;
use crate::types::Node;
use alloc::boxed::Box;
use typed_builder::TypedBuilder;

/// The entry point of IOTA.
#[derive(Default, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Client {
    pub node: Option<Node>,
    pub http: Option<Box<dyn HttpClient>>,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }
}
