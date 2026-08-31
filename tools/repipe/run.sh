#!/usr/bin/env bash
# The RE-friction loop's supervisor: keep the captain alive, enforce the budget, drain cleanly.
#
#   tools/repipe/run.sh                 # bounded run (REPIPE_ROUNDS, default 3)
#   tools/repipe/run.sh --once          # exactly one full cycle
#   tools/repipe/run.sh --preflight     # check the machine and exit
#
#   touch .kuna-repipe/STOP     graceful drain: in-flight agents finish, INTEGRATE still runs
#   touch .kuna-repipe/PAUSE    finish what is running, spawn nothing new
#   touch .kuna-repipe/ABORT    hard stop: SIGTERM recorded pids, leave every worktree intact
#
# This process is deliberately dumb. It does not know what a round is; it re-invokes the
# captain, which performs at most one guarded state transition per tick and exits. That split
# is what makes a crash cost one tick instead of a run: kill this at any moment and the next
# invocation resumes from the recorded state.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
STATE_DIR="$REPO/${REPIPE_STATE_DIRNAME:-.kuna-repipe}"
POLL="${REPIPE_POLL:-15}"
ROUNDS="${REPIPE_ROUNDS:-3}"
HOURS="${REPIPE_HOURS:-0}"
CAPTAIN_TIMEOUT="${REPIPE_CAPTAIN_TIMEOUT:-1200}"
ONCE=0

for a in "$@"; do
  case "$a" in
    --once) ONCE=1; ROUNDS=1;;
    --preflight) PREFLIGHT_ONLY=1;;
    *) echo "unknown flag: $a" 1>&2; exit 2;;
  esac
done

export KUNA_REPO="$REPO" KUNA_PY="$KUNA_PY"
export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"
export KUNA_PIPELINE_STATE_DIR="$STATE_DIR"
export REPIPE_STATE_DIRNAME="${REPIPE_STATE_DIRNAME:-.kuna-repipe}"
mkdir -p "$STATE_DIR/logs" "$STATE_DIR/rounds" "$STATE_DIR/arena" "$STATE_DIR/runs" "$STATE_DIR/worktrees"
rm -f "$STATE_DIR/STOP" "$STATE_DIR/ABORT"

log() { echo "[$(date +%H:%M:%S)] supervisor: $*" | tee -a "$STATE_DIR/logs/supervisor.log"; }

if ! "$KUNA_PY" -m scripts.repipe.captain --preflight; then
  log "preflight failed; refusing to start"
  exit 1
fi
[ "${PREFLIGHT_ONLY:-0}" = 1 ] && exit 0

trap 'log "signal received -> graceful stop"; touch "$STATE_DIR/STOP"' INT TERM

START=$(date +%s)
DEADLINE=0
[ "$HOURS" != "0" ] && DEADLINE=$(( START + HOURS * 3600 ))

log "starting (rounds=$ROUNDS hours=${HOURS:-unbounded} once=$ONCE)"
log "watch it: $KUNA_PY -m scripts.repipe.webui --port 8787"

TICKS=0
while :; do
  if [ -f "$STATE_DIR/ABORT" ]; then
    log "ABORT: terminating recorded agent pids; worktrees and arenas left intact"
    "$KUNA_PY" - <<'PY'
import json, os, signal
from scripts.repipe import config
inv = config.state_dir() / "inventory.json"
if inv.exists():
    d = json.load(open(inv))
    for pool in (d.get("slots") or {}).values():
        for sid, held in (pool.get("held") or {}).items():
            pid = held.get("pid")
            try:
                os.killpg(os.getpgid(int(pid)), signal.SIGTERM)
                print("SIGTERM %s (pid %s)" % (sid, pid))
            except Exception:
                pass
PY
    break
  fi

  [ "$DEADLINE" != "0" ] && [ "$(date +%s)" -ge "$DEADLINE" ] && { log "time budget reached"; touch "$STATE_DIR/STOP"; }

  # A tick is TWO things and both are required:
  #   1. the deterministic housekeeping tick -- reap, preflight, disk/budget brakes, the
  #      supervisor's own BOOT/RUNNING/DRAINING edges. Pure Python, no LLM.
  #   2. the LLM captain session, which is what actually advances TestTrack and BuildTrack:
  #      picking a slate, spawning testers, gating, clustering, judging proposals.
  # Running only (1) leaves the loop inert -- it would tick forever at T_IDLE/B_IDLE.
  OUT="$("$KUNA_PY" -m scripts.repipe.captain --tick 2>&1)"
  RC=$?
  TICKS=$((TICKS+1))
  printf '%s\n' "$OUT" >> "$STATE_DIR/logs/captain.log"
  if [ $RC -ne 0 ]; then
    log "captain housekeeping tick failed (rc=$RC); see $STATE_DIR/logs/captain.log"
  fi

  if [ -f "$STATE_DIR/PAUSE" ]; then
    log "PAUSE present: not starting a captain session"
  elif [ -f "$STATE_DIR/STOP" ]; then
    : # draining; the loop below will exit
  else
    log "captain session (tick $TICKS)"
    bash "$REPO/tools/repipe/captain.sh" >>"$STATE_DIR/logs/captain.log" 2>&1
    CRC=$?
    [ $CRC -ne 0 ] && log "captain session rc=$CRC"
  fi

  SUP="$("$KUNA_PY" -m scripts.repipe.captain --status 2>/dev/null | "$KUNA_PY" -c 'import json,sys; print(json.load(sys.stdin)["states"]["supervisor"])' 2>/dev/null)"
  case "$SUP" in
    HALTED)  log "captain HALTED: $(cat "$STATE_DIR/HALT_REASON" 2>/dev/null | head -3)"; break;;
    STOPPED) log "captain STOPPED"; break;;
  esac

  # The captain drives the ROUNDS budget; this is only the outer safety net.
  CUR="$("$KUNA_PY" -m scripts.repipe.captain --status 2>/dev/null | "$KUNA_PY" -c 'import json,sys; print(json.load(sys.stdin)["round"])' 2>/dev/null)"
  if [ "$ROUNDS" != "0" ] && [ -n "$CUR" ] && [ "$CUR" -gt "$ROUNDS" ]; then
    log "round budget ($ROUNDS) exhausted"; touch "$STATE_DIR/STOP"
  fi

  sleep "$POLL"
done

log "draining: waiting for in-flight agents"
for _ in $(seq 1 120); do
  N="$("$KUNA_PY" -c 'import json,sys
from scripts.repipe import config
p = config.state_dir()/"inventory.json"
d = json.load(open(p)) if p.exists() else {}
print(sum(len((v.get("held") or {})) for k,v in (d.get("slots") or {}).items() if k!="captain"))' 2>/dev/null)"
  [ "${N:-0}" = "0" ] && break
  sleep 10
done
"$KUNA_PY" -m scripts.pipeline.state reap --stale-seconds 0 >/dev/null 2>&1
log "done after $TICKS tick(s)."
"$KUNA_PY" -m scripts.repipe.captain --status || true
