use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn pumpfun_buy_original_ix(
    pumpfun_global_acc: Pubkey,
    mint_addr: Pubkey,
    pumpfun_fee_acc: Pubkey,
    pumpfun_bonding_curve: Pubkey,
    pumpfun_bonding_curve_ata: Pubkey,
    event_authority: Pubkey,
    payer_ata: Pubkey,
    payer: Pubkey,
    rent: Pubkey,
    pumpfun_program_id: Pubkey,
    system_program: Pubkey,
    token_program: Pubkey,
    token_amount: u64,
    max_sol_cost: u64,
) -> Instruction {
    let discriminator = vec![102, 6, 61, 18, 1, 218, 235, 234];
    let mut data = discriminator;
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&max_sol_cost.to_le_bytes());

    println!(
        "{:#?}",
        (
            pumpfun_global_acc,
            pumpfun_fee_acc,
            mint_addr,
            pumpfun_bonding_curve,
            pumpfun_bonding_curve_ata,
            payer_ata,
            payer,
            system_program,
            token_program,
            rent,
            event_authority,
            pumpfun_program_id,
        )
    );

    let accounts = vec![
        AccountMeta::new_readonly(pumpfun_global_acc, false),
        AccountMeta::new(pumpfun_fee_acc, false),
        AccountMeta::new(mint_addr, false),
        AccountMeta::new(pumpfun_bonding_curve, false),
        AccountMeta::new(pumpfun_bonding_curve_ata, false),
        AccountMeta::new(payer_ata, false),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(system_program, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new(rent, false),
        AccountMeta::new_readonly(event_authority, false),
        AccountMeta::new_readonly(pumpfun_program_id, false),
    ];

    Instruction {
        program_id: pumpfun_program_id,
        accounts,
        data,
    }
}
