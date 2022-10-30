use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = IpfsClient::default();
    let john = json!({
        "attributes": [
            {
                "trait_type": "name",
                "value": "Tomoya Kobayashi"
            }
        ]
        });
    let data = Cursor::new(john.to_string());

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
