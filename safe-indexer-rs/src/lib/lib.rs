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
use crate::rpc::models::{BlockNumber, Topic};
use tokio::try_join;

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
    let time_tick_interval = config::iteration_sleep_interval();
    let block_tick_interval = config::block_step();
    let safe_address = "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67";

    let mut next_block = start_block;
    loop {
        let latest_block = rpc_client.get_current_block().await?;
        if next_block >= latest_block {
            log::debug!("Finished the block chain, waiting for 10 seconds");
            sleep(Duration::from_millis(10000)).await;
            continue;
        }

        let (result_incoming_eth, result_exec_success, result_exec_failure, result_multisig_txs) = try_join!(
            tasks::impls::tx_hashes_for_topic(safe_address, BlockNumber::Value(number_utils::to_hex_string(next_block)?), Topic::IncomingEth),
            tasks::impls::tx_hashes_for_topic(safe_address, BlockNumber::Value(number_utils::to_hex_string(next_block)?), Topic::ExecutionSuccess),
            tasks::impls::tx_hashes_for_topic(safe_address, BlockNumber::Value(number_utils::to_hex_string(next_block)?), Topic::ExecutionFailure),
            tasks::impls::tx_hashes_for_topic(safe_address, BlockNumber::Value(number_utils::to_hex_string(next_block)?), Topic::SafeMultisigTransaction),
        )?;

        log::info!("========================================================================");
        log::info!("Starting at block             : {:#?}", start_block);
        log::info!("Requesting logs for block     : {:#?}", &next_block);
        log::info!("Current block                 : {:#?}", &latest_block);
        log::info!("Block step interval           : {:#?}", &block_tick_interval);
        log::info!("Incoming eth tx hashes        : {:#?}", result_incoming_eth);
        log::info!("Execution success hashes      : {:#?}", result_exec_success);
        log::info!("Execution failure hashes      : {:#?}", result_exec_failure);
        log::info!("Execution Multisig hashes     : {:#?}", result_multisig_txs);
        log::info!("Sleeping for {} milliseconds", &time_tick_interval);
        log::info!("========================================================================");

        sleep(Duration::from_millis(time_tick_interval)).await;
        next_block += block_tick_interval;
    }
}
