use crate::config;
use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransaction, Topic, BlockNumber};
use anyhow::Result;

pub (crate) async fn check_incoming_eth(safe_address: &str, from: BlockNumber) -> Result<Vec<String>>{
    let client = reqwest::Client::new();
    let request = RpcRequest::build_get_logs(&safe_address, Topic::IncomingEth, from);

    log::debug!("INCOMING ETH REQ BODY {:#?}", serde_json::to_string(&request).unwrap());
    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<RpcResponse<Vec<RpcTransaction>>>(&response).expect("Result deserialize failed");
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
}
