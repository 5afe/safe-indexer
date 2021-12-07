use super::models::Topic;
use crate::decoders::EthDataDecoder;
use async_trait::async_trait;

pub struct TopicDecoder;

#[async_trait]
impl EthDataDecoder for TopicDecoder {
    type DecodedOutput = TopicDecodedOutput;
    type DecoderInput = TopicDecoderInput;

    async fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput> {
        if !self.can_decode(&input) {
            anyhow::bail!("Can't decode input");
        }
        Ok(input.topic.decode(input.data.into())?)
    }

    fn can_decode(&self, data: &Self::DecoderInput) -> bool {
        match data.topic {
            Topic::ExecutionSuccess | Topic::ExecutionFailure | Topic::SafeMultisigTransaction => {
                true
            }
            Topic::IncomingEth => false,
        }
    }
}

impl TopicDecoder {
    pub fn new() -> Self {
        TopicDecoder
    }
}

pub struct TopicDecoderInput {
    pub topic: Topic,
    pub data: String,
}

#[derive(Debug)]
pub enum TopicDecodedOutput {
    ExecutionSuccess {
        safe_tx_hash: String,
    },
    ExecutionFailure {
        safe_tx_hash: String,
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
    Unknown,
}
