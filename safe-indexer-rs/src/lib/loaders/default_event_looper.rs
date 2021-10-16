use crate::config;
use crate::decoders::topic::decoder::{TopicDecodedParams, TopicDecoder, TopicDecoderInput};
use crate::decoders::EthDataDecoder;
use crate::loaders::{EventLoader, EventLooper};
use crate::rpc::models::{RpcTransactionLog, Topic};
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;
use tokio::try_join;

pub struct ConsoleLoggerEventLoop {
    start_block: u64,
    sleep_between_ticks_ms: u64,
    block_step: u64,
    topic_decoder: TopicDecoder,
}

impl ConsoleLoggerEventLoop {
    pub fn new() -> Self {
        ConsoleLoggerEventLoop {
            start_block: config::start_block(),
            sleep_between_ticks_ms: config::iteration_sleep_interval(),
            block_step: config::block_step(),
            topic_decoder: TopicDecoder::new(),
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
                event_loader.get_events(safe_address, next_block, Topic::ExecutionSuccess),
                event_loader.get_events(safe_address, next_block, Topic::ExecutionFailure),
                event_loader.get_events(safe_address, next_block, Topic::SafeMultisigTransaction),
            )?;

            let (
                execution_success_results,
                execution_failure_results,
                safe_multisig_transaction_results,
            ) = try_join!(
                process_transaction_logs(
                    &Topic::ExecutionSuccess,
                    &result_exec_success,
                    &self.topic_decoder,
                ),
                process_transaction_logs(
                    &Topic::ExecutionFailure,
                    &result_exec_failure,
                    &self.topic_decoder,
                ),
                process_transaction_logs(
                    &Topic::SafeMultisigTransaction,
                    &result_multisig_txs,
                    &self.topic_decoder
                )
            )?;

            let decoded_results = {
                let mut results = vec![];
                for result in execution_success_results {
                    results.push((Topic::ExecutionSuccess, result));
                }
                for result in execution_failure_results {
                    results.push((Topic::ExecutionFailure, result));
                }
                for result in safe_multisig_transaction_results {
                    results.push((Topic::SafeMultisigTransaction, result));
                }
                results
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
            log::info!("New transactions in this loop : {:#?}", decoded_results);
            log::info!("Sleeping for {} milliseconds", &self.sleep_between_ticks_ms);
            log::info!("========================================================================");
            sleep(Duration::from_millis(self.sleep_between_ticks_ms)).await;
            next_block += self.block_step;
        }
    }
}

async fn process_transaction_logs(
    topic: &Topic,
    tx_logs: &Vec<RpcTransactionLog>,
    topic_decoder: &TopicDecoder,
) -> anyhow::Result<Vec<TopicDecodedParams>> {
    let output = {
        let mut output = vec![];
        for tx_log in tx_logs {
            let decoder_input = TopicDecoderInput {
                topic: topic.clone(),
                data: tx_log.data.to_string(),
            };
            let decoded_output = topic_decoder.decode(decoder_input).await?;
            output.push(decoded_output);
        }
        output
    };
    Ok(output)
}
