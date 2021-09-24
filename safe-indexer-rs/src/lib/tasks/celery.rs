use crate::config;
use celery::prelude::*;
use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransaction, Topic, BlockNumber};
use anyhow::Result;

// time_limit = 10 can be set to timeout the task
#[celery::task(
on_failure = incoming_eth_log_failure,
on_success = incoming_eth_log_success,
)]
pub async fn tx_hashes_for_topic(safe_address: String, from: BlockNumber, topic: Topic) -> TaskResult<Vec<String>> {
    Ok(super::impls::tx_hashes_for_topic(&safe_address, from, topic).await.map_err(|error| TaskError::ExpectedError(error.to_string()))?)
}

async fn incoming_eth_log_failure<T: Task>(task: &T, err: &TaskError) {
    match err {
        TaskError::TimeoutError => log::error!("Oops! Task {} timed out!", task.name()),
        _ => log::error!("Task {} failed with {:?}", task.name(), err),
    };
}

async fn incoming_eth_log_success<T: Task>(_task: &T, ret: &Vec<String>) {
    let result: &Vec<String> = ret;
    log::debug!("RETURN after type coercion {:#?}", result);
}
