pub mod http_decoder;
pub mod models;

use async_trait::async_trait;
use crate::decoder::models::DataDecoded;

#[async_trait]
pub trait EthDataDecoder {
    async fn decode(&self, data: &str) -> anyhow::Result<DataDecoded>;
}
