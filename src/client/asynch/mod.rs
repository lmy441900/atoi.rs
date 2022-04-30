//! IOTA client with asynchronous interfaces.

mod private;
mod public;

use crate::comm::http::AsyncHttpClient;
use crate::types::Node;
use alloc::boxed::Box;
use typed_builder::TypedBuilder;

/// The entry point of IOTA, with asynchronous interfaces.
#[derive(Default, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct AsyncClient {
    pub node: Option<Node>,
    pub http: Option<Box<dyn AsyncHttpClient>>,
}

impl AsyncClient {
    pub fn new() -> Self {
        Default::default()
    }
}
