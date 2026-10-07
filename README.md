# Agent Treasury Guard

Mandate-bound idempotent disbursement vault for AI agents on Solana. An agent's
operating budget sits in a PDA vault spendable ONLY inside a human-signed on-chain
mandate: allowlisted payees, per-payee + per-tx caps, rolling budget, expiry,
idempotency keys, kill switch, human co-approval for large spends. Safe even with
the agent key fully stolen — enforcement is Anchor constraints + account state.

## Setup

1. Install: Anchor 1.2.0, Solana CLI 4.1.2, Rust 1.89+, Node 23.9+ (see TOOLCHAIN.md).
   Windows: use WSL2 — Anchor builds require Linux.
2. `anchor build`
3. `anchor test` (starts local validator, runs LiteSVM + integration suites)
4. Devnet: `anchor deploy --provider.cluster devnet`, record program ID below.

## Program ID

- Devnet: `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` (placeholder — replaced at Todo 13 deploy)
- Explorer: https://explorer.solana.com/address/Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS?cluster=devnet

## Demo

```bash
bash scripts/demo.sh   # 5 scenes: legit, rogue-redirect, replay, over-cap, kill
```

Frontend: `cd app && npm install && npm run dev` (wallet → devnet), `npm run build` for Vercel static dist.

## Money defaults (demo-tuned, not production policy)

per-tx ceiling 0.5 SOL · per-payee cap 2 SOL · rolling budget 5 SOL / 1000 slots ·
velocity ≤3 / window · large threshold 1 SOL · expiry = slot number · idem key = 32 bytes.

## Troubleshooting

- `anchor: command not found` → install via AVM (`avm install 1.2.0 && avm use 1.2.0`).
- `buffer is not defined` (Vite) → `vite.config.ts` already aliases `buffer`; `npm install buffer`.
- Wallet tx fails on devnet → airdrop: `solana airdrop 2 --url devnet`.
- Replay test "fails" on second run → designed (E09): use a fresh idem key.
