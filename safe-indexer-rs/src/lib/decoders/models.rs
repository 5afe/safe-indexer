use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::utils::json;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Operation {
    CALL = 0,
    DELEGATE = 1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataDecoded {
    pub method: String,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub value: ParamValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "json::try_deserialize")]
    #[serde(default)]
    pub value_decoded: Option<ValueDecodedType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParamValue {
    SingleValue(String),
    ArrayValue(Vec<ParamValue>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ValueDecodedType {
    InternalTransaction(Vec<InternalTransaction>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransaction {
    pub operation: Operation,
    pub to: String,
    pub value: Option<String>,
    pub data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
}
