#!/usr/bin/env bash
set -euo pipefail
PROGRAM_ID="${PROGRAM_ID:-Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS}"
PLACEHOLDER="Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
FAILED=0
scene() { echo "=== SCENE: $1 ==="; }
pass() { echo "PASS: $1"; }
fail() { echo "FAIL: $1"; FAILED=$((FAILED+1)); }

echo "program: $PROGRAM_ID"
echo "explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
if [ "$PROGRAM_ID" = "$PLACEHOLDER" ]; then
  echo "placeholder ID: finish Todo 13 deploy, then PROGRAM_ID=<real-id> bash scripts/demo.sh"
  exit 2
fi
command -v anchor >/dev/null 2>&1 || { echo "anchor CLI missing: WSL2 Ubuntu + Anchor 1.2.0 per TOOLCHAIN.md"; exit 2; }

scene "legit disburse (disburse_happy_path_moves_exact_lamports)"
scene "rogue redirect E05 + over-ceiling E07 (disburse_negatives_revert_with_codes)"
scene "large matrix E12/E06 + single-use (large_* tests)"
scene "kill E10 + agent-E03 + unkill restores (kill_* tests)"
scene "velocity E11 fourth-tx + expiry E08 warp + nonce-reuse (new negatives)"
if anchor test --provider.cluster devnet 2>&1 | tee demo.log; then
  pass "suite green on devnet (all 5 judged scenes + negatives)"
else
  fail "suite red (see demo.log)"
fi

echo "replay note: re-running replays spent idem keys and fails by design (E09/init-collision)"
echo "failed scenes: $FAILED"
exit "$FAILED"
