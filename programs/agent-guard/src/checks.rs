use anchor_lang::prelude::*;

use crate::errors::GuardError;
use crate::{DEFAULT_MAX_DISBURSE_PER_WINDOW, DEFAULT_WINDOW_SLOTS};

pub fn payee_index(payees: &[Pubkey], payee: &Pubkey) -> Result<usize> {
    payees
        .iter()
        .position(|p| p == payee)
        .ok_or(GuardError::PayeeNotAllowlisted.into())
}

pub fn rollover_window(
    spent_window: &mut u64,
    window_start_slot: &mut u64,
    window_count: &mut u8,
    per_payee_spent: &mut [u64],
    now_slot: u64,
) -> Result<()> {
    let elapsed = now_slot
        .checked_sub(*window_start_slot)
        .ok_or(GuardError::Expired)?;
    if elapsed > DEFAULT_WINDOW_SLOTS {
        *spent_window = 0;
        *window_start_slot = now_slot;
        *window_count = 0;
        for s in per_payee_spent.iter_mut() {
            *s = 0;
        }
    }
    Ok(())
}

pub fn check_budget_velocity(
    spent_window: u64,
    window_count: u8,
    budget: u64,
    amount: u64,
) -> Result<()> {
    let next = spent_window
        .checked_add(amount)
        .ok_or(GuardError::BudgetOrVelocity)?;
    require!(next <= budget, GuardError::BudgetOrVelocity);
    require!(
        window_count < DEFAULT_MAX_DISBURSE_PER_WINDOW,
        GuardError::BudgetOrVelocity
    );
    Ok(())
}

pub fn reject_self_transfer(
    payee: &Pubkey,
    treasury: &Pubkey,
    mandate: &Pubkey,
    record: &Pubkey,
    approval: Option<&Pubkey>,
) -> Result<()> {
    require_keys_neq!(*payee, *treasury);
    require_keys_neq!(*payee, *mandate);
    require_keys_neq!(*payee, *record);
    if let Some(a) = approval {
        require_keys_neq!(*payee, *a);
    }
    Ok(())
}
