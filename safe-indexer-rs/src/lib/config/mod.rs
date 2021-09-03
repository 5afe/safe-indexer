use std::env;
use crate::models::json_rpc::BlockNumber;

pub fn redis_uri() -> String {
    env::var("REDIS_URI").expect("Please set your REDIS_URI")
}

pub fn node_uri() -> String {
    env::var("NODE_URI").expect("Please set your NODE_URI")
}

pub fn start_block() -> BlockNumber {
    BlockNumber::Value(String::from("9227661"))
}
