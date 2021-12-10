use crate::utils::number_utils::from_hex_string;

use super::{
    decoder::TopicDecodedOutput,
    initializer::{
        EXECUTION_FAILURE_TOPIC, EXECUTION_SUCCESS_TOPIC, SAFE_MULTISIG_TRANSACTION_TOPIC,
    },
    models::{DataChunks, Topic, TopicArgument},
};

impl Topic {
    pub fn decode(&self, data_chunks: DataChunks) -> anyhow::Result<TopicDecodedOutput> {
        // let chunks = data_chunks.0;
        Ok(match self {
            Topic::IncomingEth => TopicDecodedOutput::Unknown,
            Topic::ExecutionSuccess => TopicDecodedOutput::ExecutionSuccess {
                safe_tx_hash: EXECUTION_SUCCESS_TOPIC.signature.arguments[0].parse(0, &data_chunks),
            },
            Topic::ExecutionFailure => TopicDecodedOutput::ExecutionFailure {
                safe_tx_hash: EXECUTION_FAILURE_TOPIC.signature.arguments[0].parse(0, &data_chunks),
            },
            Topic::SafeMultisigTransaction => TopicDecodedOutput::SafeMultisigTransaction {
                to: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[0].parse(0, &data_chunks),
                value: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[1]
                    .parse(1, &data_chunks),
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

impl DataChunks {
    fn value_of_dyn_type(&self, start_index: usize) -> String {
        let offset = from_hex_string(self.get(start_index)).expect(&format!(
            "Dynamic type at index {} has unexpected offset",
            start_index
        ));

        offset.to_string()
    }
}

impl TopicArgument {
    fn parse(&self, index: usize, chunks: &DataChunks) -> String {
        match self {
            TopicArgument::Address => chunks.as_slice()[index][24..64].to_string(),
            TopicArgument::Uint8 => chunks.as_slice()[index][56..64].to_string(),
            TopicArgument::Uint256 | TopicArgument::Bytes32 => chunks.as_slice()[index].to_owned(),
            TopicArgument::Bytes => chunks.value_of_dyn_type(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_type() {
        let topic_argument = TopicArgument::Address;
        let chunks = DataChunks::new(vec![
            "00000000000000000000000026a7ecdb60d38b06fffeba426713aa191cffc2ed".to_string(),
        ]);
        let expected = "26a7ecdb60d38b06fffeba426713aa191cffc2ed";

        let actual = topic_argument.parse(0, &chunks);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dynamic_type() {
        let topic_argument = TopicArgument::Address;
        let chunks = DataChunks::new(vec![
            "00000000000000000000000026a7ecdb60d38b06fffeba426713aa191cffc2ed".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000160".to_string(), // offset of bytes
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000011ef3".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "00000000000000000000000000000000000000000000000000000000000001e0".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000260".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000044".to_string(), // size of bytes
            "0d582f13000000000000000000000000be8c10dbf4c6148f9834c56c3331f819".to_string(),
            "1f35555200000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000100000000000000000000000000000000000000000000000000000000".to_string(),
        ]);
        let expected = "0d582f13000000000000000000000000be8c10dbf4c6148f9834c56c3331f819\
        1f35555200000000000000000000000000000000000000000000000000000000\
        00000001";

        let actual = topic_argument.parse(2, &chunks);
        assert_eq!(actual, expected);
    }

    #[test]
    fn full_tx_log_parsing() {}
}
