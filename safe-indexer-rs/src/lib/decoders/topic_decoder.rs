use crate::decoders::EthDataDecoder;

pub struct TopicDecoder;

impl EthDataDecoder for TopicDecoder {
    type DecodedOutput = TopicDecodedParams;

    async fn decode(&self, chain_id: &str, data: &str) -> anyhow::Result<Self::DecodedOutput> {
        todo!()
    }

    fn can_decode(&self, data: &str) -> bool {
        !data.is_empty() && data.len() > 2
    }
}

pub enum TopicDecodedParams {
    ExecutionSuccess { tx_hash: String },
    ExecutionFailure { tx_hash: String },
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
