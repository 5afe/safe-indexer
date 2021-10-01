use std::env;
use std::str::FromStr;

pub fn redis_uri() -> String {
    env::var("REDIS_URI").expect("Please set your REDIS_URI")
}

pub fn node_uri() -> String {
    env::var("NODE_URI").expect("Please set your NODE_URI")
}

pub fn start_block() -> u64 {
    env_with_default("START_BLOCK_NUMBER", 9177270)
}

pub fn iteration_sleep_interval() -> u64 {
    env_with_default("ITERATION_SLEEP_INTERVAL", 5000)
}

pub fn block_step() -> u64 {
    env_with_default("BLOCK_STEP", 1000)
}

fn env_with_default<T: FromStr>(key: &str, default: T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    match env::var(key) {
        Ok(value) => value.parse().unwrap(),
        Err(_) => default,
    }
}
