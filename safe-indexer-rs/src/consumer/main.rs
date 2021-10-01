extern crate celery;
extern crate celery_codegen;
extern crate dotenv;
extern crate log;

use anyhow::Result;
use celery::prelude::*;
use commons::{config, tasks};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let my_app = celery::app!(
        broker = RedisBroker { config::redis_uri() },
        tasks = [
            tasks::celery::tx_hashes_for_topic
        ],
        task_routes = [
            "*" => "celery"
        ],
    )
    .await?;

    my_app.display_pretty().await;
    my_app.consume_from(&["celery"]).await?;
    my_app.close().await?;
    Ok(())
}
