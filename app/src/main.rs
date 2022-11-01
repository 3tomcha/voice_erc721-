use yew::prelude::*;
use ethers::{ prelude::* };
use ethers_core::{ abi::Abi, types::{ Address, H256 } };
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use ethers_contract::Contract;
use web_sys::console;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use ethers_providers::{ Ws };

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| "");
    {
        let value = value.clone();
        use_effect_with_deps(move |_| {
            // let value = value;
            spawn_local(async move {
                let addr = "0x6B54d1665a0199e910cFE8D40C2eeeA0111Fd51c"
                    .parse::<Address>()
                    .expect("error");
                // let client = Provider::<Http>
                //     ::try_from(
                //         "https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W"
                //     )
                //     .unwrap();
                let client = Provider::<Ws>::connect("wss://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W").await.expect("error");
                let abi: Abi = Request::get("https://raw.githubusercontent.com/3tomcha/voice_erc721-/master/app/src/contract_abi.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                // console::log_1(&JsValue::from(abi.to_string()));
                let contract = Contract::new(addr, abi, client);
                let receipient_address = "0x32a9E70324862ef7BF8bA7610AF701822ddE5364".parse::<Address>().expect("error");
                let token_uri = String::from("https://gateway.pinata.cloud/ipfs/QmTFCG9UPu5gfa2edbXEZVcr6BLu8NLzV14DWCLsedNFUd");
                contract
                    .method::<_, String>("mintNFT", (receipient_address, token_uri))
                    .expect("error")
                    .send().await
                    .expect("error");
                // console::log_1(&JsValue::from(res));
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