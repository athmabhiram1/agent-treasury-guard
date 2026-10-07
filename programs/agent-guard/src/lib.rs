use anchor_lang::prelude::*;

pub mod checks;
pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const DEFAULT_TX_CEILING: u64 = 500_000_000;
pub const DEFAULT_PER_PAYEE_CAP: u64 = 2_000_000_000;
pub const DEFAULT_ROLLING_BUDGET: u64 = 5_000_000_000;
pub const DEFAULT_WINDOW_SLOTS: u64 = 1_000;
pub const DEFAULT_MAX_DISBURSE_PER_WINDOW: u8 = 3;
pub const DEFAULT_LARGE_THRESHOLD: u64 = 1_000_000_000;
pub const MAX_PAYEES: usize = 8;

#[program]
pub mod agent_guard {
    use super::*;

    pub fn create_treasury(ctx: Context<CreateTreasury>) -> Result<()> {
        instructions::create_treasury::handler(ctx)
    }

    pub fn sign_mandate(
        ctx: Context<SignMandate>,
        nonce: u64,
        payees: Vec<Pubkey>,
        expiry_slot: u64,
    ) -> Result<()> {
        instructions::sign_mandate::handler(ctx, nonce, payees, expiry_slot)
    }

    pub fn agent_disburse(
        ctx: Context<AgentDisburse>,
        amount: u64,
        idem_key: [u8; 32],
    ) -> Result<()> {
        instructions::agent_disburse::handler(ctx, amount, idem_key)
    }

    pub fn human_approve_large(
        ctx: Context<HumanApproveLarge>,
        amount: u64,
        idem_key: [u8; 32],
    ) -> Result<()> {
        instructions::approve_large::handler(ctx, amount, idem_key)
    }

    pub fn agent_disburse_large(
        ctx: Context<AgentDisburseLarge>,
        amount: u64,
        idem_key: [u8; 32],
    ) -> Result<()> {
        instructions::disburse_large::handler(ctx, amount, idem_key)
    }

    pub fn kill_switch(ctx: Context<KillSwitch>, kill: bool) -> Result<()> {
        instructions::kill::handler(ctx, kill)
    }
}
