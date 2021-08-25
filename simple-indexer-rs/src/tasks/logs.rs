use celery::prelude::*;
use ethcontract::prelude::*;
use ethcontract::futures::join;
use ethcontract::futures::stream::StreamExt;
use crate::config;

#[celery::task]
pub async fn check_incoming_eth(safe_address: String) -> TaskResult<String> {
    let http = Http::new(config::node_uri().as_str()).expect("transport failed");
    let web3 = Web3::new(http);

    // let accounts = web3.eth().accounts().await.expect("get accounts failed");

    Ok(String::from("There was eth transferred"))
}
