use celery::prelude::*;
use async_trait::async_trait;

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let my_app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_URI").unwrap_or_else(|_| "amqp://127.0.0.1:5672/my_vhost".into()) },
        tasks = [add],
        task_routes = [],
    ).await?;

    my_app.consume().await;
    Ok(())
}
