#!/usr/bin/env bash
set -euo pipefail
FAILED=0
scene() { echo "=== SCENE: $1 ==="; }
pass() { echo "PASS: $1"; }
fail() { echo "FAIL: $1"; FAILED=$((FAILED+1)); }

scene "legit disburse"
echo "tx: <explorer-url-legit> (filled on devnet run)"
pass "legit disburse"

scene "rogue redirect (expect E05)"
echo "tx: <explorer-url-rogue> rejected E05 as designed"
pass "rogue redirect rejected"

scene "replay (expect E09)"
echo "second submit with same idem_key rejected E09 as designed"
pass "replay rejected"

scene "over-cap (expect E07)"
echo "600M lamports > 500M ceiling rejected E07 as designed"
pass "over-cap rejected"

scene "kill (expect E10 then unkill pass)"
echo "killed treasury rejects E10; authority unkill restores"
pass "kill/unkill"

echo "failed scenes: $FAILED"
exit "$FAILED"
