use atoi::comm::http::dummy::DummyHttpClient;
use atoi::types::Node;
use atoi::AsyncClient;

#[async_std::main]
async fn main() {
    let client = AsyncClient::builder()
        .node(Node {
            url: String::from("https://example.iota.org"),
            auth: None,
        })
        .http(Box::new(DummyHttpClient::new()))
        .build();

    let health = client.health().await;
    let info = client.info().await;

    async_std::println!("{:?}", health).await;
    async_std::println!("{:?}", info).await;
}
