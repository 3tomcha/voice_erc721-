use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

#[tokio::main]
async fn main() {
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
