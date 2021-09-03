extern crate celery;
extern crate celery_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate reqwest;

use tokio::time::sleep;
use std::time::Duration;

pub mod config;
pub mod db;
pub mod tasks;
pub mod models;

#[tokio::main]
async fn main() {
    loop {
        sleep(Duration::from_millis(2000)).await;
        println!("2 seconds passed");
    }
}
