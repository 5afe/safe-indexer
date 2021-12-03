use dotenv::dotenv;

use commons::loaders::default_event_looper::ConsoleLoggerEventLoop;
use commons::loaders::in_mem_loader::InMemLoader;
use commons::loaders::EventLooper;
use commons::rpc::client::RpcClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let safe_address = "0x26A7ECdB60D38b06FffebA426713AA191CFFC2eD";
    let in_memory_loader = InMemLoader::new(RpcClient::new(reqwest::Client::new()));
    let event_console_looper = ConsoleLoggerEventLoop::new();

    event_console_looper
        .start(safe_address, &in_memory_loader)
        .await
}
