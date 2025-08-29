use serde_json::json;

use crate::NOZOMI_CLIENT;

#[derive(Debug)]
pub enum NozomiRegion {
    NY,
	FRA,
}

impl NozomiRegion {
    pub fn base_url(&self) -> &'static str {
        match self {
            NozomiRegion::NY => "http://ewr1.secure.nozomi.temporal.xyz",
            NozomiRegion::FRA => "http://fra2.nozomi.temporal.xyz",
        }
    }

    pub fn tx_url(&self, auth_key: &str) -> String {
        format!("{}/?c={}", self.base_url(), auth_key)
    }
}

pub async fn submit_nozomi_tx(
    transaction_content: &str,
    region: &NozomiRegion,
    auth_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = region.tx_url(auth_key);

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": [transaction_content, {"encoding": "base64"}]
    });

    let response = NOZOMI_CLIENT.post(url).json(&payload).send().await?;

    let data: serde_json::Value = response.json().await?;

    Ok(data)
}