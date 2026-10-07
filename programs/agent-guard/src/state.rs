use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey,
    pub kill_authority: Pubkey,
    pub budget: u64,
    pub spent_window: u64,
    pub window_start_slot: u64,
    pub window_count: u8,
    pub killed: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Mandate {
    pub treasury: Pubkey,
    pub nonce: u64,
    pub agent: Pubkey,
    pub ct_mint: Pubkey,
    #[max_len(8)]
    pub payees: Vec<Pubkey>,
    pub payee_count: u8,
    #[max_len(8)]
    pub per_payee_spent: Vec<u64>,
    pub per_payee_cap: u64,
    pub tx_ceiling: u64,
    pub large_threshold: u64,
    pub expiry_slot: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct SpendRecord {
    pub mandate: Pubkey,
    pub idem_key: [u8; 32],
    pub amount: u64,
    pub slot: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Approval {
    pub mandate: Pubkey,
    pub idem_key: [u8; 32],
    pub amount: u64,
    pub bump: u8,
}
