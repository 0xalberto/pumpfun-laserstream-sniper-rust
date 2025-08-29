use dotenvy::dotenv;
use once_cell::sync::Lazy;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};
use std::env;

pub static PRIVATE_KEY: Lazy<Keypair> = Lazy::new(|| {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let payer: Keypair = Keypair::from_base58_string(private_key.as_str());

    payer
});

pub static PUBLIC_KEY: Lazy<Pubkey> = Lazy::new(|| {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let payer: Keypair = Keypair::from_base58_string(private_key.as_str());

    payer.pubkey()
});

pub static CONFIRM_SERVICE: Lazy<String> =
    Lazy::new(|| env::var("CONFIRM_SERVICE").expect("CONFIRM_SERVICE must be set"));

pub static LASER_ENDPOINT: Lazy<String> =
    Lazy::new(|| env::var("LASER_ENDPOINT").expect("LASER_ENDPOINT must be set"));

pub static LASER_TOKEN_KEY: Lazy<String> =
    Lazy::new(|| env::var("LASER_TOKEN_KEY").expect("LASER_TOKEN_KEY must be set"));

pub static RPC_ENDPOINT: Lazy<String> =
    Lazy::new(|| env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set"));

pub static PRIORITY_FEE: Lazy<(u64, u64, f64)> = Lazy::new(|| {
    dotenv().ok();

    let cu = env::var("CU")
        .ok()
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0); // fallback if missing or invalid

    let priority_fee_micro_lamport = env::var("PRIORITY_FEE_MICRO_LAMPORT")
        .ok()
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0); // fallback if missing or invalid

    let third_party_fee = env::var("THIRD_PARTY_FEE")
        .ok()
        .and_then(|val| val.parse::<f64>().ok())
        .unwrap_or(0.0); // fallback if missing or invalid

    (cu, priority_fee_micro_lamport, third_party_fee)
});

pub static RELAYER_KEY: Lazy<(String)> = Lazy::new(|| {
    dotenv().ok();

    let nozomi_key = env::var("NOZOMI_API_KEY").expect("NOZOMI_API_KEY must be set");

    (nozomi_key)
});
