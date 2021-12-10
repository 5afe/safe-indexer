use std::ops::Div;

use crate::utils::number_utils::from_hex_string;

use super::{
    decoder::TopicDecodedOutput,
    initializer::{
        EXECUTION_FAILURE_TOPIC, EXECUTION_SUCCESS_TOPIC, SAFE_MULTISIG_TRANSACTION_TOPIC,
    },
    models::{DataChunks, Topic, TopicArgument, WORD_LENGTH},
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
                data: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[2].parse(2, &data_chunks),
                operation: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[3]
                    .parse(3, &data_chunks),
                safe_tx_gas: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[4]
                    .parse(4, &data_chunks),
                base_gas: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[5]
                    .parse(5, &data_chunks),
                gas_price: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[6]
                    .parse(6, &data_chunks),
                gas_token: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[7]
                    .parse(7, &data_chunks),
                refund_receiver: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[8]
                    .parse(8, &data_chunks),
                signatures: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[9]
                    .parse(9, &data_chunks),
                additional_info: SAFE_MULTISIG_TRANSACTION_TOPIC.signature.arguments[10]
                    .parse(10, &data_chunks),
            },
        })
    }
}

impl DataChunks {
    fn value_of_dyn_type(&self, start_index: usize) -> String {
        let offset_bytes = from_hex_string(self.get(start_index)).expect(&format!(
            "Dynamic type at index {} has unexpected offset",
            start_index
        )) as usize;

        let word_length_bytes = (WORD_LENGTH / 2) as usize;
        let offset_index = offset_bytes / word_length_bytes;

        let value_size_hex = from_hex_string(self.get(offset_index)).expect(&format!(
            "Dynamic type at index {} has unexpected data size",
            start_index
        )) * 2; // we are interested in hex

        let mut current_data_index = offset_index + 1;
        let mut output = String::new();
        let data_chunks = value_size_hex.div(WORD_LENGTH as u64);
        let last_chunk_carry = value_size_hex % (WORD_LENGTH as u64);

        if data_chunks > 0 {
            for _index in 0..data_chunks {
                output.push_str(self.get(current_data_index));
                current_data_index += 1;
            }
        }
        if last_chunk_carry > 0 {
            output.push_str(&self.get(current_data_index)[0..(last_chunk_carry as usize)]);
        }
        output
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
        let topic_argument = TopicArgument::Bytes;
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
    fn full_tx_log_parsing() {
        let topic_argument = vec![
            TopicArgument::Address,
            TopicArgument::Uint256,
            TopicArgument::Bytes,
            // ignoring arguments after bytes. Index > 2
        ];
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

        let expected = [
            "26a7ecdb60d38b06fffeba426713aa191cffc2ed",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0d582f13000000000000000000000000be8c10dbf4c6148f9834c56c3331f819\
            1f35555200000000000000000000000000000000000000000000000000000000\
            00000001",
        ];

        for (index, topic_argument) in topic_argument.iter().enumerate() {
            let actual = topic_argument.parse(index, &chunks);
            assert_eq!(actual, expected[index]);
        }
    }
}
