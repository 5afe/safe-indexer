use crate::decoders::EthDataDecoder;
use crate::rpc::models::Topic;
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
        Ok(match input.topic {
            Topic::IncomingEth => TopicDecodedOutput::Unknown,
            Topic::ExecutionSuccess => TopicDecodedOutput::ExecutionSuccess {
                safe_tx_hash: String::from(&input.data.as_str()[..66]),
            },
            Topic::ExecutionFailure => TopicDecodedOutput::ExecutionFailure {
                safe_tx_hash: String::from(&input.data.as_str()[..66]),
            },
            Topic::SafeMultisigTransaction => TopicDecodedOutput::SafeMultisigTransaction {
                to: String::from(&input.data.as_str()[26..66]),
                value: String::from(&input.data.as_str()[41..48]),
                data: String::from(&input.data.as_str()[..40]),
                operation: String::from(&input.data.as_str()[..40]),
                safe_tx_gas: String::from(&input.data.as_str()[..40]),
                base_gas: String::from(&input.data.as_str()[..40]),
                gas_price: String::from(&input.data.as_str()[..40]),
                gas_token: "".to_string(),
                refund_receiver: "".to_string(),
                signatures: "".to_string(),
                additional_info: String::from(&input.data.as_str()[100..input.data.len()]),
            },
        })
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
