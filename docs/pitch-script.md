# Pitch script (≤3min) — read at pace, total ~360 words

## 0:00 Team
Solo builder, India. Shipping an agent-safety primitive for the agent economy.

## 0:20 Problem
AI agents move real money now — payroll, bounties, ops. One stolen agent key
drains the whole wallet. Today's fixes live off-chain: guard servers and
middleware the key can simply bypass.

## 0:45 Who-for + market
Teams running Solana agent wallets. Every agentúl treasury is a customer the
day agents hold budgets — that day is this hackathon.

## 1:05 Product + GTM
Agent Treasury Guard: the budget sits in a PDA vault spendable ONLY inside a
human-signed on-chain mandate — allowlisted payees, per-tx + per-payee caps,
rolling budget, expiry, idempotency keys, kill switch, human co-approval for
large spends. Deterministic, audit-friendly. No ML black box, no anonymity
set. GTM in docs/GTM.md.

## 1:40 Demo (aha at ~2:00)
Watch a STOLEN agent key try three attacks and fail on-chain: rogue redirect
(E05), replayed payout (E09), capped drain (E06/E07/E11) — then the human
kills and unkills the treasury (E10). Enforcement is constraints + state, so
the thief follows the rules anyway.

## 2:30 Ask
Devnet live, tests green, pre-audit ready. Agent budgets should be
mandate-bound. Demo links in docs/submissions.md.

# Demo script was: docs/demo-script.md (technical 2-3min) — stack / why Solana / architecture
