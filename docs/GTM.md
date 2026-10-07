# GTM — Agent Treasury Guard (Todo 16 one-pager)

## Problem
AI agents now move real money (payroll, bounties, trading ops), but a stolen
agent key means an unlimited drain. Existing mitigations live off-chain
(guard servers, middleware) or anonymize instead of authorize.

## Who pays
Teams running Solana agent payroll/ops wallets; hackathon-to-mainnet
shipping path via the Adevar pre-audit (trust signal) + Colosseum launch.

## Wedge vs incumbents
- Cloak = UTXO shield-pool privacy (Groth16 2-in/2-out, viewing keys,
  `@cloak.dev/sdk` relay). Hides amounts, does not bound agent spend.
- Aegis (exPardus) = autonomous ML defense (4 experts + meta-selector,
  staged GROWTH→NEUTRAL→DEFENSIVE). Probabilistic, black-box for auditors.
- Aegis SDKs = off-chain guard middleware (`maxTransaction`, rate-limit,
  killswitch). Bypassed if the key is used outside the middleware.
- Treasury Guard = deterministic on-chain mandate: allowlist + caps +
  rolling budget/velocity + expiry + idempotency + kill + human co-approval,
  enforced in Anchor constraints + account state. Safe with the agent key
  fully stolen; no ML, no anonymity set, audit-friendly.

## Distribution
Colosseum Crypto World's Fair entry + Superteam India track (separate) +
Adevar Earn sidetrack + tweet; devnet demo (5 scenes, explorer links) +
pitch ≤3min / technical 2-3min videos.

## Money defaults are demo-tuned (0.5 tx / 2 payee / 5 budget SOL, 1000-slot
window, ≤3/window, 1 SOL large threshold), not production policy.
