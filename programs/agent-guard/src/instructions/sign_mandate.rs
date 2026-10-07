use anchor_lang::prelude::*;

use crate::errors::GuardError;
use crate::state::{Mandate, Treasury};
use crate::{DEFAULT_LARGE_THRESHOLD, DEFAULT_PER_PAYEE_CAP, DEFAULT_TX_CEILING, MAX_PAYEES};

#[event]
pub struct MandateSigned {
    pub mandate: Pubkey,
    pub treasury: Pubkey,
    pub agent: Pubkey,
    pub nonce: u64,
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct SignMandate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"treasury", treasury.authority.as_ref()],
        bump = treasury.bump,
        has_one = authority @ GuardError::Unauthorized,
    )]
    pub treasury: Account<'info, Treasury>,
    /// CHECK: agent pubkey is only recorded into the mandate; it signs
    /// nothing here. Authorization is enforced at use sites via
    /// require_keys_eq against mandate.agent in both disburse handlers.
    pub agent: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Mandate::INIT_SPACE,
        seeds = [b"mandate", treasury.key().as_ref(), &nonce.to_le_bytes()],
        bump,
    )]
    pub mandate: Account<'info, Mandate>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SignMandate>,
    nonce: u64,
    payees: Vec<Pubkey>,
    expiry_slot: u64,
) -> Result<()> {
    require!(
        !payees.is_empty() && payees.len() <= MAX_PAYEES,
        GuardError::PayeeNotAllowlisted
    );
    let clock = Clock::get()?;
    require!(expiry_slot > clock.slot, GuardError::Expired);
    let bump = ctx.bumps.mandate;
    let m = &mut ctx.accounts.mandate;
    m.treasury = ctx.accounts.treasury.key();
    m.nonce = nonce;
    m.agent = ctx.accounts.agent.key();
    m.ct_mint = Pubkey::default();
    m.payees = payees;
    m.payee_count = m.payees.len() as u8;
    m.per_payee_spent = vec![0u64; m.payees.len()];
    m.per_payee_cap = DEFAULT_PER_PAYEE_CAP;
    m.tx_ceiling = DEFAULT_TX_CEILING;
    m.large_threshold = DEFAULT_LARGE_THRESHOLD;
    m.expiry_slot = expiry_slot;
    m.bump = bump;
    emit!(MandateSigned {
        mandate: m.key(),
        treasury: m.treasury,
        agent: m.agent,
        nonce,
    });
    Ok(())
}
