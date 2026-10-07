use anchor_lang::prelude::*;

use crate::state::Treasury;
use crate::DEFAULT_ROLLING_BUDGET;

#[event]
pub struct TreasuryCreated {
    pub treasury: Pubkey,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CreateTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury", authority.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTreasury>) -> Result<()> {
    let bump = ctx.bumps.treasury;
    let t = &mut ctx.accounts.treasury;
    t.authority = ctx.accounts.authority.key();
    t.kill_authority = ctx.accounts.authority.key();
    t.budget = DEFAULT_ROLLING_BUDGET;
    t.spent_window = 0;
    t.window_start_slot = Clock::get()?.slot;
    t.window_count = 0;
    t.killed = false;
    t.bump = bump;
    emit!(TreasuryCreated {
        treasury: t.key(),
        authority: t.authority,
    });
    Ok(())
}
