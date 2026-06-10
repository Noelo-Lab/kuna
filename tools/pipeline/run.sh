#!/usr/bin/env bash
# The continuous driver loop: keep up to N feature workers running, each implementing one
# angr-inspired kuna feature and opening a PR, until the backlog is exhausted or the time
# budget runs out. Observe live with: python -m kuna.pipeline.status --watch
#
#   PIPELINE_WORKERS=2 PIPELINE_HOURS=6 tools/pipeline/run.sh
#   PIPELINE_WORKERS=1 tools/pipeline/run.sh --once     # do exactly one feature, then stop
#
# PIPELINE_MODE=port switches the backlog to the Rust-port checklist
# (docs/rust-port/checklist.json via kuna.pipeline.select_port); workers then run the
# porter/verifier prompts on rport/<item> branches off rust-port. Default ("feature")
# behavior is unchanged. See docs/pipeline.md "Port mode".
#
#   PIPELINE_MODE=port PIPELINE_WORKERS=2 PIPELINE_HOURS=6 tools/pipeline/run.sh
#
# Stop gracefully any time: `touch .kuna-pipeline/STOP` (workers in flight finish first),
# or Ctrl-C (same). Nothing ever lands on main without a human-reviewed PR.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
MODE="${PIPELINE_MODE:-feature}"       # "feature" (default, unchanged) | "port"
WORKERS="${PIPELINE_WORKERS:-1}"
HOURS="${PIPELINE_HOURS:-0}"            # 0 = no time limit (run until backlog empty)
POLL="${PIPELINE_POLL:-15}"            # seconds between scheduler ticks
ONCE=0
[ "${1:-}" = "--once" ] && ONCE=1

export KUNA_REPO="$REPO" KUNA_PY="$KUNA_PY"
STATE_DIR="$REPO/.kuna-pipeline"
STOP_FILE="$STATE_DIR/STOP"
mkdir -p "$STATE_DIR"
rm -f "$STOP_FILE"

START=$(date +%s)
DEADLINE=0
[ "$HOURS" != "0" ] && DEADLINE=$(( START + $(printf '%.0f' "$(echo "$HOURS*3600" | bc)") ))

declare -A WPID   # worker_id -> pid
SEQ=0

log() { echo "[$(date +%H:%M:%S)] driver: $*"; }

trap 'log "signal received -> graceful stop"; touch "$STOP_FILE"' INT TERM

active_count() {
  local n=0 wid
  for wid in "${!WPID[@]}"; do
    if kill -0 "${WPID[$wid]}" 2>/dev/null; then n=$((n+1)); else unset 'WPID[$wid]'; fi
  done
  echo "$n"
}

gc_merged_worktrees() {
  # remove worktrees whose feature PR is merged or closed (keep open-PR worktrees for review)
  command -v gh >/dev/null 2>&1 || return 0
  local wt branch state
  while read -r wt; do
    [ -d "$wt" ] || continue
    branch="$(git -C "$wt" symbolic-ref --short HEAD 2>/dev/null)"
    [ -n "$branch" ] || continue
    state="$(gh pr view "$branch" --repo Noelo-Lab/kuna --json state -q .state 2>/dev/null)"
    if [ "$state" = "MERGED" ] || [ "$state" = "CLOSED" ]; then
      log "GC worktree $wt (PR $state)"
      git -C "$REPO" worktree remove --force "$wt" 2>/dev/null
      git -C "$REPO" branch -D "$branch" 2>/dev/null
    fi
  done < <(git -C "$REPO" worktree list --porcelain | awk '/^worktree /{print $2}' | grep "/.kuna-pipeline/worktrees/")
  git -C "$REPO" worktree prune 2>/dev/null
}

spawn_worker() {
  local opp
  if [ "$MODE" = "port" ]; then
    opp="$("$KUNA_PY" -m kuna.pipeline.select_port --shell 2>/dev/null)" || return 1
    eval "$opp"   # sets ITEM_ID KIND CRATE MODULES GATE SLUG TARGET_BRANCH
    SEQ=$((SEQ+1))
    local wid; wid="w$(date +%s)-$SEQ"
    # claim atomically (keyed by checklist item id); if already taken (race), skip this tick
    if ! "$KUNA_PY" -m kuna.pipeline.state claim --worker "$wid" --opportunity "$ITEM_ID" >/dev/null 2>&1; then
      log "item $ITEM_ID already claimed; skipping"
      return 1
    fi
    log "spawning $wid for port item $ITEM_ID (kind $KIND, crate $CRATE, target ${TARGET_BRANCH:-n/a})"
    WORKER_ID="$wid" PIPELINE_MODE=port ITEM_ID="$ITEM_ID" KIND="$KIND" CRATE="$CRATE" \
      MODULES="$MODULES" GATE="$GATE" SLUG="$SLUG" TARGET_BRANCH="$TARGET_BRANCH" \
      bash "$REPO/tools/pipeline/worker.sh" >>"$STATE_DIR/logs/driver-$wid.log" 2>&1 &
    WPID["$wid"]=$!
    return 0
  fi
  opp="$("$KUNA_PY" -m kuna.pipeline.select --shell 2>/dev/null)" || return 1
  eval "$opp"   # sets OPP_ID TEST_NAME BINARY SELECTOR ARCH SLUG SCORE KINDS
  SEQ=$((SEQ+1))
  local wid; wid="w$(date +%s)-$SEQ"
  # claim atomically; if already taken (race), skip this tick
  if ! "$KUNA_PY" -m kuna.pipeline.state claim --worker "$wid" --opportunity "$OPP_ID" >/dev/null 2>&1; then
    log "opportunity $OPP_ID already claimed; skipping"
    return 1
  fi
  log "spawning $wid for [$SCORE] $OPP_ID (slug $SLUG, kinds $KINDS)"
  WORKER_ID="$wid" OPP_ID="$OPP_ID" TEST_NAME="$TEST_NAME" SELECTOR="$SELECTOR" \
    BINARY="$BINARY" SLUG="$SLUG" ARCH="$ARCH" \
    bash "$REPO/tools/pipeline/worker.sh" >>"$STATE_DIR/logs/driver-$wid.log" 2>&1 &
  WPID["$wid"]=$!
  return 0
}

[ "$MODE" = "port" ] && log "mode=port (Rust-port checklist backlog; base branch rust-port)"
log "starting (workers=$WORKERS, hours=${HOURS:-unbounded}, once=$ONCE)"
SPAWNED=0
while :; do
  [ -f "$STOP_FILE" ] && { log "STOP file present"; break; }
  [ "$DEADLINE" != "0" ] && [ "$(date +%s)" -ge "$DEADLINE" ] && { log "time budget reached"; break; }

  gc_merged_worktrees
  n="$(active_count)"
  if [ "$n" -lt "$WORKERS" ]; then
    if spawn_worker; then
      SPAWNED=$((SPAWNED+1))
      [ "$ONCE" = "1" ] && { log "--once: spawned one worker, will wait for it"; break; }
    else
      # nothing to spawn; if no active workers either, the backlog is drained
      [ "$(active_count)" = "0" ] && { log "backlog drained and no active workers"; break; }
    fi
  fi
  sleep "$POLL"
done

log "waiting for in-flight workers to finish"
for wid in "${!WPID[@]}"; do wait "${WPID[$wid]}" 2>/dev/null; done
log "done. $SPAWNED worker(s) launched this run."
"$KUNA_PY" -m kuna.pipeline.status || true
