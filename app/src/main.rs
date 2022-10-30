use yew::prelude::*;
use ethers::{prelude::*};

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World2" } </h1>
    }
}

abigen!(
    SimpleContract,
    "./src/contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

fn main() {

    // let transport = web3::transports::Http::new("https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W")?;
    // let web3 = web3::Web3::new(transport);

    // println!("Calling Accounts");

    // https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W

    yew::start_app::<App>();
}