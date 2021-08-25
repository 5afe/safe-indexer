extern crate celery;
extern crate celery_codegen;
extern crate log;
extern crate dotenv;

use celery::prelude::*;
use anyhow::Result;
use dotenv::dotenv;

mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let my_app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()) },
        tasks = [
            tasks::add::add,
            tasks::logs::check_incoming_eth
        ],
        task_routes = [
            "*" => "celery"
        ],
    ).await?;

    my_app.display_pretty().await;
    my_app.consume_from(&["celery"]).await?;
    my_app.close().await?;
    Ok(())
}
