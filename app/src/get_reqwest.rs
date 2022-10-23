use std::env;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let url = format!("{}me?access_token={}", env!("URL"), env!("ACCESS_TOKEN"));
    println!("{}", url);
    let body = reqwest::get(url)
                .await?
                .text()
                .await?;
    println!("body = {:?}", body);
    Ok(())
}