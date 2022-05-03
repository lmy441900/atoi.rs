use atoi::comm::http::CurlHttpClient;
use atoi::types::Node;
use atoi::Client;

fn main() {
    let client = Client::builder()
        .node(Node {
            url: String::from("https://api.alphanet.iotaledger.net"),
            auth: None,
        })
        .http(Box::new(CurlHttpClient::new()))
        .build();

    let health = client.health();
    let info = client.info();

    println!("{:?}", health);
    println!("{:?}", info);
}
