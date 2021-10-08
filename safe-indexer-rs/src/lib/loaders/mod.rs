use crate::rpc::models::{RpcTransaction, RpcTransactionLog, Topic};
use async_trait::async_trait;

pub mod default_event_looper;
pub mod in_mem_loader;

#[async_trait]
pub trait EventLoader {
    async fn get_events(
        &self,
        safe_address: &str,
        from: u64,
        topic: Topic,
    ) -> anyhow::Result<Vec<RpcTransactionLog>>;

    async fn was_tx_hash_checked(&self, tx_hash: &str) -> bool;

    async fn process_tx_hash(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction>;

    async fn last_available_block(&self) -> anyhow::Result<u64>;
}

#[async_trait]
pub trait EventLooper {
    async fn start(
        &self,
        safe_address: &str,
        event_loader: &(impl EventLoader + Sync),
    ) -> anyhow::Result<()>;
}
