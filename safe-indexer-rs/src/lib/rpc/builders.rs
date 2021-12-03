use crate::rpc::models::{BlockNumber, RequestParam, RpcRequest, Topic};

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

impl Topic {
    pub fn get_hash(&self) -> String {
        match self {
            Topic::IncomingEth => {
                String::from("0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d")
            }
            Topic::ExecutionSuccess => {
                String::from("0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e")
            }
            Topic::ExecutionFailure => {
                String::from("0x23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23")
            }
            Topic::SafeMultisigTransaction => {
                String::from("0x66753cd2356569ee081232e3be8909b950e0a76c1f8460c3a5e3c2be32b11bed")
            }
        }
    }
}
