use crate::decoders::EthDataDecoder;
use crate::decoders::models::DataDecoded;
use std::collections::HashMap;
use async_trait::async_trait;

pub struct HttpDataDecoder {
    http_client: reqwest::Client,
}

impl HttpDataDecoder {
    pub fn new() -> Self {
        HttpDataDecoder { http_client: reqwest::Client::new() }
    }
}

#[async_trait]
impl EthDataDecoder for HttpDataDecoder {
    type DecodedOutput = DataDecoded; 

    async fn decode(&self, chain_id: &str, data: &str) -> anyhow::Result<Self::DecodedOutput> {
        let url = format!("https://safe-client.gnosis.io/v1/chains/{}/data-decoder", chain_id);
        let mut params = HashMap::new();

        params.insert("data", data);
        let response = self.http_client.post(url)
            .json(&params)
            .send().await?;

        Ok(serde_json::from_str::<DataDecoded>(
            &response.text().await?,
        )?)
    }

    fn can_decode(&self, data: &str) -> bool {
        !data.is_empty() && data != "0x"
    }
}
