# TOOLCHAIN.md — Agent Treasury Guard (Todo 1 spike output)

Pinned STABLE-ONLY versions. Policy: no nightly, no experimental features, cheapest
integration-risk choice. Newest is banned unless a pinned stable lacks the required API.

## Pins (verbatim, used by all later todos)

| Component | Pinned version | Source |
|---|---|---|
| Anchor CLI / `anchor-lang` / `anchor-spl` | **1.2.0** (stable) | crates.io `anchor-lang` Stable: 1.2.0; `2.0.0-rc.1` is RC — BANNED per stable-only policy |
| Solana / Agave CLI | **4.1.2** (`solana-cli 4.1.2, client:Agave`, platform-tools `v1.57`) | Anchor 1.2.0 release notes: recommended toolchain moves to 4.1.2 |
| Rust | **1.89.0** (`anchor-lang` MSRV per changelog #4638; local rustc 1.94.1 is newer and accepted) | Anchor changelog |
| `anchor-litesvm` (dev-dep) | **0.4** | crates.io latest 0.4.0 (2026-04-09) |
| `litesvm` (via anchor-litesvm) | **0.17.0** | crates.io latest 0.17.0 (2026-09-28) |
| `@solana-program/token-2022` (JS CT helpers) | latest stable `getConfidentialTransferInstructionPlan` line | solana.com CT integration-guide |
| `spl-token-client` (Rust CT helpers) | latest stable `confidential_transfer_transfer` line | solana.com CT transfer-tokens page |
| Node.js | **23.9.0** (local v24.11.0 accepted) | Anchor install docs |
| Frontend | **Vite 7 + React SPA**, `vite build` → static `dist` for Vercel | Context7 `/vitejs/vite` static-deploy docs |

## Context7 baseline lookups (Todo 1)

- Anchor PDA/seeds/bump + `close` semantics: `/websites/anchor-lang` — `init` uses bare
  `bump` (target errors on init); later uses store bump + `bump = <target>`; `close = <target>`.
  Docs: https://www.anchor-lang.com/docs/references/account-constraints ,
  https://www.anchor-lang.com/docs/updates/changelog
- Anchor workspace/test/`init`/`InitSpace`/`require!`/`emit!`: `/websites/anchor-lang`.
- Confidential transfers (mint → configure → mint-to → deposit → apply → 3 proof-context
  accounts → transfer; **multi-tx required today**, single-tx is future): `/websites/solana`
  integration-guide + transfer-tokens pages.
- Vite static deploy + `buffer` polyfill alias for Solana web3: `/vitejs/vite`.
- If Context7 unreachable later: websearch fallback, URL logged as `Docs:` trailer on the commit.

## LiteSVM probe (expected green once Anchor/Solana CLIs installed)

> NOTE (Windows dev machine): `anchor` and `solana` CLIs are NOT installed here
> (Rust 1.94.1 + Node v24.11.0 present). Anchor builds require Linux/WSL.
> Run the loop in WSL2 or CI: `anchor build`, `anchor test`, LiteSVM slot-warp +
> event-assertion probe. Evidence log goes to `spike/probe.log`.

## CT helper gap list (from docs)

- `anchor-spl::token_2022_extensions` covers common extension helpers but NOT all
  confidential-transfer instructions → manual CPI where missing (Todo 12).
- Client proofs via `@solana-program/token-2022/confidential`
  (`getConfidentialTransferInstructionPlan`) or `spl-token-client`; on-chain program only
  verifies against mandate-bound test mint, never builds proofs.

## Submission-rule notes (verified 2026-10-05)

- Colosseum Crypto World's Fair: submissions due **Oct 12 2026 11:59pm PT**
  (= Oct 13 06:59 UTC). Rules PDF + worldsfair page agree.
- Adevar Earn sidetrack: opens Sep 23 2026, closes **Oct 12 2026** (alongside CWF deadline),
  5 × $4,000 pre-audit in-kind awards. Winners ~Oct 27.
- Superteam India Track exists (5,000 USDG, due in ~9d) — submit SEPARATELY from Adevar.
- Colosseum profile country=**India** required for the India track.
- Hard buffer: everything live by **Oct 12 2026 12:00 UTC** (~19h before PT close).
- Venue note: `task.md` at repo root is a scraped Earn-listing snapshot, not source of truth.
