use crate::config;
use crate::decoders::http::decoder::HttpDataDecoder;
use crate::decoders::http::models::HttpDecoderInput;
use crate::decoders::EthDataDecoder;
use crate::loaders::{EventLoader, EventLooper};
use crate::rpc::models::Topic;
use crate::utils::number_utils::to_decimal;
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;
use tokio::try_join;

pub struct ConsoleLoggerEventLoop {
    start_block: u64,
    sleep_between_ticks_ms: u64,
    block_step: u64,
    http_data_decoder: HttpDataDecoder,
}

impl ConsoleLoggerEventLoop {
    pub fn new() -> Self {
        ConsoleLoggerEventLoop {
            start_block: config::start_block(),
            sleep_between_ticks_ms: config::iteration_sleep_interval(),
            block_step: config::block_step(),
            http_data_decoder: HttpDataDecoder::new(),
        }
    }
}

#[async_trait]
impl EventLooper for ConsoleLoggerEventLoop {
    async fn start(
        &self,
        safe_address: &str,
        event_loader: &(impl EventLoader + Sync),
    ) -> anyhow::Result<()> {
        let mut next_block = self.start_block;
        loop {
            let latest_block = event_loader.last_available_block().await?;
            if next_block >= latest_block {
                log::debug!("Finished the block chain, waiting for 10 seconds");
                sleep(Duration::from_millis(10000)).await;
                continue;
            }

            let (result_exec_success, result_exec_failure, result_multisig_txs) = try_join!(
                event_loader.get_transaction_hashes_for_event(
                    safe_address,
                    next_block,
                    Topic::ExecutionSuccess
                ),
                event_loader.get_transaction_hashes_for_event(
                    safe_address,
                    next_block,
                    Topic::ExecutionFailure
                ),
                event_loader.get_transaction_hashes_for_event(
                    safe_address,
                    next_block,
                    Topic::SafeMultisigTransaction
                ),
            )?;

            let all_results = {
                let mut all_results = vec![];
                all_results.extend(&result_exec_success);
                all_results.extend(&result_exec_failure);
                all_results.extend(&result_multisig_txs);
                all_results
            };

            let tx_results = {
                let mut tx_results = vec![];
                for tx_hash in all_results {
                    if !event_loader.was_tx_hash_checked(&tx_hash).await {
                        let rpc_tx = event_loader.process_tx_hash(&tx_hash).await?;
                        let decoder_input = HttpDecoderInput {
                            chain_id: to_decimal(&rpc_tx.chain_id)?,
                            data: rpc_tx.input.to_string(),
                        };
                        if self.http_data_decoder.can_decode(&decoder_input) {
                            let data_decoded = self.http_data_decoder.decode(decoder_input).await?;
                            tx_results.push(data_decoded);
                        }
                        // tx_results.push(event_loader.process_tx_hash(&tx_hash).await?);
                    }
                }
                tx_results
            };

            log::info!("========================================================================");
            log::info!("Starting at block             : {:#?}", self.start_block);
            log::info!("Requesting logs for block     : {:#?}", &next_block);
            log::info!("Current block                 : {:#?}", &latest_block);
            log::info!("Block step interval           : {:#?}", &self.block_step);
            log::info!("Execution success hashes      : {:#?}", result_exec_success);
            log::info!("Execution failure hashes      : {:#?}", result_exec_failure);
            log::info!("Execution Multisig hashes     : {:#?}", result_multisig_txs);
            log::info!("========================================================================");
            log::info!("New transactions in this loop : {:#?}", tx_results);
            log::info!("Sleeping for {} milliseconds", &self.sleep_between_ticks_ms);
            log::info!("========================================================================");

            sleep(Duration::from_millis(self.sleep_between_ticks_ms)).await;
            next_block += self.block_step;
        }
    }
}
