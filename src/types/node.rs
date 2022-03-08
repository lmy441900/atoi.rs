use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Auth {
    Basic { username: String, password: String },
    Jwt(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub url: Url,
    pub auth: Option<Auth>,
}
