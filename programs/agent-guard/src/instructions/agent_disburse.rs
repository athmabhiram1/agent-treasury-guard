use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::checks::{check_budget_velocity, payee_index, reject_self_transfer, rollover_window};
use crate::errors::GuardError;
use crate::state::{Mandate, SpendRecord, Treasury};

#[event]
pub struct Disbursed {
    pub mandate: Pubkey,
    pub payee: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(amount: u64, idem_key: [u8; 32])]
pub struct AgentDisburse<'info> {
    #[account(mut)]
    pub agent: Signer<'info>,
    #[account(
        mut,
        seeds = [b"treasury", treasury.authority.as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [b"mandate", treasury.key().as_ref(), &mandate.nonce.to_le_bytes()],
        bump = mandate.bump,
    )]
    pub mandate: Account<'info, Mandate>,
    #[account(mut)]
    pub payee: SystemAccount<'info>,
    #[account(
        init,
        payer = agent,
        space = 8 + SpendRecord::INIT_SPACE,
        seeds = [b"spent", mandate.key().as_ref(), &idem_key],
        bump,
    )]
    pub spend_record: Account<'info, SpendRecord>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AgentDisburse>, amount: u64, idem_key: [u8; 32]) -> Result<()> {
    let now_slot = Clock::get()?.slot;
    let treasury_key = ctx.accounts.treasury.key();
    let mandate_key = ctx.accounts.mandate.key();
    let record_key = ctx.accounts.spend_record.key();
    let payee_key = ctx.accounts.payee.key();

    require!(!ctx.accounts.treasury.killed, GuardError::Killed);
    require!(
        now_slot < ctx.accounts.mandate.expiry_slot,
        GuardError::Expired
    );
    require_keys_eq!(
        ctx.accounts.agent.key(),
        ctx.accounts.mandate.agent,
        GuardError::Unauthorized
    );
    require!(
        ctx.accounts.mandate.treasury == treasury_key,
        GuardError::Unauthorized
    );

    let idx = payee_index(&ctx.accounts.mandate.payees, &payee_key)?;
    require!(
        amount <= ctx.accounts.mandate.tx_ceiling,
        GuardError::TxCeiling
    );

    {
        let m = &ctx.accounts.mandate;
        let spent = m.per_payee_spent.get(idx).copied().unwrap_or(u64::MAX);
        let next = spent.checked_add(amount).ok_or(GuardError::PerPayeeCap)?;
        require!(next <= m.per_payee_cap, GuardError::PerPayeeCap);
    }

    rollover_window(
        &mut ctx.accounts.treasury.spent_window,
        &mut ctx.accounts.treasury.window_start_slot,
        &mut ctx.accounts.treasury.window_count,
        &mut ctx.accounts.mandate.per_payee_spent,
        now_slot,
    )?;
    check_budget_velocity(
        ctx.accounts.treasury.spent_window,
        ctx.accounts.treasury.window_count,
        ctx.accounts.treasury.budget,
        amount,
    )?;

    require!(
        amount <= ctx.accounts.mandate.large_threshold,
        GuardError::ApprovalMissing
    );

    reject_self_transfer(&payee_key, &treasury_key, &mandate_key, &record_key, None)?;

    let authority = ctx.accounts.treasury.authority;
    let bump = ctx.accounts.treasury.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", authority.as_ref(), &[bump]]];
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.payee.to_account_info(),
        },
    )
    .with_signer(signer_seeds);
    transfer(cpi_context, amount)?;
    ctx.accounts.treasury.spent_window = ctx
        .accounts
        .treasury
        .spent_window
        .checked_add(amount)
        .ok_or(GuardError::BudgetOrVelocity)?;
    ctx.accounts.treasury.window_count = ctx
        .accounts
        .treasury
        .window_count
        .checked_add(1)
        .ok_or(GuardError::BudgetOrVelocity)?;
    let entry = ctx
        .accounts
        .mandate
        .per_payee_spent
        .get_mut(idx)
        .ok_or(GuardError::PayeeNotAllowlisted)?;
    *entry = entry.checked_add(amount).ok_or(GuardError::PerPayeeCap)?;

    let rec = &mut ctx.accounts.spend_record;
    rec.mandate = mandate_key;
    rec.idem_key = idem_key;
    rec.amount = amount;
    rec.slot = now_slot;
    rec.bump = ctx.bumps.spend_record;

    emit!(Disbursed {
        mandate: mandate_key,
        payee: payee_key,
        amount,
    });
    Ok(())
}
