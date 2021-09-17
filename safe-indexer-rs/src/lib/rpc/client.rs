use crate::rpc::models::{RpcRequest, RpcResponse};
use crate::config;
use serde::Deserialize;
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
