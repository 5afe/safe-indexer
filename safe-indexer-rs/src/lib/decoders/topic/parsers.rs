use crate::utils::number_utils::from_hex_string;

use super::{
    decoder::TopicDecodedOutput,
    initializer::{
        EXECUTION_FAILURE_TOPIC, EXECUTION_SUCCESS_TOPIC, SAFE_MULTISIG_TRANSACTION_TOPIC,
    },
    models::{DataChunks, Topic, TopicArgument, TopicMetadata},
};

impl Topic {
    pub fn decode(&self, data_chunks: DataChunks) -> anyhow::Result<TopicDecodedOutput> {
        let chunks = data_chunks.0;
        Ok(match self {
            Topic::IncomingEth => TopicDecodedOutput::Unknown,
            Topic::ExecutionSuccess => TopicDecodedOutput::ExecutionSuccess {
                safe_tx_hash: EXECUTION_SUCCESS_TOPIC.signature.arguments[0].from_chunk(&chunks[0]),
            },
            Topic::ExecutionFailure => TopicDecodedOutput::ExecutionFailure {
                safe_tx_hash: EXECUTION_FAILURE_TOPIC.signature.arguments[0].from_chunk(&chunks[0]),
            },
            Topic::SafeMultisigTransaction => TopicDecodedOutput::SafeMultisigTransaction {
                to: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[0].from_chunk(&chunks[0]),
                value: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[1]
                    .from_chunk(&chunks[1]),
                data: String::from("todo!()"),
                operation: String::from("todo!()"),
                safe_tx_gas: String::from("todo!()"),
                base_gas: String::from("todo!()"),
                gas_price: String::from("todo!()"),
                gas_token: String::from("todo!()"),
                refund_receiver: String::from("todo!()"),
                signatures: String::from("todo!()"),
                additional_info: String::from("todo!()"),
            },
        })
    }
}

impl TopicArgument {
    fn from_chunks(&self, chunk: &[&str]) -> String {
        match self {
            TopicArgument::Address => chunk[0][26..66].to_string(),
            TopicArgument::Uint8 => todo!(),
            TopicArgument::Uint256 => todo!(),
            TopicArgument::Bytes32 => todo!(),
            TopicArgument::Bytes => todo!(),
        }
    }

    fn from_chunk(&self, chunk: &str) -> String {
        match self {
            TopicArgument::Address => chunk[24..64].to_string(),
            TopicArgument::Uint8 => chunk[60..64].to_string(),
            TopicArgument::Uint256 => chunk.to_string(),
            TopicArgument::Bytes32 => chunk.to_string(),
            TopicArgument::Bytes => todo!(),
        }
    }

    fn required_chunks(&self, first_chunk: &str) -> usize {
        match self {
            TopicArgument::Address
            | TopicArgument::Uint8
            | TopicArgument::Uint256
            | TopicArgument::Bytes32 => 1,
            TopicArgument::Bytes => {
                from_hex_string(first_chunk).expect("Invalid hex number of chunks") as usize
            }
        }
    }
}
