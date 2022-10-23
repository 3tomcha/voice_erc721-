#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let body = reqwest::get("https://www.rust-lang.org")
                .await?
                .text()
                .await?;
    println!("body = {:?}", body);
    Ok(())
}