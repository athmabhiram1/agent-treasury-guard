use anchor_lang::prelude::*;

use crate::errors::GuardError;
use crate::state::{Approval, Mandate, Treasury};

#[event]
pub struct LargeApproved {
    pub mandate: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(amount: u64, idem_key: [u8; 32])]
pub struct HumanApproveLarge<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"treasury", treasury.authority.as_ref()],
        bump = treasury.bump,
        has_one = authority @ GuardError::Unauthorized,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        seeds = [b"mandate", treasury.key().as_ref(), &mandate.nonce.to_le_bytes()],
        bump = mandate.bump,
    )]
    pub mandate: Account<'info, Mandate>,
    #[account(
        init,
        payer = authority,
        space = 8 + Approval::INIT_SPACE,
        seeds = [b"approval", mandate.key().as_ref(), &idem_key],
        bump,
    )]
    pub approval: Account<'info, Approval>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<HumanApproveLarge>, amount: u64, idem_key: [u8; 32]) -> Result<()> {
    let bump = ctx.bumps.approval;
    let mandate_key = ctx.accounts.mandate.key();
    let a = &mut ctx.accounts.approval;
    a.mandate = mandate_key;
    a.idem_key = idem_key;
    a.amount = amount;
    a.bump = bump;
    emit!(LargeApproved {
        mandate: mandate_key,
        amount,
    });
    Ok(())
}
