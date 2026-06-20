#!/usr/bin/env bash
# Launch ONE feature worker: an isolated git worktree + a headless `claude -p` session
# that implements one angr-inspired kuna feature end-to-end and opens a PR.
#
# Invoked by tools/pipeline/run.sh, but runnable standalone for a single iteration:
#   WORKER_ID=w1 OPP_ID='test_x::main' TEST_NAME=test_x SELECTOR=main \
#   BINARY=/path/bin SLUG=myfeat-ab12cd ARCH= tools/pipeline/worker.sh
#
# PIPELINE_MODE=port runs a Rust-port worker instead (see docs/pipeline.md "Port mode"):
# branch rport/<SLUG> off rust-port, porter or verifier prompt chosen by KIND, no C++
# build (the main tree's decomp binaries are reused), cargo smoke build instead.
#   PIPELINE_MODE=port WORKER_ID=w1 ITEM_ID=w1-base-foundation KIND=port \
#   CRATE=kuna-base MODULES='decompiler/cpp/error.hh ...' GATE='cd rust && cargo test -p kuna-base' \
#   SLUG=w1-base-foundation TARGET_BRANCH= tools/pipeline/worker.sh
#
# The worktree and the Claude session transcript are PRESERVED after the run so a human
# reviewer can resume the session on the PR (`claude --resume <session_id>` in the worktree).
# tools/pipeline/run.sh garbage-collects a worktree only once its PR is merged/closed.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
MODE="${PIPELINE_MODE:-feature}"           # "feature" (default, unchanged) | "port"
MODEL="${WORKER_MODEL:-opus}"
WORKER_TIMEOUT="${WORKER_TIMEOUT:-7200}"   # seconds; hard cap per feature
if [ "$MODE" = "port" ]; then
  BASE_BRANCH="${BASE_BRANCH:-rust-port}"
else
  BASE_BRANCH="${BASE_BRANCH:-main}"
fi

: "${WORKER_ID:?need WORKER_ID}"
if [ "$MODE" = "port" ]; then
  : "${ITEM_ID:?need ITEM_ID}"
  : "${KIND:?need KIND (port|verify)}"
  : "${CRATE:?need CRATE}"
  : "${GATE:?need GATE}"
  : "${SLUG:?need SLUG}"
  MODULES="${MODULES:-}"
  TARGET_BRANCH="${TARGET_BRANCH:-}"
  if [ "$KIND" = "verify" ] && [ -z "$TARGET_BRANCH" ]; then
    echo "verify items need TARGET_BRANCH (the port item's rport/ branch)" 1>&2
    exit 1
  fi
  OPP_ID="$ITEM_ID"     # the state claim/done key in port mode is the checklist item id
else
  : "${OPP_ID:?need OPP_ID}"
  : "${TEST_NAME:?need TEST_NAME}"
  : "${SELECTOR:?need SELECTOR}"
  : "${BINARY:?need BINARY}"
  : "${SLUG:?need SLUG}"
  ARCH="${ARCH:-}"
fi

# Keep ALL pipeline state in the MAIN tree so `kuna.pipeline.status` (run from the main
# tree) sees this worker's heartbeats. Without this, the worktree's KUNA_ROOT would route
# state into the worktree's own .kuna-pipeline and the main-tree status would go stale.
export KUNA_PIPELINE_STATE_DIR="$REPO/.kuna-pipeline"

BRANCH="feat/angr-${SLUG}"
WT_ROOT="$REPO/.kuna-pipeline/worktrees"
WT="$WT_ROOT/$WORKER_ID"
LOG_DIR="$REPO/.kuna-pipeline/logs"
PROMPT_TMPL="$REPO/tools/pipeline/worker_prompt.md"
DATE="$(date +%Y-%m-%d)"

# Port mode: rport/ branches; the prompt template is chosen by item KIND.
# Verify-kind workers branch FROM the port branch (TARGET_BRANCH) so the ported code is
# checked out and the adversarial tests + verdict commit cleanly on top of it.
WT_BASE="$BASE_BRANCH"
if [ "$MODE" = "port" ]; then
  BRANCH="rport/${SLUG}"
  if [ "$KIND" = "verify" ]; then
    PROMPT_TMPL="$REPO/tools/pipeline/verifier_port_prompt.md"
    WT_BASE="$TARGET_BRANCH"
  else
    PROMPT_TMPL="$REPO/tools/pipeline/worker_port_prompt.md"
  fi
fi

mkdir -p "$WT_ROOT" "$LOG_DIR"
LOG="$LOG_DIR/$WORKER_ID.log"

log() { echo "[$(date +%H:%M:%S)] worker $WORKER_ID: $*" | tee -a "$LOG"; }

# --- 1. isolated worktree on a fresh feature branch ------------------------
log "creating worktree $WT on $BRANCH (base $WT_BASE)"
git -C "$REPO" worktree add -b "$BRANCH" "$WT" "$WT_BASE" >>"$LOG" 2>&1 || {
  log "worktree add failed (branch may exist); trying detached reuse"
  git -C "$REPO" worktree add "$WT" "$WT_BASE" >>"$LOG" 2>&1 || {
    "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --status failed --note "worktree add failed"
    exit 1
  }
}

"$KUNA_PY" -m kuna.pipeline.state register --worker "$WORKER_ID" --slug "$SLUG" \
  --branch "$BRANCH" --opportunity "$OPP_ID" >>"$LOG" 2>&1

# --- 2. env: kuna tooling targets THIS worktree; specs reuse the main .sla --
if [ "$MODE" = "port" ]; then
  # C++ is unchanged on rust-port: skip the in-worktree build entirely and point the
  # oracle binaries at the MAIN tree's already-built decompiler.
  export KUNA_ROOT="$WT"
  export KUNA_DECOMP_DBG="$REPO/decompiler/cpp/decomp_dbg"
  export KUNA_DECOMP_TEST="$REPO/decompiler/cpp/decomp_test_dbg"
  export KUNA_SLEIGH="$REPO/decompiler/cpp/sleigh_opt"
  export KUNA_SPECS="$REPO/specs"
else
  export KUNA_ROOT="$WT"
  export KUNA_DECOMP_DBG="$WT/decompiler/cpp/decomp_dbg"
  export KUNA_DECOMP_TEST="$WT/decompiler/cpp/decomp_test_dbg"
  export KUNA_SLEIGH="$WT/decompiler/cpp/sleigh_opt"
  export KUNA_SPECS="$REPO/specs"          # reuse the main tree's already-compiled .sla
  export KUNA_PIPELINE_ANGR_PYTHON="${KUNA_PIPELINE_ANGR_PYTHON:-$HOME/.virtualenvs/decbench/bin/python}"
fi

# --- 3. initial build so the worker's analyze phase has working binaries ----
if [ "$MODE" = "port" ]; then
  # cargo fetch first so the network hit is upfront; then a crate-scoped smoke build.
  if [ "$CRATE" = "workspace" ]; then CARGO_SEL="--workspace"; else CARGO_SEL="-p $CRATE"; fi
  log "cargo fetch + smoke build ($CARGO_SEL) in worktree"
  "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --phase setup >>"$LOG" 2>&1
  ( cd "$WT/rust" && cargo fetch && cargo build $CARGO_SEL ) >>"$LOG" 2>&1 || {
    log "initial cargo build failed"
    "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --status failed --note "initial cargo build failed"
    exit 1
  }
else
  log "building binaries in worktree (initial)"
  "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --phase setup >>"$LOG" 2>&1
  ( cd "$WT" && make binaries ) >>"$LOG" 2>&1 || {
    log "initial build failed"
    "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --status failed --note "initial build failed"
    exit 1
  }
fi

# --- 4. render the worker prompt -------------------------------------------
PROMPT_FILE="$LOG_DIR/$WORKER_ID.prompt.md"
if [ "$MODE" = "port" ]; then
  # escape sed-replacement metacharacters (\ & and the | delimiter) in dynamic values
  # (e.g. GATE strings contain `&&`)
  esc() { printf '%s' "$1" | sed -e 's/[\\&|]/\\&/g'; }
  PORT_ITEM_ID="$ITEM_ID"          # the port item whose modules are in scope
  PR_BASE="$BASE_BRANCH"           # port PRs target the integration branch
  if [ "$KIND" = "verify" ]; then
    PORT_ITEM_ID="${TARGET_BRANCH#rport/}"
    PR_BASE="$TARGET_BRANCH"       # verify PRs target the port branch (diff = tests+verdict)
  fi
  VERIFY_TEST_FILE="verify_$(printf '%s' "$PORT_ITEM_ID" | tr '-' '_').rs"
  sed \
    -e "s|{{WORKER_ID}}|$(esc "$WORKER_ID")|g" \
    -e "s|{{ITEM_ID}}|$(esc "$ITEM_ID")|g" \
    -e "s|{{KIND}}|$(esc "$KIND")|g" \
    -e "s|{{CRATE}}|$(esc "$CRATE")|g" \
    -e "s|{{MODULES}}|$(esc "$MODULES")|g" \
    -e "s|{{GATE}}|$(esc "$GATE")|g" \
    -e "s|{{SLUG}}|$(esc "$SLUG")|g" \
    -e "s|{{BRANCH}}|$(esc "$BRANCH")|g" \
    -e "s|{{BASE_BRANCH}}|$(esc "$BASE_BRANCH")|g" \
    -e "s|{{TARGET_BRANCH}}|$(esc "$TARGET_BRANCH")|g" \
    -e "s|{{PORT_ITEM_ID}}|$(esc "$PORT_ITEM_ID")|g" \
    -e "s|{{PR_BASE}}|$(esc "$PR_BASE")|g" \
    -e "s|{{VERIFY_TEST_FILE}}|$(esc "$VERIFY_TEST_FILE")|g" \
    -e "s|{{WORKTREE}}|$(esc "$WT")|g" \
    -e "s|{{KUNA_PY}}|$(esc "$KUNA_PY")|g" \
    -e "s|{{DATE}}|$DATE|g" \
    "$PROMPT_TMPL" > "$PROMPT_FILE"
else
  sed \
    -e "s|{{WORKER_ID}}|$WORKER_ID|g" \
    -e "s|{{OPPORTUNITY_ID}}|$OPP_ID|g" \
    -e "s|{{TEST_NAME}}|$TEST_NAME|g" \
    -e "s|{{SELECTOR}}|$SELECTOR|g" \
    -e "s|{{BINARY}}|$BINARY|g" \
    -e "s|{{ARCH}}|${ARCH:-none}|g" \
    -e "s|{{SLUG}}|$SLUG|g" \
    -e "s|{{BRANCH}}|$BRANCH|g" \
    -e "s|{{WORKTREE}}|$WT|g" \
    -e "s|{{KUNA_PY}}|$KUNA_PY|g" \
    -e "s|{{DATE}}|$DATE|g" \
    "$PROMPT_TMPL" > "$PROMPT_FILE"
fi

# --- 5. headless Claude session (highest-effort model), in the worktree -----
log "launching claude -p (model $MODEL, timeout ${WORKER_TIMEOUT}s)"
"$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --phase analyze >>"$LOG" 2>&1

RESULT_JSON="$LOG_DIR/$WORKER_ID.result.json"
# env -u ANTHROPIC_API_KEY: a stale/invalid ANTHROPIC_API_KEY in the environment makes
# headless `claude -p` fail with "Invalid API key"; unsetting it falls back to the working
# session auth. </dev/null so claude does not wait on stdin.
( cd "$WT" && env -u ANTHROPIC_API_KEY timeout "$WORKER_TIMEOUT" claude -p "$(cat "$PROMPT_FILE")" \
    --model "$MODEL" \
    --output-format json \
    --dangerously-skip-permissions \
    < /dev/null > "$RESULT_JSON" 2>>"$LOG" )
RC=$?

# capture the session id so a reviewer can resume the exact session on the PR
SID="$("$KUNA_PY" - "$RESULT_JSON" <<'PY' 2>/dev/null
import json,sys
try:
    d=json.load(open(sys.argv[1]))
    print(d.get("session_id") or d.get("sessionId") or "")
except Exception:
    print("")
PY
)"
[ -n "$SID" ] && "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --note "session=$SID" >>"$LOG" 2>&1

if [ $RC -ne 0 ]; then
  log "claude session exited rc=$RC (timeout=124); leaving worktree for inspection"
  "$KUNA_PY" -m kuna.pipeline.state update --worker "$WORKER_ID" --status failed --note "claude rc=$RC" >>"$LOG" 2>&1
  exit $RC
fi

log "session complete (rc=0). worktree $WT and session $SID preserved for PR review."
# the worker itself records `state done --pr <url>` when it opens the PR
exit 0
