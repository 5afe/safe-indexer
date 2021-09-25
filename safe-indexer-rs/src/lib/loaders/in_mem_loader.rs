use tokio::sync::Mutex;
use crate::rpc::models::RpcTransaction;
use crate::loaders::EventLoader;
use std::collections::HashMap;
use crate::rpc::client::RpcClient;
use async_trait::async_trait;

pub struct InMemLoader {
    rpc_client: RpcClient,
    database: Mutex<HashMap<String, RpcTransaction>>,
}

impl InMemLoader {
    pub fn new(rpc_client: RpcClient) -> Self {
        InMemLoader { rpc_client, database: Default::default() }
    }
}

#[async_trait]
impl EventLoader for InMemLoader {
    async fn was_tx_hash_checked(&self, tx_hash: &str) -> bool {
        self.database.lock().await.contains_key(tx_hash)
    }

    async fn process_tx_hash(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction> {
        let transaction = self.rpc_client.get_transaction(tx_hash).await?;
        let mut database = self.database.lock().await;
        database.insert(tx_hash.to_string(), transaction.clone());
        Ok(transaction)
    }
}
