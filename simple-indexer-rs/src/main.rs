extern crate celery;
extern crate celery_codegen;

use celery::prelude::*;
use anyhow::Result;

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

#[tokio::main]
async fn main() -> Result<()> {
    let my_app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()) },
        tasks = [add],
        task_routes = [
            "*" => "celery"
        ],
    ).await?;

    my_app.display_pretty().await;
    my_app.consume_from(&["celery"]).await?;
    my_app.close().await?;
    Ok(())
}
