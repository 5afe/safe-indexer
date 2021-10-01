pub mod http_decoder;
pub mod models;

use async_trait::async_trait;

// Ideally we can swap out our services eventually in favour of eth-contracts-rs
// For now the quickest way to a have dataDecoded is to rely in safe infra
#[async_trait]
pub trait EthDataDecoder {
    type DecodedOutput;

    async fn decode(&self, chain_id: &str, data: &str) -> anyhow::Result<Self::DecodedOutput>;

    fn can_decode(&self, data: &str) -> bool;
}

