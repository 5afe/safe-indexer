#[deprecated]
pub mod http;
pub mod topic;

use async_trait::async_trait;

#[async_trait]
pub trait EthDataDecoder {
    type DecodedOutput;
    type DecoderInput;

    async fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput>;

    fn can_decode(&self, data: &Self::DecoderInput) -> bool;
}
