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

pub mod config;
pub mod db;
pub mod tasks;
pub mod rpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let rpc_client = RpcClient::new(reqwest::Client::new());

    let start_block = config::start_block();
    let mut next_block = start_block;

    loop {
        let latest_block = rpc_client.get_current_block().await?;
        if next_block <= latest_block {
            next_block += 100;
        } else {
            log::debug!("Finished the block chain");
            continue;
        }

        log::info!("Starting at block: {:#?}", next_block);
        sleep(Duration::from_millis(2000)).await;
        println!("2 seconds passed");
    }
}
