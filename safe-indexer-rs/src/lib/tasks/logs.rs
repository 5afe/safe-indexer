use crate::config;
use celery::prelude::*;
use ethcontract::prelude::*;
use crate::models::json_rpc::{Request, Response};

#[celery::task]
pub async fn check_incoming_eth(safe_address: String) -> TaskResult<Vec<String>> {
    let client = reqwest::Client::new();
    let request = Request::build_incoming_eth(&safe_address);

    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<Response>(&response).expect("Result deserialize failed");
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
}

// ethcontract-rs implementation
// ethcontract::contract!(
//     "abis/GnosisSafeL2.json",
//     deployments {
//         1 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         4 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         42 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         5 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         56 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         100 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         137 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         246 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         4002 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         42161 =>  "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
//         73799 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E"
//     },);
//
// #[celery::task]
// pub async fn check_incoming_eth(safe_address: String) -> TaskResult<String> {
//     let http = Http::new(config::node_uri().as_str()).expect("transport failed");
//     let web3 = Web3::new(http);
//
//     let safe_l2 = GnosisSafeL2::deployed(&web3)
//         .await
//         .expect("locating deployed contract failed");
//
//     // The specific safe I would like to filter by
//     let _address: Address = safe_address.parse().expect(&format!(
//         "Couldn't parse safe address from: {}",
//         &safe_address
//     ));
//
//     let eth_transfer_topic: H256 =
//         "3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d"
//             .parse()
//             .expect("Topic hash fails");
//     let log = safe_l2
//         .all_events()
//         .from_block(BlockNumber::Earliest)
//         .topic0(Topic::This(eth_transfer_topic))
//         .query()
//         .await
//         .expect("log failed");
//
//     log::error!("{:#?}", log);
//     // let filter = LogFilterBuilder::new(web3)
//     //     .from_block(BlockNumber::Earliest)
//     //     .address(vec![address])
//     //     .topic0(Topic::This(safe_l2.));
//     Ok(String::from("There was eth transferred"))
// }
