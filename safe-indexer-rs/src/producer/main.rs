use celery::prelude::*;
use anyhow::Result;

use commons::{config, tasks};

#[tokio::main]
async fn main()  -> Result<()> {
    let my_app = celery::app!(
        broker = RedisBroker { config::redis_uri() },
        tasks = [
            tasks::add::add
        ],
        task_routes = [
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    ).await?;

    Ok(())
}