use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransactionLog, Topic, BlockNumber, RpcTransaction};
use crate::{config, number_utils};
use serde::de::DeserializeOwned;

pub struct RpcClient {
    http_client: reqwest::Client,
}

impl RpcClient {
    pub fn new(http_client: reqwest::Client) -> Self {
        RpcClient {
            http_client
        }
    }

    pub async fn get_current_block(&self) -> anyhow::Result<u64> {
        let request = RpcRequest::build_get_current_block();
        let response = self.send_request::<String>(&request).await?;
        let latest_block_number = crate::number_utils::from_hex_string(&response.result)?;
        Ok(latest_block_number)
    }

    pub async fn get_transaction_hashes_for_event(&self, safe_address: &str, from: u64, topic: Topic) -> anyhow::Result<Vec<String>> {
        let from = BlockNumber::Value(number_utils::to_hex_string(from)?);
        let request = RpcRequest::build_get_logs(&safe_address, topic, from);
        let response = self.send_request::<Vec<RpcTransactionLog>>(&request).await?;
        Ok(response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
    }

    pub async fn get_transaction(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction> {
        let request = RpcRequest::build_get_transaction_by_hash(tx_hash);
        let response = self.send_request::<RpcTransaction>(&request).await;
        Ok(response.unwrap().result)
    }

    async fn send_request<T: DeserializeOwned>(&self, rpc_request: &RpcRequest) -> anyhow::Result<RpcResponse<T>> {
        let body = self.http_client.post(config::node_uri())
            .json(rpc_request)
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str::<RpcResponse<T>>(&body)?)
    }
}
