use serde::de::DeserializeOwned;

use crate::config;
use crate::rpc::models::{
    BlockNumber, RpcRequest, RpcResponse, RpcTransaction, RpcTransactionLog, Topic,
};
use crate::utils::number_utils;

pub struct RpcClient {
    http_client: reqwest::Client,
}

impl RpcClient {
    pub fn new(http_client: reqwest::Client) -> Self {
        RpcClient { http_client }
    }

    pub async fn get_current_block(&self) -> anyhow::Result<u64> {
        let request = RpcRequest::build_get_current_block();
        let response = self.send_request::<String>(&request).await?;
        let latest_block_number = crate::utils::number_utils::from_hex_string(&response.result)?;
        Ok(latest_block_number)
    }

    pub async fn get_transaction_log(
        &self,
        safe_address: &str,
        from: u64,
        topic: Topic,
    ) -> anyhow::Result<Vec<RpcTransactionLog>> {
        let from = BlockNumber::Value(number_utils::to_hex_string(from)?);
        let request = RpcRequest::build_get_logs(&safe_address, topic, from);
        let response = self
            .send_request::<Vec<RpcTransactionLog>>(&request)
            .await?;
        Ok(response.result)
    }

    pub async fn get_transaction_hashes_for_event(
        &self,
        safe_address: &str,
        from: u64,
        topic: Topic,
    ) -> anyhow::Result<Vec<String>> {
        let transaction_logs = self.get_transaction_log(safe_address, from, topic).await?;
        Ok(transaction_logs
            .iter()
            .map(|result| result.transaction_hash.to_string())
            .collect())
    }

    pub async fn get_transaction(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction> {
        let request = RpcRequest::build_get_transaction_by_hash(tx_hash);
        let response = self.send_request::<RpcTransaction>(&request).await;
        Ok(response.unwrap().result)
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        rpc_request: &RpcRequest,
    ) -> anyhow::Result<RpcResponse<T>> {
        let body = self
            .http_client
            .post(config::node_uri())
            .json(rpc_request)
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str::<RpcResponse<T>>(&body)?)
    }
}
