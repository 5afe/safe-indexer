use crate::config;
use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransaction, Topic, BlockNumber};
use anyhow::Result;

// TODO remove duplication here, pass the `Topic` as a parameter
// See how the `producer` and `consumer` behave in this case
pub(crate) async fn check_incoming_eth(safe_address: &str, from: BlockNumber) -> Result<Vec<String>> {
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

pub(crate) async fn check_execution_success(safe_address: &str, from: BlockNumber) -> Result<Vec<String>> {
    let client = reqwest::Client::new();
    let request = RpcRequest::build_get_logs(&safe_address, Topic::ExecutionSuccess, from);

    log::debug!("EXECUTION FAILURE BODY {:#?}", serde_json::to_string(&request).unwrap());
    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<RpcResponse<Vec<RpcTransaction>>>(&response).expect("Result deserialize failed");
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
}

pub(crate) async fn check_execution_failure(safe_address: &str, from: BlockNumber) -> Result<Vec<String>> {
    let client = reqwest::Client::new();
    let request = RpcRequest::build_get_logs(&safe_address, Topic::ExecutionFailure, from);

    log::debug!("EXECUTION FAILURE BODY {:#?}", serde_json::to_string(&request).unwrap());
    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<RpcResponse<Vec<RpcTransaction>>>(&response).expect("Result deserialize failed");
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
}
