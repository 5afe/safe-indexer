use crate::{
    decoders::topic::models::Topic,
    rpc::models::{BlockNumber, RequestParam, RpcRequest},
};

impl RpcRequest {
    pub fn build_get_logs(address: &str, topic: Topic, from: BlockNumber) -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getLogs".to_string(),
            id: "1".to_string(),
            params: vec![RequestParam::Log {
                from_block: Some(from),
                address: address.to_string(),
                topics: vec![vec![topic.get_hash()]],
            }],
        }
    }

    pub fn build_get_transaction_by_hash(tx_hash: &str) -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getTransactionByHash".to_string(),
            id: "1".to_string(),
            params: vec![RequestParam::Single(tx_hash.to_string())],
        }
    }

    pub fn build_get_current_block() -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_blockNumber".to_string(),
            id: "1".to_string(),
            params: vec![],
        }
    }
}
