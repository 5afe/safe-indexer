use crate::models::json_rpc::{Request, RequestParam, BlockNumber};

impl Request {
    pub fn build_incoming_eth(address: &str) -> Self {
        Request {
            jsonrpc: "2.0".to_string(),
            method: "eth_getLogs".to_string(),
            id: "1".to_string(),
            params: vec![RequestParam::Log {
                from_block: Some(BlockNumber::Earliest),
                address: address.to_string(),
                topics: vec![vec!["0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d".to_string()]],
            }],
        }
    }
}
