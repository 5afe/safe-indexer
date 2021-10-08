use crate::decoders::EthDataDecoder;
use crate::rpc::models::Topic;
use async_trait::async_trait;

pub struct TopicDecoder;

#[async_trait]
impl EthDataDecoder for TopicDecoder {
    type DecodedOutput = TopicDecodedParams;
    type DecoderInput = TopicDecoderInput;

    async fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput> {
        if !self.can_decode(&input) {
            anyhow::bail!("Can't decode input");
        }
        Ok(match input.topic {
            Topic::IncomingEth => TopicDecodedParams::Unknown,
            Topic::ExecutionSuccess => TopicDecodedParams::Unknown,
            Topic::ExecutionFailure => TopicDecodedParams::Unknown,
            Topic::SafeMultisigTransaction => TopicDecodedParams::Unknown,
        })
    }

    fn can_decode(&self, data: &Self::DecoderInput) -> bool {
        match data.topic {
            Topic::ExecutionSuccess | Topic::ExecutionFailure => true,
            Topic::SafeMultisigTransaction | Topic::IncomingEth => false,
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
    Unknown,
}
