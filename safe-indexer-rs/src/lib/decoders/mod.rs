pub mod topic;

pub trait EthDataDecoder {
    type DecodedOutput;
    type DecoderInput;

    fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput>;

    fn can_decode(&self, data: &Self::DecoderInput) -> bool;
}
