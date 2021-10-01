use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Vec<RequestParam>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RequestParam {
    Single(String),
    #[serde(rename_all = "camelCase")]
    Log {
        from_block: Option<BlockNumber>,
        address: String,
        topics: Vec<Vec<String>>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionLog {
    pub address: String,
    pub block_hash: String,
    pub block_number: String,
    pub data: String,
    pub log_index: String,
    pub removed: bool,
    pub topics: Vec<String>,
    pub transaction_hash: String,
    pub transaction_index: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    pub to: String, // is the safe itself ?
    pub block_hash: String,
    pub block_number: String,
    pub chain_id: String,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String, // data field, is this an `execTransaction` call, check 4 bytes
    pub nonce: String,
    pub transaction_index: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum BlockNumber {
    Value(String),
    Earliest,
    Latest,
    Pending,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Topic {
    IncomingEth,
    ExecutionSuccess,
    ExecutionFailure,
    SafeMultisigTransaction,
}
