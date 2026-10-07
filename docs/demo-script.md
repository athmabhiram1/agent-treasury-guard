# Demo script (technical, 2-3min) — stack / why Solana / architecture

## 0:00 Stack
Anchor 1.2.0 program (`agent-guard`, 6 instructions) + LiteSVM/bankrun TDD
suite (16 tests) + Vite React SPA + `scripts/demo.sh` scene runner. Pinned in
TOOLCHAIN.md; CI runs build/test/fmt/clippy.

## 0:30 Why Solana
Slots are PoH-monotonic time (expiry + 1000-slot rolling windows need no
oracle); PDAs give canonical vault/mandate/spent/approval addresses from
seeds + stored bump; System CPI with PDA signer moves lamports with no
custodian.

## 1:00 Architecture
Treasury PDA `[treasury, authority]` holds budget + window + kill flag.
Mandate PDA `[mandate, treasury, nonce]` binds agent + 8-payee allowlist +
caps + expiry. SpendRecord PDA `[spent, mandate, idem_key]` makes replays
collide (E09). Approval PDA `[approval, mandate, idem_key]` is single-use via
unconditional `close = agent`. Disburse runs 11 ordered checks (killed E10 →
expiry E08 → agent E03 → allowlist E05 → ceiling E07 → payee-cap E06 →
budget/velocity E11 → large-route E12 → replay → pinned System CPI →
checked writes). Approval supersedes the ceiling on the large path; caps
still bind.

## 1:50 Five scenes (run `PROGRAM_ID=<id> bash scripts/demo.sh`)
Legit disburse moves exact lamports; rogue redirect → E05; replay same key →
E09; 600M over 500M ceiling → E07; kill → E10, authority unkill restores.
Explorer links print per scene; exit code = failed scenes.

## 2:30 Threat model
THREAT_MODEL.md: redirect injection, retry-storm duplicate, stolen-key drain
+ 9-row self-audit with file:line evidence and honest gaps (E01/E02/E04 via
init-collision; small-path E12 + budget-E11 unreachable under demo defaults).
