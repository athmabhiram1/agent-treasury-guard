use anchor_lang::prelude::*;

#[error_code]
pub enum GuardError {
    #[msg("E01: treasury already initialized")]
    AlreadyInitialized,
    #[msg("E02: payer must be the authority")]
    PayerNotAuthority,
    #[msg("E03: unauthorized signer")]
    Unauthorized,
    #[msg("E04: mandate nonce already used")]
    NonceReuse,
    #[msg("E05: payee not allowlisted")]
    PayeeNotAllowlisted,
    #[msg("E06: per-payee cap exceeded")]
    PerPayeeCap,
    #[msg("E07: per-transaction ceiling exceeded")]
    TxCeiling,
    #[msg("E08: mandate expired")]
    Expired,
    #[msg("E09: idempotency key replayed")]
    Replay,
    #[msg("E10: treasury killed")]
    Killed,
    #[msg("E11: rolling budget or velocity exceeded")]
    BudgetOrVelocity,
    #[msg("E12: approval missing, consumed, or amount mismatch")]
    ApprovalMissing,
}
