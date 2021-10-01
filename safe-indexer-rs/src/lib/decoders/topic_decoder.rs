pub struct TopicDecoder;

impl EthDataDecoder for TopicDecoder {}

pub enum TopicDecodedParams {
    ExecutionSuccess { pub tx_hash: String },
    ExecutionFailure { pub tx_hash: String },
    SafeMultisigTransaction,
}
