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

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| "");
    {
        let value = value.clone();
        use_effect_with_deps(move |_| {
            // let value = value;
            spawn_local(async move {
                let addr = "0xDe990C95CCA66bAed94FdF67DaB43BCfC6d3B3aD"
                    .parse::<Address>()
                    .expect("error");
                let client = Provider::<Http>
                    ::try_from(
                        "https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W"
                    )
                    .unwrap();
                let abi: Abi = Request::get("https://raw.githubusercontent.com/3tomcha/voice_erc721-/master/app/src/contract_abi.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                // console::log_1(&JsValue::from(abi.to_string()));
                let contract = Contract::new(addr, abi, client);
                let name = contract
                    .method::<_, String>("name", ())
                    .expect("error")
                    .call().await
                    .expect("error");
                console::log_1(&JsValue::from(name));
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