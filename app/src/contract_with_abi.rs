use ethers::{prelude::*, utils::Anvil};
use eyre::Result;
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};

abigen!(
    SimpleContract,
    "/contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();
    let source = Path::new(&env!("CARGO_MANIFEST_DIR")).join("contract.sol");
    let compiled = Solc::default().compile_source(source).expect("Could not compile contracts");
    Ok(())
}