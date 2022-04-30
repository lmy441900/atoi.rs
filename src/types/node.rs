//! Node information.

use alloc::string::String;

/// Authentication information to an IOTA node.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Auth {
    Basic { username: String, password: String },
    Jwt(String),
}

/// Information needed to connect to an IOTA node.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    pub url: String,
    pub auth: Option<Auth>,
}
