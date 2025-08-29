use serde_json::json;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    message::{VersionedMessage, v0::Message},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signer::Signer,
    system_instruction, system_program,
    transaction::VersionedTransaction,
};
use solana_transaction_status_client_types::UiTransactionEncoding;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use crate::*;

pub async fn buy_token_handler(
    token_amount: u64,
    buy_amount: u64,
    token_creator: &str,
    token_mint_ca: &str,
) {
    let (cu, priority_fee_micro_lamport, third_party_fee) = *PRIORITY_FEE;
    let (nozomi_key) = &*RELAYER_KEY;
    let (fee_addr, third_party_fee) = match CONFIRM_SERVICE.as_str() {
        "JITO" => {
            let fee = third_party_fee.max(JITO_MIN_TIP);
            (Pubkey::from_str_const(JITO_TIP[0]), fee)
        }
        "NOZOMI" => {
            let fee = third_party_fee.max(NOZOMI_MIN_TIP);
            (Pubkey::from_str_const(NOZOMI_TIP[0]), fee)
        }
        _ => {
            let fee = third_party_fee.max(JITO_MIN_TIP);
            (Pubkey::from_str_const(JITO_TIP[0]), fee)
        }
    };

    let mut ixs: Vec<Instruction> = vec![];

    let set_cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu as u32);
    let set_cu_pf_ix = ComputeBudgetInstruction::set_compute_unit_price(priority_fee_micro_lamport);

    let transfer_3rd_fee_ix =
        system_instruction::transfer(&PUBLIC_KEY, &fee_addr, sol_to_lamports(third_party_fee));

    let token_creator = Pubkey::from_str_const(&token_creator);
    let pump_program_id = Pubkey::from_str_const(PUMP_FUN_PROGRAM);
    let mint_addr = Pubkey::from_str_const(&token_mint_ca);
    let pumpfun_global_acc = Pubkey::from_str_const(PUMP_FUN_GLOBAL);
    let pumpfun_fee_acc = Pubkey::from_str_const(PUMP_FUN_FEE_ACCOUNT);
    let (pumpfun_bonding_curve, _) = Pubkey::find_program_address(
        &[BONDING_CURVE_SEED, &mint_addr.to_bytes()],
        &pump_program_id,
    );
    let pumpfun_bonding_curve_ata =
        get_associated_token_address(&pumpfun_bonding_curve, &mint_addr);
    let event_authority = Pubkey::from_str_const(PUMPFUN_EVENT_AUTH);
    let payer_ata = get_associated_token_address(&PUBLIC_KEY, &mint_addr);

    let (rent, _) = Pubkey::find_program_address(
        &[PUMPFUN_CREATOR_VAULT, &token_creator.to_bytes()],
        &pump_program_id,
    );
    let associated_token_program = Pubkey::from_str_const(ASSOCIATED_TOKEN_PROGRAM_ID);
    let token_program = Pubkey::from_str_const(TOKEN_PROGRAM_ID);

    let create_ata_ix =
        create_associated_token_account(&PUBLIC_KEY, &PUBLIC_KEY, &mint_addr, &token_program);

    let buy_ix = pumpfun_buy_original_ix(
        pumpfun_global_acc,
        mint_addr,
        pumpfun_fee_acc,
        pumpfun_bonding_curve,
        pumpfun_bonding_curve_ata,
        event_authority,
        payer_ata,
        *PUBLIC_KEY,
        rent,
        pump_program_id,
        system_program::ID,
        token_program,
        token_amount,
        buy_amount,
    );

    ixs.insert(0, set_cu_ix);
    ixs.insert(1, set_cu_pf_ix);
    ixs.insert(2, create_ata_ix);
    ixs.insert(3, buy_ix);
    if CONFIRM_SERVICE.as_str() == "JITO" || CONFIRM_SERVICE.as_str() == "NOZOMI" {
        ixs.insert(4, transfer_3rd_fee_ix);
    }

    let recent_blockhash = get_slot();

    // let txn = Transaction::new_signed_with_payer(
    //     &ixs,
    //     Some(&PUBLIC_KEY),
    //     &[&PRIVATE_KEY],
    //     recent_blockhash,
    // );

    let message = Message::try_compile(&PUBLIC_KEY, &ixs, &[], recent_blockhash).unwrap();
    let versioned_message = VersionedMessage::V0(message);
    let txn = VersionedTransaction::try_new(versioned_message, &[&PRIVATE_KEY]).unwrap();

    // let rpc_client = RpcClient::new_with_commitment(
    //         ("http://quadratically-encounter-mkptthwaoy-dedicated-bypass.helius-rpc.com?api-key=79e0a224-045e-4e54-ab47-e184bb2446ef".to_string()).clone(),
    //         CommitmentConfig::processed(),
    //     );

    // let tx_log = rpc_client.simulate_transaction(&txn).await.unwrap();

    // println!("tx_log {:#?}" , tx_log);

    let serialized_tx = bincode::serialize(&txn).expect("Failed to serialize transaction");

    let encoded_tx = &bs64::encode(&serialized_tx);

    let results = match CONFIRM_SERVICE.as_str() {
        "JITO" => send_tx_using_jito(&encoded_tx, &JitoRegion::NY)
            .await
            .unwrap_or_default(),
        "NOZOMI" => submit_nozomi_tx(&encoded_tx, &NozomiRegion::FRA, &nozomi_key)
            .await
            .unwrap_or_default(),
        &_ => {
            json!({"result" : "error".to_string()})
        }
    };

    println!("{:#?} ", results);

    if let Some(result_str) = results.get("result").and_then(|r| r.as_str()) {
        println!(
            "Submit BUY to {} / {}",
            CONFIRM_SERVICE.as_str(),
            result_str
        );
    }
}
