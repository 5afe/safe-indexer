use super::models::Topic;
use crate::decoders::EthDataDecoder;

pub struct TopicDecoder;

impl EthDataDecoder for TopicDecoder {
    type DecodedOutput = TopicDecodedOutput;
    type DecoderInput = TopicDecoderInput;

    fn decode(&self, input: Self::DecoderInput) -> anyhow::Result<Self::DecodedOutput> {
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
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_success() {
        let decoder = TopicDecoder::new();
        let decoder_input = TopicDecoderInput {
            topic: Topic::ExecutionSuccess,
            data: String::from("0xd2c64a47741878350c85ad6ad202800dc4ce3bb7cbdf9e60399bf71e6d4bd64c0000000000000000000000000000000000000000000000000000000000000000"),
        };
        let expected = TopicDecodedOutput::ExecutionSuccess {
            safe_tx_hash: String::from(
                "d2c64a47741878350c85ad6ad202800dc4ce3bb7cbdf9e60399bf71e6d4bd64c",
            ),
        };

        let actual = decoder.decode(decoder_input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn execution_failure() {
        let decoder = TopicDecoder::new();
        let decoder_input = TopicDecoderInput {
            topic: Topic::ExecutionFailure,
            data: String::from("0xd2c64a47741878350c85ad6ad202800dc4ce3bb7cbdf9e60399bf71e6d4bd64c0000000000000000000000000000000000000000000000000000000000000000"),
        };
        let expected = TopicDecodedOutput::ExecutionFailure {
            safe_tx_hash: String::from(
                "d2c64a47741878350c85ad6ad202800dc4ce3bb7cbdf9e60399bf71e6d4bd64c",
            ),
        };

        let actual = decoder.decode(decoder_input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn safe_multisig_transaction() {
        let decoder = TopicDecoder::new();
        let data = "\
        0x00000000000000000000000026a7ecdb60d38b06fffeba426713aa191cffc2ed\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000160\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000011ef3\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000001e0\
        0000000000000000000000000000000000000000000000000000000000000260\
        0000000000000000000000000000000000000000000000000000000000000044\
        0d582f13000000000000000000000000be8c10dbf4c6148f9834c56c3331f819\
        1f35555200000000000000000000000000000000000000000000000000000000\
        0000000100000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000041\
        0000000000000000000000004d3101d77aac1b90ae42efa38d235a81af270d40\
        0000000000000000000000000000000000000000000000000000000000000000\
        0100000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000060\
        0000000000000000000000000000000000000000000000000000000000000001\
        0000000000000000000000004d3101d77aac1b90ae42efa38d235a81af270d40\
        0000000000000000000000000000000000000000000000000000000000000001";
    }
}
