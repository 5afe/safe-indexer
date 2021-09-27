extern crate celery;
extern crate celery_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate reqwest;

use dotenv::dotenv;
use crate::loaders::in_mem_loader::InMemLoader;
use crate::loaders::EventLooper;
use crate::loaders::default_event_looper::ConsoleLoggerEventLoop;
use crate::rpc::client::RpcClient;

pub mod config;
pub mod db;
pub mod loaders;
pub mod number_utils;
pub mod rpc;
pub mod tasks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let safe_address = "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67";
    let in_memory_loader = InMemLoader::new(RpcClient::new(reqwest::Client::new()));
    let event_console_looper = ConsoleLoggerEventLoop::new();

    event_console_looper.start(safe_address, &in_memory_loader).await
}
