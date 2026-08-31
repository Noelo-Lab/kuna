#!/usr/bin/env bash
# Launch ONE feature worker: an isolated git worktree + a headless `claude -p` session
# that implements one angr-inspired kuna feature end-to-end and opens a PR.
#
# Invoked by tools/pipeline/run.sh, but runnable standalone for a single iteration:
#   WORKER_ID=w1 OPP_ID='test_x::main' TEST_NAME=test_x SELECTOR=main \
#   BINARY=/path/bin SLUG=myfeat-ab12cd ARCH= tools/pipeline/worker.sh
#
# The worktree and the Claude session transcript are PRESERVED after the run so a human
# reviewer can resume the session on the PR (`claude --resume <session_id>` in the worktree).
# tools/pipeline/run.sh garbage-collects a worktree only once its PR is merged/closed.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
MODEL="${WORKER_MODEL:-opus}"
WORKER_TIMEOUT="${WORKER_TIMEOUT:-7200}"   # seconds; hard cap per feature
BASE_BRANCH="${BASE_BRANCH:-main}"

# Seams so a second pipeline reuses this driver without a fork (docs/re-pipeline.md).
# Unset == the angr fleet's behaviour, byte for byte.
PROMPT_TMPL="${WORKER_PROMPT:-$REPO/tools/pipeline/worker_prompt.md}"
BRANCH_PREFIX="${WORKER_BRANCH_PREFIX:-feat/angr-}"
STATE_DIRNAME="${PIPELINE_STATE_DIRNAME:-.kuna-pipeline}"
WORKER_BUDGET_USD="${WORKER_BUDGET_USD:-}"      # empty = no --max-budget-usd flag
WORKER_SESSION_ID="${WORKER_SESSION_ID:-}"      # empty = let claude allocate one
WORKER_EXTRA_PROMPT="${WORKER_EXTRA_PROMPT:-}"  # file spliced in at {{SIBLINGS}}

: "${WORKER_ID:?need WORKER_ID}"
: "${OPP_ID:?need OPP_ID}"
: "${TEST_NAME:?need TEST_NAME}"
: "${SELECTOR:?need SELECTOR}"
: "${BINARY:?need BINARY}"
: "${SLUG:?need SLUG}"
ARCH="${ARCH:-}"

# Run the Python pipeline (scripts.pipeline.*) from the main tree so the `scripts`
# package is importable without an install (no pyproject/pip install -e).
export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"

# Keep ALL pipeline state in the MAIN tree so `scripts.pipeline.status` (run from the main
# tree) sees this worker's heartbeats. Without this, the worktree's KUNA_ROOT would route
# state into the worktree's own .kuna-pipeline and the main-tree status would go stale.
export KUNA_PIPELINE_STATE_DIR="${KUNA_PIPELINE_STATE_DIR:-$REPO/$STATE_DIRNAME}"

# Normal mode opens a fresh branch off main. Implementation mode (IMPL_PROPOSAL=1) resumes
# an APPROVED proposal's existing branch (RESUME_BRANCH) instead of branching fresh.
IMPL_PROPOSAL="${IMPL_PROPOSAL:-0}"
RESUME_BRANCH="${RESUME_BRANCH:-}"
if [ "$IMPL_PROPOSAL" = 1 ] && [ -n "$RESUME_BRANCH" ]; then
  BRANCH="$RESUME_BRANCH"
else
  BRANCH="${BRANCH_PREFIX}${SLUG}"
fi
WT_ROOT="$REPO/$STATE_DIRNAME/worktrees"
WT="$WT_ROOT/$WORKER_ID"
LOG_DIR="$REPO/$STATE_DIRNAME/logs"
DATE="$(date +%Y-%m-%d)"

mkdir -p "$WT_ROOT" "$LOG_DIR"
LOG="$LOG_DIR/$WORKER_ID.log"

log() { echo "[$(date +%H:%M:%S)] worker $WORKER_ID: $*" | tee -a "$LOG"; }

# --- 1. isolated worktree (fresh branch, or an approved proposal's branch) --
if [ "$IMPL_PROPOSAL" = 1 ]; then
  log "resuming approved proposal on existing branch $BRANCH"
  git -C "$REPO" worktree add "$WT" "$BRANCH" >>"$LOG" 2>&1 || {
    "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "resume worktree add failed"
    exit 1
  }
else
  log "creating worktree $WT on $BRANCH (base $BASE_BRANCH)"
  git -C "$REPO" worktree add -b "$BRANCH" "$WT" "$BASE_BRANCH" >>"$LOG" 2>&1 || {
    log "worktree add failed (branch may exist); trying detached reuse"
    git -C "$REPO" worktree add "$WT" "$BASE_BRANCH" >>"$LOG" 2>&1 || {
      "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "worktree add failed"
      exit 1
    }
  }
fi

# --pid $$ is THIS driver's pid, which lives as long as the worker does. Without it the
# inventory records the ephemeral `state register` process and reap() cannot tell a live
# worker from a dead one.
"$KUNA_PY" -m scripts.pipeline.state register --worker "$WORKER_ID" --slug "$SLUG" \
  --branch "$BRANCH" --opportunity "$OPP_ID" --pid $$ >>"$LOG" 2>&1

# --- 2. env: kuna tooling targets THIS worktree; specs reuse the main .sla --
export KUNA_ROOT="$WT"
export KUNA_DECOMP_DBG="$WT/decompiler/target/release/decomp_dbg"
export KUNA_DECOMP_TEST="$WT/decompiler/target/release/decomp_test_dbg"
export KUNA_SPECS="$REPO/specs"          # reuse the main tree's already-compiled .sla
export SLEIGHHOME="$REPO/specs"
export KUNA_PIPELINE_ANGR_PYTHON="${KUNA_PIPELINE_ANGR_PYTHON:-$HOME/.virtualenvs/decbench/bin/python}"

# docs/agents.md's worktree hygiene, enforced here rather than left to the prompt -- the
# prompt did not carry it, and both failures below have actually happened on this machine.
#
# 1. The default debug profile costs 20-30 GB per worktree and has filled the disk mid-run,
#    so debug info is off and target/debug is removed on EVERY exit path (success, timeout,
#    crash) via the trap -- not only on the happy path.
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cleanup_target() { rm -rf "$WT/decompiler/target/debug" 2>/dev/null; }
trap cleanup_target EXIT
# 2. KUNA_SPECS/SLEIGHHOME do NOT reach the cargo workspace suite: ~22 targets resolve
#    <repo>/specs relative to their own crate and fail with "Could not find .sla file". The
#    .sla are gitignored artifacts inside the TRACKED specs/Ghidra tree, so they are linked
#    file by file -- symlinking the directory just nests a dead link inside it, and
#    `make specs` must never run in a worktree.
link_specs() {
  local n=0 rel
  while IFS= read -r sla; do
    rel="${sla#$REPO/specs/}"
    mkdir -p "$WT/specs/$(dirname "$rel")"
    ln -sf "$sla" "$WT/specs/$rel" && n=$((n+1))
  done < <(find "$REPO/specs" -name '*.sla' -type f 2>/dev/null)
  echo "$n"
}
SLA_N="$(link_specs)"
log "linked $SLA_N compiled .sla into the worktree"
[ "$SLA_N" = "0" ] && log "WARNING: no .sla in $REPO/specs -- run \`make specs\` in the MAIN tree first"

# --- 3. initial build so the worker's analyze phase has working binaries ----
log "building binaries in worktree (initial)"
"$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --phase setup >>"$LOG" 2>&1
( cd "$WT" && make binaries ) >>"$LOG" 2>&1 || {
  log "initial build failed"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "initial build failed"
  exit 1
}

# --- 4. render the worker prompt -------------------------------------------
# {{RESUME_PROPOSAL}} is a single-line directive: empty for a normal feature, or the
# implementation-mode preamble when resuming an approved proposal.
if [ "$IMPL_PROPOSAL" = 1 ]; then
  RESUME_PROPOSAL_LINE="IMPLEMENTATION MODE — this branch has an APPROVED docs/features/$SLUG/proposal.md. Read it and implement that plan starting at Phase 3 (skip analyze/design — they are approved). At Phase 8, after the PR is updated, mark it ready: tools/pipeline/open_pr.sh --undraft $BRANCH"
else
  RESUME_PROPOSAL_LINE=""
fi
PROMPT_FILE="$LOG_DIR/$WORKER_ID.prompt.md"
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
  -e "s|{{RESUME_PROPOSAL}}|$RESUME_PROPOSAL_LINE|g" \
  "$PROMPT_TMPL" > "$PROMPT_FILE"

# {{SIBLINGS}} is a whole FILE, not a line, so it is spliced after the sed pass: the captain
# writes what the other in-flight builders are doing so they stay off each other's files.
if [ -n "$WORKER_EXTRA_PROMPT" ] && [ -f "$WORKER_EXTRA_PROMPT" ]; then
  "$KUNA_PY" -c 'import sys; t,e=sys.argv[1],sys.argv[2]; b=open(t).read(); open(t,"w").write(b.replace("{{SIBLINGS}}", open(e).read()))' "$PROMPT_FILE" "$WORKER_EXTRA_PROMPT"
else
  sed -i 's|{{SIBLINGS}}||g' "$PROMPT_FILE"
fi

# --- 5. headless Claude session (highest-effort model), in the worktree -----
log "launching claude -p (model $MODEL, timeout ${WORKER_TIMEOUT}s)"
"$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --phase analyze >>"$LOG" 2>&1

RESULT_JSON="$LOG_DIR/$WORKER_ID.result.json"
CLAUDE_EXTRA=()
# A caller-supplied session id makes the transcript path deterministic BEFORE the run, which
# is what lets a harness tail a live builder. codex has no equivalent.
[ -n "$WORKER_SESSION_ID" ] && CLAUDE_EXTRA+=(--session-id "$WORKER_SESSION_ID")
[ -n "$WORKER_BUDGET_USD" ] && CLAUDE_EXTRA+=(--max-budget-usd "$WORKER_BUDGET_USD")
# env -u ANTHROPIC_API_KEY: a stale/invalid ANTHROPIC_API_KEY in the environment makes
# headless `claude -p` fail with "Invalid API key"; unsetting it falls back to the working
# session auth. </dev/null so claude does not wait on stdin.
( cd "$WT" && env -u ANTHROPIC_API_KEY timeout "$WORKER_TIMEOUT" claude -p "$(cat "$PROMPT_FILE")" \
    --model "$MODEL" \
    --output-format json \
    --dangerously-skip-permissions \
    "${CLAUDE_EXTRA[@]+"${CLAUDE_EXTRA[@]}"}" \
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
[ -n "$SID" ] && "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --note "session=$SID" >>"$LOG" 2>&1

if [ $RC -ne 0 ]; then
  log "claude session exited rc=$RC (timeout=124); leaving worktree for inspection"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "claude rc=$RC" >>"$LOG" 2>&1
  exit $RC
fi

log "session complete (rc=0). worktree $WT and session $SID preserved for PR review."
# the worker itself records `state done --pr <url>` when it opens the PR
exit 0
