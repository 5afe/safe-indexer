use crate::loaders::EventLoader;
use crate::rpc::client::RpcClient;
use crate::rpc::models::{RpcTransaction, RpcTransactionLog, Topic};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct InMemLoader {
    rpc_client: RpcClient,
    database: Mutex<HashMap<String, RpcTransaction>>,
}

impl InMemLoader {
    pub fn new(rpc_client: RpcClient) -> Self {
        InMemLoader {
            rpc_client,
            database: Default::default(),
        }
    }
}

#[async_trait]
impl EventLoader for InMemLoader {
    async fn get_transaction_hashes_for_event(
        &self,
        safe_address: &str,
        from: u64,
        topic: Topic,
    ) -> anyhow::Result<Vec<String>> {
        self.rpc_client
            .get_transaction_hashes_for_event(safe_address, from, topic)
            .await
    }

    async fn get_events_data_for(
        &self,
        safe_address: &str,
        from: u64,
        topic: Topic,
    ) -> anyhow::Result<Vec<RpcTransactionLog>> {
        self.rpc_client
            .get_transaction_log(safe_address, from, topic)
            .await
    }

    async fn was_tx_hash_checked(&self, tx_hash: &str) -> bool {
        self.database.lock().await.contains_key(tx_hash)
    }

    async fn process_tx_hash(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction> {
        let transaction = self.rpc_client.get_transaction(tx_hash).await?;
        let mut database = self.database.lock().await;
        database.insert(tx_hash.to_string(), transaction.clone());
        Ok(transaction)
    }

    async fn last_available_block(&self) -> anyhow::Result<u64> {
        self.rpc_client.get_current_block().await
    }
}
