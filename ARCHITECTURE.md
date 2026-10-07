# ARCHITECTURE — Agent Treasury Guard

## PDAs

- `treasury [b"treasury", authority]` — vault + budget window + kill flag (99 B).
- `mandate [b"mandate", treasury, nonce]` — agent, allowlist, caps, expiry (466 B).
- `spent [b"spent", mandate, idem_key]` — replay guard, existence = spent (89 B).
- `approval [b"approval", mandate, idem_key]` — single-use large approval, `close = agent` (81 B).

## Flows

```
create_treasury (authority) → vault PDA, budget 5 SOL, kill_authority = authority
sign_mandate (authority)    → allowlist + caps + expiry, agent bound
agent_disburse (agent)      → 11 ordered checks → System CPI (treasury PDA signer) → writes
human_approve_large (authority) → amount-bound approval PDA
agent_disburse_large (agent)    → approval match → checks → CPI → unconditional close
kill_switch (kill_authority)    → killed=true/false, checked FIRST in disburse
```

## Check order in disburse (fail-fast, cheap first)

1. killed→E10 · 2. expiry→E08 · 3. agent match→E03 · 4. allowlist→E05 ·
5. tx ceiling→E07 · 6. per-payee cap→E06 · 7. window rollover + budget/velocity→E11 ·
8. large threshold→E12 (route to large) · 9. spend_record init (replay→E09) ·
10. pinned System CPI via `new_with_signer [b"treasury", authority, bump]` · 11. checked writes.

NOTE: Anchor `init` runs before the body, so a replayed key (E09) outranks later body
checks when both apply; killed-first holds otherwise.

## Exact integrations

- On-chain: anchor-lang 1.2.0, anchor-spl 1.2.0, System Program (ID pinned in code).
- Tests: anchor-litesvm 0.4 / litesvm 0.17, `anchor test` on localnet validator.
- Client: `@anchor-lang/core` 1.2.0, `@solana/web3.js` 1.x, wallet-adapter, Vite 7 static.
- Stretch: Token-2022 confidential transfers via `@solana-program/token-2022`
  (`getConfidentialTransferInstructionPlan`) / `spl-token-client`, mandate-bound test mint.

## Diagram

```
[human authority] ─sign_mandate→ [mandate PDA] ←agent_disburse─ [agent key]
        │                              │                    (stolen key: still bounded)
        │ kill_switch                  ▼
        └────────────────────→ [treasury PDA] ─CPI→ [allowlisted payee]
                                       │
                        [spent PDA / approval PDA] = replay + large guards
```

Money/window defaults are demo-tuned values for judging legibility, not production
treasury policy.
