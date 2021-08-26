use crate::config;
use celery::prelude::*;
// use ethcontract::futures::join;
// use ethcontract::futures::stream::StreamExt;
use ethcontract::{common::DeploymentInformation, log::*, prelude::*};

ethcontract::contract!(
    "abis/GnosisSafeL2.json",
    deployments {
        1 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        4 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        42 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        5 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        56 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        100 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        137 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        246 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        4002 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        42161 =>  "0x3E5c63644E683549055b9Be8653de26E0B4CD36E",
        73799 => "0x3E5c63644E683549055b9Be8653de26E0B4CD36E"
    },);

#[celery::task]
pub async fn check_incoming_eth(safe_address: String) -> TaskResult<String> {
    let http = Http::new(config::node_uri().as_str()).expect("transport failed");
    let web3 = Web3::new(http);

    let safe_l2 = GnosisSafeL2::deployed(&web3)
        .await
        .expect("locating deployed contract failed");

    let address: Address = safe_address.parse().expect(&format!(
        "Couldn't parse safe address from: {}",
        &safe_address
    ));
    // let filter = LogFilterBuilder::new(web3)
    //     .from_block(BlockNumber::Earliest)
    //     .address(vec![address])
    //     .topic0(Topic::This(safe_l2.));
    Ok(String::from("There was eth transferred"))
}
