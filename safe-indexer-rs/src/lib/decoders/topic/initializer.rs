use crate::utils::number_utils::keccak256_str;

use super::models::{Topic, TopicArgument, TopicMetadata, TopicSignature};
use lazy_static;
use std::str::FromStr;

lazy_static! {
    pub static ref INCOMING_ETH_TOPIC: TopicMetadata = TopicMetadata {
        topic: Topic::IncomingEth,
        digest: Topic::IncomingEth.calculate_hash(),
        signature: TopicSignature::from_str(INCOMING_ETH)
            .expect("SafeReceived event parsing failure")
    };
    pub static ref EXECUTION_SUCCESS_TOPIC: TopicMetadata = TopicMetadata {
        topic: Topic::ExecutionSuccess,
        digest: Topic::ExecutionSuccess.calculate_hash(),
        signature: TopicSignature::from_str(EXECUTION_SUCCESS)
            .expect("ExecutionSuccess event parsing failure")
    };
    pub static ref EXECUTION_FAILURE_TOPIC: TopicMetadata = TopicMetadata {
        topic: Topic::ExecutionFailure,
        digest: Topic::ExecutionFailure.calculate_hash(),
        signature: TopicSignature::from_str(EXECUTION_FAILURE)
            .expect("ExecutionFailure event parsing failure")
    };
    pub static ref SAFE_MULTISIG_TRANSACTION_TOPIC: TopicMetadata = TopicMetadata {
        topic: Topic::SafeMultisigTransaction,
        digest: Topic::SafeMultisigTransaction.calculate_hash(),
        signature: TopicSignature::from_str(SAFE_MULTISIG_TRANSACTION)
            .expect("SafeMultiSigTransaction event parsing failure")
    };
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct TopicParseError;

const INCOMING_ETH: &str = "SafeReceived(address,uint256)";
const EXECUTION_SUCCESS: &str = "ExecutionSuccess(bytes32,uint256)";
const EXECUTION_FAILURE: &str = "ExecutionFailure(bytes32,uint256)";
const SAFE_MULTISIG_TRANSACTION: &str = "SafeMultiSigTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,bytes)";

impl Topic {
    pub fn get_hash(&self) -> String {
        String::from(match self {
            Topic::IncomingEth => &INCOMING_ETH_TOPIC.digest,
            Topic::ExecutionSuccess => &EXECUTION_SUCCESS_TOPIC.digest,
            Topic::ExecutionFailure => &EXECUTION_FAILURE_TOPIC.digest,
            Topic::SafeMultisigTransaction => &SAFE_MULTISIG_TRANSACTION_TOPIC.digest,
        })
    }

    fn calculate_hash(&self) -> String {
        match self {
            Topic::IncomingEth => keccak256_str(INCOMING_ETH),
            Topic::ExecutionSuccess => keccak256_str(EXECUTION_SUCCESS),
            Topic::ExecutionFailure => keccak256_str(EXECUTION_FAILURE),
            Topic::SafeMultisigTransaction => keccak256_str(SAFE_MULTISIG_TRANSACTION),
        }
    }
}

impl FromStr for TopicSignature {
    type Err = TopicParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let signature_parts: Vec<&str> = input.split(&['(', ',', ')'][..]).collect();
        let topic = Topic::from_str(signature_parts[0])?;
        let arguments = {
            let mut arguments = Vec::new();
            for canonical_type in signature_parts[1..].iter() {
                if canonical_type.trim().is_empty() {
                    continue;
                }
                arguments.push(TopicArgument::from_str(canonical_type)?);
            }
            arguments
        };

        Ok(TopicSignature { topic, arguments })
    }
}

impl FromStr for Topic {
    type Err = TopicParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "SafeReceived" => Ok(Topic::IncomingEth),
            "ExecutionSuccess" => Ok(Topic::ExecutionSuccess),
            "ExecutionFailure" => Ok(Topic::ExecutionFailure),
            "SafeMultiSigTransaction" => Ok(Topic::SafeMultisigTransaction),
            _ => Err(TopicParseError),
        }
    }
}

impl FromStr for TopicArgument {
    type Err = TopicParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "address" => Ok(TopicArgument::Address),
            "uint8" => Ok(TopicArgument::Uint8),
            "uint256" => Ok(TopicArgument::Uint256),
            "bytes" => Ok(TopicArgument::Bytes),
            "bytes32" => Ok(TopicArgument::Bytes32),
            _ => Err(TopicParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::decoders::topic::{
        initializer::{EXECUTION_FAILURE, EXECUTION_SUCCESS, SAFE_MULTISIG_TRANSACTION},
        models::{Topic, TopicArgument, TopicSignature},
    };

    #[test]
    fn execution_success() {
        let expected = Ok(TopicSignature {
            topic: Topic::ExecutionSuccess,
            arguments: vec![TopicArgument::Bytes32, TopicArgument::Uint256],
        });

        let actual = TopicSignature::from_str(EXECUTION_SUCCESS);

        assert_eq!(expected, actual);
    }

    #[test]
    fn execution_failure() {
        let expected = Ok(TopicSignature {
            topic: Topic::ExecutionFailure,
            arguments: vec![TopicArgument::Bytes32, TopicArgument::Uint256],
        });

        let actual = TopicSignature::from_str(EXECUTION_FAILURE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn safe_multisig_transaction() {
        let expected = Ok(TopicSignature {
            topic: Topic::SafeMultisigTransaction,
            arguments: vec![
                TopicArgument::Address,
                TopicArgument::Uint256,
                TopicArgument::Bytes,
                TopicArgument::Uint8,
                TopicArgument::Uint256,
                TopicArgument::Uint256,
                TopicArgument::Uint256,
                TopicArgument::Address,
                TopicArgument::Address,
                TopicArgument::Bytes,
                TopicArgument::Bytes,
            ],
        });

        let actual = TopicSignature::from_str(SAFE_MULTISIG_TRANSACTION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn incoming_eth_topic_digest() {
        let expected =
            String::from("0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d");
        let actual = Topic::IncomingEth.get_hash();
        assert_eq!(expected, actual);
    }

    #[test]
    fn execusion_success_topic_digest() {
        let expected =
            String::from("0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e");
        let actual = Topic::ExecutionSuccess.get_hash();
        assert_eq!(expected, actual);
    }

    #[test]
    fn execusion_failure_topic_digest() {
        let expected =
            String::from("0x23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23");
        let actual = Topic::ExecutionFailure.get_hash();
        assert_eq!(expected, actual);
    }

    #[test]
    fn safe_multisig_transaction_topic_digest() {
        let expected =
            String::from("0x66753cd2356569ee081232e3be8909b950e0a76c1f8460c3a5e3c2be32b11bed");
        let actual = Topic::SafeMultisigTransaction.get_hash();
        assert_eq!(expected, actual);
    }
}
