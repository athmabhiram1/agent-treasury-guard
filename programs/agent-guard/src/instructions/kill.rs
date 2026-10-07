use anchor_lang::prelude::*;

use crate::errors::GuardError;
use crate::state::Treasury;

#[event]
pub struct Killed {
    pub treasury: Pubkey,
    pub killed: bool,
}

#[derive(Accounts)]
pub struct KillSwitch<'info> {
    #[account(
        mut,
        seeds = [b"treasury", treasury.authority.as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(address = treasury.kill_authority @ GuardError::Unauthorized)]
    pub kill_authority: Signer<'info>,
}

pub fn handler(ctx: Context<KillSwitch>, kill: bool) -> Result<()> {
    ctx.accounts.treasury.killed = kill;
    emit!(Killed {
        treasury: ctx.accounts.treasury.key(),
        killed: kill,
    });
    Ok(())
}
