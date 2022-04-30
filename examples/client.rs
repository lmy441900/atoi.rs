use atoi::comm::http::dummy::DummyHttpClient;
use atoi::types::Node;
use atoi::Client;

fn main() {
    let client = Client::builder()
        .node(Node {
            url: String::from("https://example.iota.org"),
            auth: None,
        })
        .http(Box::new(DummyHttpClient::new()))
        .build();

    let health = client.health();
    let info = client.info();

    println!("{:?}", health);
    println!("{:?}", info);
}
