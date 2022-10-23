use std::env;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let url = env!("URL");
    let body = reqwest::get(url)
                .await?
                .text()
                .await?;
    println!("body = {:?}", body);
    Ok(())
}