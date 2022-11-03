use yew::prelude::*;
use ethers::{ prelude::* };
use ethers_core::{ abi::Abi, types::{ Address, H256 } };
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use ethers_contract::Contract;
use web_sys::console;
use web_sys::Window;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use ethers_providers::{ Ws };
use std::future::Future;
use mopa::Any;
use gloo_utils::format::JsValueSerdeExt;

// #[wasm_bindgen]
// Provider::<Http>
#[macro_use]
extern crate mopa;

#[wasm_bindgen(module = "/src/index.js")]
extern "C" {
    async fn detectEthereumProvider() -> JsValue;
}

// pub trait FromJsValue: WasmDescribe {
//     type 
// }

// pub fn render() -> Provider<Http> {
//     let provider: Provider<Http> = Vec::new();
//     detectEthereumProvider(&provider);
// }

// extern "C" {
//     async fn detectEthereumProvider() -> wasm_bindgen::JsValue;
// }

// #[wasm_bindgen]
// extern "C" {
//     fn detect_ethereum_provider(this: &Client) -> String;
// }


// extern {
//     type Native;
//     #[wasm_bindgen(method, js_name = "detectEthereumProvider")]
//     pub fn detect_ethereum_provider(this: &Client) -> String;
// }
// #[wasm_bindgen]
// extern {
//     static: native: Native;
// }

// pub fn test() {
//     spawn_local(async {
//         let provider: Provider<Http> = detectEthereumProvider().await;
//         let chainId = provider.request({method: "eth_chainId"}).await;
//         // console::log_1(&provider);
//         console::log_1(&chainId);
//     });
// }

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
                let mut client = detectEthereumProvider().await;
                let client2: mopa::Any = JsValueSerdeExt::into_serde(&client).unwrap();
                mopafy!(client2);
                // let client = provider::Provider::<Http>();
                let abi: Abi = Request::get("https://raw.githubusercontent.com/3tomcha/voice_erc721-/master/app/src/contract_abi.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                // console::log_1(&JsValue::from(abi.to_string()));
                let mut contract = Contract::new(addr, abi, client2);
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

// #[function_component(App)]
// fn app() -> Html {
//     let value = use_state(|| "");
//     {
//         let value = value.clone();
//         use_effect_with_deps(move |_| {
//             // let value = value;
//             spawn_local(async move {
//                 let addr = "0x6B54d1665a0199e910cFE8D40C2eeeA0111Fd51c"
//                     .parse::<Address>()
//                     .expect("error");
//                 // let client = Provider::<Http>
//                 //     ::try_from(
//                 //         "https://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W"
//                 //     )
//                 //     .unwrap();
//                 let client = Provider::<Ws>::connect("wss://eth-goerli.g.alchemy.com/v2/lQQcdlj4Fye1AKw5R94wVNA-BDKevm0W").await.expect("error");
//                 let abi: Abi = Request::get("https://raw.githubusercontent.com/3tomcha/voice_erc721-/master/app/src/contract_abi.json")
//                             .send()
//                             .await
//                             .unwrap()
//                             .json()
//                             .await
//                             .unwrap();
//                 // console::log_1(&JsValue::from(abi.to_string()));
//                 let contract = Contract::new(addr, abi, client);
//                 let receipient_address = "0x32a9E70324862ef7BF8bA7610AF701822ddE5364".parse::<Address>().expect("error");
//                 let token_uri = String::from("https://gateway.pinata.cloud/ipfs/QmTFCG9UPu5gfa2edbXEZVcr6BLu8NLzV14DWCLsedNFUd");
//                 contract
//                     .method::<_, String>("mintNFT", (receipient_address, token_uri))
//                     .expect("error")
//                     .send().await
//                     .expect("error");
//                 // console::log_1(&JsValue::from(res));
//                 // value.set(owner);
//             });
//             || ()
//         }, ());
//     }
//     html! {
//         <h1>{ "aaa" }</h1>
//     }
// }

fn main() {
    yew::start_app::<App>();
}