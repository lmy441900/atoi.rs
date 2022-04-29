/// Authentication information to an IOTA node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Auth {
    Basic { username: String, password: String },
    Jwt(String),
}

/// Information needed to connect to an IOTA node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub url: String,
    pub auth: Option<Auth>,
}
