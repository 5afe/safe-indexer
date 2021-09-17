extern crate celery;
extern crate celery_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate reqwest;

use dotenv::dotenv;
use tokio::time::sleep;
use std::time::Duration;
use crate::rpc::client::RpcClient;
use crate::rpc::models::BlockNumber;

pub mod config;
pub mod db;
pub mod tasks;
pub mod rpc;
pub mod number_utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let rpc_client = RpcClient::new(reqwest::Client::new());

    let start_block = config::start_block();
    let mut next_block = start_block;

    let time_tick_interval = 5000;
    let block_tick_interval = 1000;
    loop {
        let latest_block = rpc_client.get_current_block().await?;
        if next_block <= latest_block {
            next_block += block_tick_interval;
        } else {
            log::debug!("Finished the block chain, waiting for 10 seconds");
            sleep(Duration::from_millis(10000)).await;
            continue;
        }

        log::info!("Starting at block: {:#?}", next_block);
        let result = tasks::logs::check_incoming_eth_impl("0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67", BlockNumber::Value(number_utils::to_hex_string(next_block)?)).await?;
        log::debug!("get eth result: {:#?}", result);
        sleep(Duration::from_millis(time_tick_interval)).await;
        println!("{} milliseconds have passed", &time_tick_interval);
    }
}
