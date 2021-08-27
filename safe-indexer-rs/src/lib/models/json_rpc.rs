use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Vec<RequestParam>,
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
#[serde(rename_all = "lowercase")]
pub enum BlockNumber {
    Value(String),
    Earliest,
    Latest,
    Pending,
}
