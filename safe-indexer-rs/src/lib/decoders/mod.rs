pub mod http_decoder;
pub mod models;

use async_trait::async_trait;
use crate::decoders::models::DataDecoded;

// Ideally we can swap out our services eventually in favour of eth-contracts-rs
// For now the quickest way to a have dataDecoded is to rely in safe infra
#[async_trait]
pub trait EthDataDecoder {
    async fn decode(&self, chain_id: &str, data: &str) -> anyhow::Result<DataDecoded>;

    fn can_decode(&self, data: &str) -> bool;
}
