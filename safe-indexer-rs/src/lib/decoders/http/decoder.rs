use crate::decoders::http::models::{DataDecoded, HttpDecoderInput};
use crate::decoders::EthDataDecoder;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct HttpDataDecoder {
    http_client: reqwest::Client,
}

impl HttpDataDecoder {
    pub fn new() -> Self {
        HttpDataDecoder {
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl EthDataDecoder for HttpDataDecoder {
    type DecodedOutput = DataDecoded;
    type DecoderInput = HttpDecoderInput;

    async fn decode(&self, input: HttpDecoderInput) -> anyhow::Result<Self::DecodedOutput> {
        let url = format!(
            "https://safe-client.gnosis.io/v1/chains/{}/data-decoder",
            input.chain_id
        );
        let mut params = HashMap::new();

        params.insert("data", &input.data);
        let response = self.http_client.post(url).json(&params).send().await?;

        Ok(serde_json::from_str::<DataDecoded>(
            &response.text().await?,
        )?)
    }

    fn can_decode(&self, data: &HttpDecoderInput) -> bool {
        !data.data.is_empty() && data.data != "0x"
    }
}
