use serde::{Deserialize, Serialize};

pub struct TopicMetadata {
    pub topic: Topic,
    pub digest: String,
    pub signature: TopicSignature,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Topic {
    IncomingEth,
    ExecutionSuccess,
    ExecutionFailure,
    SafeMultisigTransaction,
}

// Adding more types as we need them for new Topics
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TopicArgument {
    Address,
    Uint8,
    Uint256,
    Bytes,
    Bytes32,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct TopicSignature {
    pub topic: Topic,
    pub arguments: Vec<TopicArgument>,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct DataChunks(Vec<String>);

impl DataChunks {
    pub fn new(chunks: Vec<String>) -> Self {
        DataChunks(chunks)
    }

    pub fn get(&self, index: usize) -> &str {
        &self.0[index]
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0.as_slice()
    }
}

pub const WORD_LENGTH: usize = 64; // data in hex -> 2x single hex char == 1 byte

impl From<String> for DataChunks {
    fn from(raw_input: String) -> Self {
        let input = if raw_input.starts_with("0x") {
            raw_input[2..].to_string()
        } else {
            raw_input
        };

        if input.len() < WORD_LENGTH {
            panic!("Data field not chunky enough");
        }
        let chunks = input
            .chars()
            .collect::<Vec<char>>()
            .chunks(WORD_LENGTH)
            .map(|word| word.iter().collect::<String>())
            .collect::<Vec<String>>();
        DataChunks(chunks)
    }
}

#[cfg(test)]
mod tests {
    use crate::decoders::topic::models::{DataChunks, WORD_LENGTH};

    #[test]
    fn happy_path_chunk() {
        let input = String::from(
            "\
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
            0000000000000000000000000000000000000000000000000000000000000001",
        );
        let actual_chunks: DataChunks = input.into();
        let expected_chunks = DataChunks::new(vec![
            "00000000000000000000000026a7ecdb60d38b06fffeba426713aa191cffc2ed".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000160".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000011ef3".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "00000000000000000000000000000000000000000000000000000000000001e0".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000260".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000044".to_string(),
            "0d582f13000000000000000000000000be8c10dbf4c6148f9834c56c3331f819".to_string(),
            "1f35555200000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000100000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000041".to_string(),
            "0000000000000000000000004d3101d77aac1b90ae42efa38d235a81af270d40".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0100000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000060".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            "0000000000000000000000004d3101d77aac1b90ae42efa38d235a81af270d40".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        ]);
        assert_eq!(actual_chunks, expected_chunks);
    }

    #[test]
    #[should_panic]
    fn too_short_chunk() {
        let input = String::from("0x01");
        DataChunks::from(input);
    }

    #[test]
    #[should_panic]
    fn empty_input() {
        let input = String::new();
        DataChunks::from(input);
    }

    #[test]
    fn strip_prefix_single_chunk() {
        let input =
            String::from("0x0000000000000000000000005592ec0cfb4dbc12d3ab100b257153436a1f0fea");
        let actual_chunks: DataChunks = input.into();
        let expected_chunks = DataChunks::new(vec![String::from(
            "0000000000000000000000005592ec0cfb4dbc12d3ab100b257153436a1f0fea",
        )]);
        assert_eq!(actual_chunks, expected_chunks);
    }

    #[test]
    fn word_length_is_64() {
        assert_eq!(WORD_LENGTH, 64);
    }
}
