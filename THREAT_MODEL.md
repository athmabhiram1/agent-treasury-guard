# THREAT_MODEL — Agent Treasury Guard (Todo 11)

## Attack A — redirect injection (rogue payee)

Mechanism: stolen agent key submits disburse to attacker wallet.
Mitigation: allowlist scan in `agent_disburse` (check 4 → E05). Tests: Todo 7 rogue case.

## Attack B — retry-storm duplicate (replay)

Mechanism: attacker/client replays a valid disburse (retry storm, double-submit).
Mitigation: `spent` PDA per `(mandate, idem_key)`; existence = replay (E09 via Anchor
init collision). Tests: Todo 7 replay case + demo double-run.

## Attack C — stolen-key drain (cap/velocity/budget bypass)

Mechanism: stolen agent key attempts max extraction.
Mitigation: layered caps — tx ceiling E07, per-payee cap E06, rolling budget + velocity
E11, large-threshold route E12, kill switch E10, expiry E08. Tests: Todos 7-10.

## Self-audit checklist (file:line evidence — 2026-10-07, post disburse_large killed-first fix)

- [x] signer + has_one on every privileged ix — sign_mandate.rs:20-23 (`has_one = authority @ E03`), approve_large.rs:18-21, kill.rs:21 (`address = kill_authority @ E03`); agent is `Signer` in agent_disburse.rs:12 + disburse_large.rs:12
- [x] no bare-authority `AccountInfo` where typed account required — all privileged accounts are `Account<'info, Treasury/Mandate/Approval>` or `Signer`/`SystemAccount`/`Program<System>`; agent in sign_mandate.rs:24 is `UncheckedAccount` (pubkey only, never authority)
- [x] canonical PDA: stored bump + `bump = <target>` on non-init use — create_treasury.rs:20 bare `bump` on init + :28 stored; sign_mandate.rs:21-23 `bump = treasury.bump`; agent_disburse.rs:15-17,21-23 `bump = treasury/mandate.bump`; disburse_large.rs:15-17,21-23,38 `bump = approval.bump`; kill.rs:15 `bump = treasury.bump`
- [x] typed `Account<'info, T>` everywhere (anti-cosplay discriminator) — state.rs:3-52 all `#[account] + InitSpace`; no raw `AccountInfo` in any instruction context
- [x] pinned System Program ID (never account input) — `Program<'info, System>` in create_treasury.rs:25, sign_mandate.rs:36, agent_disburse.rs:42, approve_large.rs:39, disburse_large.rs:42; CPI via `new_with_signer` agent_disburse.rs:101-114 + disburse_large.rs:116-129
- [x] duplicate-mutable rejection + payee ≠ treasury/mandate/record/approval keys — checks.rs:51-65 `reject_self_transfer` called agent_disburse.rs:99 + disburse_large.rs:108-114 (with approval key)
- [x] `checked_*` math + workspace `overflow-checks = true` — checks.rs:20-21,40-41, checks agent_disburse.rs:73-78,116-134 + disburse_large.rs:87-92,131-149 all `checked_add`; overflow-checks=true in Cargo.toml [profile.release] + programs/agent-guard/Cargo.toml [profile.release]
- [x] `init` never `init_if_needed`; single-use approval closes unconditionally — grep `init_if_needed` = zero hits; disburse_large.rs:35-40 `close = agent` unconditional, no conditional close path
- [x] slots (PoH monotonic), never wall-clock — Clock::get()?.slot only: create_treasury.rs:34, sign_mandate.rs:47+49 (expiry>slot), agent_disburse.rs:46+53-56, disburse_large.rs:46+expiry check; zero `unix_timestamp` references

Known gaps (honest disclosure for Adevar human review): E01/E02/E04 never emitted as typed errors — re-init/nonce-reuse surface as Anchor init-collision, E02 unenforced beyond init payer (errors.rs:5,7,11 unused); E09 replay via init-collision not typed error (tests assert failure); rollover underflow maps to E08 (checks.rs:22); ECC AgentShield 2026-10-07 Grade A on harness config (0 files — no agent-config surface, NOT a program audit).

2026-10-07 fixes: disburse_large.rs approval checks moved after killed/expiry/agent binding (E10 now outranks E12); tx_ceiling check REMOVED from large path (human-signed approval supersedes ceiling; per-payee cap E06 + budget/velocity E11 still bind) — this also flips the pre-existing 1.5 SOL large test from E07-fail to pass. Reachability notes under demo defaults: small-path E12 dead (ceiling 0.5 < threshold 1.0, E07 fires first — kept as defense-in-depth); budget-E11 unreachable (max 3×2 SOL < 5 SOL budget — velocity E11 fires first); E06 reachable only via large path (covered by large_second_disburse test). New tests 2026-10-07 (verify in WSL2/CI via `anchor test`): large_without_approval, large_amount_mismatch_E12, large_single_use_reuse, large_E06_second_disburse, killed_large_E10, kill_agent_E03 ×2, velocity_fourth_E11, expired_E08 (warp_to_slot), nonce_reuse. Rust `cargo check` + `anchor test` NOT runnable on this Windows box (no MSVC linker, no Anchor CLI, WSL has docker-desktop distro only) — verification deferred to CI/WSL2 Ubuntu.
