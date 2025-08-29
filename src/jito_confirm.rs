use serde_json::json;

use crate::JITO_CLIENT;

#[derive(Debug)]
pub enum JitoRegion {
    Mainnet,
    Amsterdam,
    Frankfurt,
    NY,
    Tokyo,
}

impl JitoRegion {
    pub fn base_url(&self) -> &'static str {
        match self {
            JitoRegion::Mainnet => "https://mainnet.block-engine.jito.wtf",
            JitoRegion::Amsterdam => "https://amsterdam.mainnet.block-engine.jito.wtf",
            JitoRegion::Frankfurt => "https://frankfurt.mainnet.block-engine.jito.wtf",
            JitoRegion::NY => "https://ny.mainnet.block-engine.jito.wtf",
            JitoRegion::Tokyo => "https://tokyo.mainnet.block-engine.jito.wtf",
        }
    }

    pub fn tx_url(&self) -> String {
        format!("{}/api/v1/transactions", self.base_url())
    }
}

pub async fn send_tx_using_jito(
    encoded_tx: &str,
    region: &JitoRegion,
) -> anyhow::Result<serde_json::Value> {
    let tx_url = region.tx_url();

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": [encoded_tx , {
            "encoding": "base64"
          }],

    });

    let response = JITO_CLIENT
        .post(&format!("{}?bundleOnly=false", tx_url))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    Ok(json)
}