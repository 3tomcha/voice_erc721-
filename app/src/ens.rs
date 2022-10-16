use ethers::{prelude::*, utils::Anvil};
use eyre::Result;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().fork("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").spawn();
    let from = anvil.addresses()[0];
    let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap().with_sender(from);
    let tx = TransactionRequest::new().to("vitalik.eth").value(100_000);

    let receipt = provider.send_transaction(tx, None).await?.await?.ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;
    let tx = provider.get_transaction(receipt.transaction_hash).await?;

    println!("{}", serde_json::to_string(&tx)?);
    println!("{}", serde_json::to_string(&receipt)?);

    Ok(())
}
