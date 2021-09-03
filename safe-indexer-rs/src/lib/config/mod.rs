use std::env;
use crate::rpc::models::BlockNumber;

pub fn redis_uri() -> String {
    env::var("REDIS_URI").expect("Please set your REDIS_URI")
}

pub fn node_uri() -> String {
    env::var("NODE_URI").expect("Please set your NODE_URI")
}

pub fn start_block() -> i64 {
    9227661
}
