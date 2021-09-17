use crate::rpc::models::{RpcRequest, RequestParam, BlockNumber, Topic};

impl RpcRequest {
    pub fn build_get_logs(address: &str, topic: Topic) -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getLogs".to_string(),
            id: "1".to_string(),
            params: vec![RequestParam::Log {
                from_block: Some(BlockNumber::Earliest),
                address: address.to_string(),
                topics: vec![vec![topic.get_hash()]],
            }],
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
        match self{
            Topic::IncomingEth => String::from("0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d"),
            Topic::ExecutionSuccess => String::from("0x17d07c166ac116d0e12be36f9d21670849dd8f7a6dd91a7edf56c83f07c8622e"),
            Topic::ExecutionFailure => String::from("0x9304a5b1a17f881c79a07cd051dc098979104b9d1574dcdc4ad5d1b6acae4900"),
        }
    }
}
