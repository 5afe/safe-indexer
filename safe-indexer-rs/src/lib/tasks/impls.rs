use crate::config;
use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransaction, Topic, BlockNumber};
use anyhow::Result;

pub(crate) async fn tx_hashes_for_topic(safe_address: &str, from: BlockNumber, topic: Topic) -> Result<Vec<String>> {
    log::debug!("LOG REQUEST FOR TOPIC {:#?}", &topic);

    let client = reqwest::Client::new();
    let request = RpcRequest::build_get_logs(&safe_address, topic, from);

    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<RpcResponse<Vec<RpcTransaction>>>(&response)?;
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
}
