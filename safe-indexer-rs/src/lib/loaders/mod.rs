pub mod in_mem_loader;

use async_trait::async_trait;

//TODO there can be a few implementations: redis, postgres, in mem
#[async_trait]
pub trait EventLoader {
    async fn was_tx_hash_checked(self, tx_hash: &str) -> bool;

    async fn process_tx_hash(self, tx_hash: &str) -> anyhow::Result<()>;
}
