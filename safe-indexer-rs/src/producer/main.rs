use anyhow::Result;
use celery::beat::DeltaSchedule;
use celery::prelude::*;
use commons::{config, decoders::topic::models::Topic, tasks};
use dotenv::dotenv;
use tokio::time::Duration;

// Tasks on the producer side don't need to know the implementation of what the consumer
// runs, only the method signature. Therefore, methods should be declared but the body
// can be left as `unimplemented!()
// Reference: https://github.com/rusty-celery/rusty-celery/blob/41efda696132bff7451b191d185a48129ca1b2e3/examples/beat_app.rs#L13

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // TODO, this always checks the same BlockNumber, there needs to be a way to move forward the blockNumber
    let mut beat = celery::beat!(
        broker = RedisBroker { config::redis_uri() },
        tasks = [
            "INCOMING ETH" => {
                tasks::celery::tx_hashes_for_topic,
                schedule = DeltaSchedule::new(Duration::from_secs(15)),
                args = ("0x26A7ECdB60D38b06FffebA426713AA191CFFC2eD".to_string(), config::start_block(), Topic::IncomingEth),
                },
            "EXECUTION SUCCESS" => {
                tasks::celery::tx_hashes_for_topic,
                schedule = DeltaSchedule::new(Duration::from_secs(15)),
                args = ("0x26A7ECdB60D38b06FffebA426713AA191CFFC2eD".to_string(), config::start_block(), Topic::ExecutionSuccess),
                },
            "EXECUTION FAILURE" => {
                tasks::celery::tx_hashes_for_topic,
                schedule = DeltaSchedule::new(Duration::from_secs(15)),
                args = ("0x26A7ECdB60D38b06FffebA426713AA191CFFC2eD".to_string(), config::start_block(), Topic::ExecutionFailure),
                }
        ],
        task_routes = [
            "*" => tasks::QUEUE_NAME,
        ],
    )
        .await?;

    beat.start().await?;
    Ok(())
}
