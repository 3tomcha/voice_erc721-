use yew::prelude::*;
use ethers::{ prelude::* };
use ethers_core::{ abi::Abi, types::{ Address, H256 } };
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use ethers_contract::Contract;
use web_sys::console;
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| "");
    {
        let value = value.clone();
        use_effect_with_deps(move |_| {
            // let value = value;
            spawn_local(async move {
                let addr = "0x1261b8535678D59d1AeC7757aD3fbAE87A35F0FD"
                    .parse::<Address>()
                    .expect("error");
                let client = Provider::<Http>
                    ::try_from(
                        "https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W"
                    )
                    .unwrap();
                let file = File::open("./contract_abi.json").unwrap();
                let reader = BufReader::new(file);
                let abi: Abi = serde_json::from_reader(reader).unwrap();
                let contract = Contract::new(addr, abi, client);
                let owner = contract
                    .method::<_, String>("owner", ())
                    .expect("error")
                    .call().await
                    .expect("error").to_string();
                console::log_1(&owner.into());
                // value.set(owner);
            });
            || ()
        }, ());
    }
    html! {
        <h1>{ "aaa" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}