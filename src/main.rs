use futures_util::StreamExt;
use helius_laserstream::{
    LaserstreamConfig,
    grpc::{SubscribeRequest, SubscribeRequestFilterTransactions, subscribe_update::UpdateOneof},
    subscribe,
};
use pumpfun_laserstream_sniper::{
    LASER_ENDPOINT, LASER_TOKEN_KEY, RPC_ENDPOINT, buy_token_handler, recent_blockhash_handler,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::collections::HashMap;
use std::sync::Arc;

pub const MINT_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let config = LaserstreamConfig {
        api_key: LASER_TOKEN_KEY.to_string(),
        endpoint: LASER_ENDPOINT.parse()?,
        ..Default::default()
    };

    tokio::spawn({
        let rpc_client = RpcClient::new_with_commitment(
            (RPC_ENDPOINT.to_string()).clone(),
            CommitmentConfig::processed(),
        );

        let rpc_client = Arc::new(rpc_client);
        async move {
            loop {
                recent_blockhash_handler(rpc_client.clone()).await;
            }
        }
    });

    let mut token_transactions_filter = HashMap::new();
    token_transactions_filter.insert(
        "client".to_string(),
        SubscribeRequestFilterTransactions {
            account_include: vec!["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string()],
            vote: Some(false),
            failed: Some(false),
            ..Default::default()
        },
    );

    let request = SubscribeRequest {
        transactions: token_transactions_filter,
        ..Default::default()
    };

    println!("Connecting and subscribing...");
    let stream = subscribe(config, request);
    futures::pin_mut!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                if let Some(UpdateOneof::Transaction(tx)) = update.update_oneof {
                    let message = tx
                        .transaction
                        .as_ref()
                        .and_then(|tx_detail| tx_detail.transaction.as_ref())
                        .and_then(|inner| inner.message.as_ref());

                        //
                        //
                        //  Paring part here
                        //
                        //
                    
                        println!("Mint: {:?}", mint_pubkey);
                        println!("Payer: {:?}", payer_pubkey);

                        let _ = buy_token_handler(
                            10000,
                            1000000,
                            &payer_pubkey.to_string(),
                            &mint_pubkey.to_string(),
                        )
                        .await;
                        // You can now use mint_pubkey and payer_pubkey freely here
                    }
                }
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
            }
        }
    }

    println!("Stream finished.");
    Ok(())
}
