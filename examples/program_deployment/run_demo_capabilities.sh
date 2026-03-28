#!/usr/bin/env bash
# LP-0015 demo: runs the three capability-system test scenarios.
# Usage: bash examples/program_deployment/run_demo_capabilities.sh
# Requires: Rust toolchain + rzup (https://risczero.com/install)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

GREEN='\033[0;32m'; RED='\033[0;31m'; BOLD='\033[1m'; RESET='\033[0m'
pass() { echo -e "${GREEN}${BOLD}PASS${RESET}  $1"; }
fail() { echo -e "${RED}${BOLD}FAIL${RESET}  $1"; exit 1; }

echo "LP-0015 — capability system demo"
echo ""

echo "1/3  positive path: A → B → A_internal"
if RISC0_DEV_MODE=1 cargo test -p nssa --release \
       -- general_call_capability_positive_path 2>&1 | grep -q "ok"; then
    pass "chained execution completed successfully"
else
    fail "positive path test failed"
fi

echo "2/3  negative path: direct call to internal rejected"
if RISC0_DEV_MODE=1 cargo test -p nssa --release \
       -- direct_call_to_internal_entrypoint_is_rejected 2>&1 | grep -q "ok"; then
    pass "direct invocation correctly rejected"
else
    fail "internal entrypoint was not protected"
fi

echo "3/3  forgery: sequencer rejects a program that forges capabilities"
if RISC0_DEV_MODE=1 cargo test -p nssa --release \
       -- sequencer_rejects_forged_capability 2>&1 | grep -q "ok"; then
    pass "capability forgery correctly rejected"
else
    fail "sequencer accepted a forged capability"
fi

echo ""
echo -e "${GREEN}${BOLD}All scenarios passed.${RESET}"
echo ""
echo "Full suite:  RISC0_DEV_MODE=1 cargo test --release"
echo "Spec:        docs/general-calls-via-tail-c-calls.md"
