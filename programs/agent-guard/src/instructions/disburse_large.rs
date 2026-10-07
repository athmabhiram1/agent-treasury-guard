use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::checks::{check_budget_velocity, payee_index, reject_self_transfer, rollover_window};
use crate::errors::GuardError;
use crate::state::{Approval, Mandate, SpendRecord, Treasury};

#[derive(Accounts)]
#[instruction(amount: u64, idem_key: [u8; 32])]
pub struct AgentDisburseLarge<'info> {
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
    #[account(
        mut,
        seeds = [b"approval", mandate.key().as_ref(), &idem_key],
        bump = approval.bump,
        close = agent,
    )]
    pub approval: Account<'info, Approval>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AgentDisburseLarge>, amount: u64, idem_key: [u8; 32]) -> Result<()> {
    let now_slot = Clock::get()?.slot;
    let treasury_key = ctx.accounts.treasury.key();
    let mandate_key = ctx.accounts.mandate.key();
    let record_key = ctx.accounts.spend_record.key();
    let approval_key = ctx.accounts.approval.key();
    let payee_key = ctx.accounts.payee.key();

    // Killed-first: fail-fast cheap checks before approval binding (E10 outranks E12).
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

    require!(
        ctx.accounts.approval.mandate == mandate_key,
        GuardError::ApprovalMissing
    );
    require!(
        ctx.accounts.approval.idem_key == idem_key,
        GuardError::ApprovalMissing
    );
    require!(
        ctx.accounts.approval.amount == amount,
        GuardError::ApprovalMissing
    );

    // Approval supersedes the per-tx ceiling: the human-signed amount IS the
    // authorization, so no tx_ceiling check here. Per-payee cap (E06) and
    // budget/velocity (E11) still bind every large spend.
    let idx = payee_index(&ctx.accounts.mandate.payees, &payee_key)?;

    {
        let m = &ctx.accounts.mandate;
        let spent = m.per_payee_spent.get(idx).copied().unwrap_or(u64::MAX);
        let next = spent.checked_add(amount).ok_or(GuardError::PerPayeeCap)?;
        require!(next <= m.per_payee_cap, GuardError::PerPayeeCap);
    }

    let mut spent_window = ctx.accounts.treasury.spent_window;
    let mut window_start_slot = ctx.accounts.treasury.window_start_slot;
    let mut window_count = ctx.accounts.treasury.window_count;
    rollover_window(
        &mut spent_window,
        &mut window_start_slot,
        &mut window_count,
        &mut ctx.accounts.mandate.per_payee_spent,
        now_slot,
    )?;
    ctx.accounts.treasury.spent_window = spent_window;
    ctx.accounts.treasury.window_start_slot = window_start_slot;
    ctx.accounts.treasury.window_count = window_count;
    check_budget_velocity(
        ctx.accounts.treasury.spent_window,
        ctx.accounts.treasury.window_count,
        ctx.accounts.treasury.budget,
        amount,
    )?;

    reject_self_transfer(
        &payee_key,
        &treasury_key,
        &mandate_key,
        &record_key,
        Some(&approval_key),
    )?;

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

    emit!(crate::instructions::agent_disburse::Disbursed {
        mandate: mandate_key,
        payee: payee_key,
        amount,
    });
    Ok(())
}
