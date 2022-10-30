use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = IpfsClient::default();
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
            ]
        });
    let data = Cursor::new(john.to_string());

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
