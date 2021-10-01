pub mod http;
pub mod topic;

use async_trait::async_trait;

// Ideally we can swap out our services eventually in favour of eth-contracts-rs
// For now the quickest way to a have dataDecoded is to rely in safe infra
#[async_trait]
pub trait EthDataDecoder {
    type DecodedOutput;
    type DecoderInput;

    async fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput>;

    fn can_decode(&self, data: &Self::DecoderInput) -> bool;
}
