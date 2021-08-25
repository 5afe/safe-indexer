use std::env;

pub fn redis_uri() -> String {
    env::var("REDIS_URI").expect("Please set your REDIS_URI")
}

pub fn node_uri() -> String {
    env::var("NODE_URI").expect("Please set your NODE_URI")
}
