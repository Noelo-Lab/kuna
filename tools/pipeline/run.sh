#!/usr/bin/env bash
# The continuous driver loop: keep up to N feature workers running, each implementing one
# angr-inspired kuna feature and opening a PR, until the backlog is exhausted or the time
# budget runs out. Observe live with: python -m scripts.pipeline.status --watch
#
#   PIPELINE_WORKERS=2 PIPELINE_HOURS=6 tools/pipeline/run.sh
#   PIPELINE_WORKERS=1 tools/pipeline/run.sh --once     # do exactly one feature, then stop
#
# Stop gracefully any time: `touch .kuna-pipeline/STOP` (workers in flight finish first),
# or Ctrl-C (same). Nothing ever lands on main without a human-reviewed PR.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
WORKERS="${PIPELINE_WORKERS:-1}"
HOURS="${PIPELINE_HOURS:-0}"            # 0 = no time limit (run until backlog empty)
POLL="${PIPELINE_POLL:-15}"            # seconds between scheduler ticks
ONCE=0
[ "${1:-}" = "--once" ] && ONCE=1

export KUNA_REPO="$REPO" KUNA_PY="$KUNA_PY"
# Run scripts.pipeline.* without an install: repo root on the import path.
export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"
STATE_DIR="$REPO/.kuna-pipeline"
STOP_FILE="$STATE_DIR/STOP"
# logs/ must exist BEFORE spawn_worker, whose `>>"$STATE_DIR/logs/..."` redirect is set up by
# THIS shell -- worker.sh's own mkdir runs in the child, too late. Without this the very first
# spawn into a fresh state dir fails.
mkdir -p "$STATE_DIR/logs" "$STATE_DIR/worktrees"
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
  opp="$("$KUNA_PY" -m scripts.pipeline.select --shell 2>/dev/null)" || return 1
  eval "$opp"   # sets OPP_ID TEST_NAME BINARY SELECTOR ARCH SLUG SCORE KINDS
  SEQ=$((SEQ+1))
  local wid; wid="w$(date +%s)-$SEQ"
  # claim atomically; if already taken (race), skip this tick
  if ! "$KUNA_PY" -m scripts.pipeline.state claim --worker "$wid" --opportunity "$OPP_ID" >/dev/null 2>&1; then
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

log "starting (workers=$WORKERS, hours=${HOURS:-unbounded}, once=$ONCE)"
SPAWNED=0
while :; do
  [ -f "$STOP_FILE" ] && { log "STOP file present"; break; }
  [ "$DEADLINE" != "0" ] && [ "$(date +%s)" -ge "$DEADLINE" ] && { log "time budget reached"; break; }

  gc_merged_worktrees
  # Free the claims of workers that died without saying so; otherwise their opportunities are
  # blocked forever and the backlog silently shrinks.
  "$KUNA_PY" -m scripts.pipeline.state reap >/dev/null 2>&1
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
"$KUNA_PY" -m scripts.pipeline.status || true
