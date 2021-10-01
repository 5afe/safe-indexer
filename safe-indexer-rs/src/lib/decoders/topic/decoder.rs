use crate::decoders::EthDataDecoder;
use async_trait::async_trait;

pub struct TopicDecoder;

#[async_trait]
impl EthDataDecoder for TopicDecoder {
    type DecodedOutput = TopicDecodedParams;
    type DecoderInput = TopicDecoderInput;

    async fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput> {
        todo!()
    }

    fn can_decode(&self, data: &Self::DecoderInput) -> bool {
        todo!()
    }
}

pub struct TopicDecoderInput {
    pub topic: String,
    pub data: String,
}

pub enum TopicDecodedParams {
    ExecutionSuccess {
        tx_hash: String,
    },
    ExecutionFailure {
        tx_hash: String,
    },
    SafeMultisigTransaction {
        to: String,
        value: String,
        data: String,
        operation: String,
        safe_tx_gas: String,
        base_gas: String,
        gas_price: String,
        gas_token: String,
        refund_receiver: String,
        signatures: String,
        additional_info: String,
    },
}
