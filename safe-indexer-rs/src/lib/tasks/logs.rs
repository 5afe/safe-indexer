use crate::config;
use celery::prelude::*;
use crate::rpc::models::{RpcRequest, RpcResponse, RpcTransaction};

// time_limit = 10 can be set to timeout the task
#[celery::task(
on_failure = incoming_eth_log_failure,
on_success = incoming_eth_log_success,
)]
pub async fn check_incoming_eth(safe_address: String) -> TaskResult<Vec<String>> {
    let client = reqwest::Client::new();
    let request = RpcRequest::build_incoming_eth(&safe_address);

    let response = client.post(config::node_uri())
        .json(&request)
        .send()
        .await.expect("request failed")
        .text()
        .await.expect("response failed");

    let rpc_response = serde_json::from_str::<RpcResponse<Vec<RpcTransaction>>>(&response).expect("Result deserialize failed");
    Ok(rpc_response.result.iter().map(|result| result.transaction_hash.to_string()).collect())
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
