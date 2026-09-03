#!/usr/bin/env bash
# Launch ONE tester: a headless `codex exec` session that tries to solve one crackme with
# kuna as its primary tool and records every place kuna failed it.
#
#   ROUND=3 HEXID=64f1f7afd931496abf909525 tools/repipe/tester.sh
#
# The tester's real job is not to solve the crackme. It is to find ways kuna is bad, and
# giving up because kuna was unusable is a first-class result (docs/re-pipeline.md).
#
# Two things here are load-bearing and easy to get wrong if you edit this file:
#
#   1. CONTAINMENT IS A MOUNT NAMESPACE, NOT A POLICY. codex's `-s workspace-write`
#      restricts writes, not reads, and the dataset leaks the answer four different ways
#      (meta.json carries the plaintext flag; solutions/ holds full writeups; extras/ can
#      hold valid serials on a challenge whose ships_source_code is false; six challenges
#      ship source). So the dataset is tmpfs'd out of the tester's namespace entirely and
#      the arena holds only sanitized copies. A prompt-level "please don't look" is worthless.
#   2. CODEX CANNOT BE TOLD ITS SESSION ID. Unlike `claude --session-id`, the thread id only
#      appears in the first `thread.started` line of the --json stream, so the stream is
#      tee'd and scraped. Do not "simplify" that away.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
STATE_DIR="${KUNA_PIPELINE_STATE_DIR:-$REPO/.kuna-repipe}"
DATASET="${REPIPE_DATASET:-$HOME/github/kuna-re-dataset}"
TIMEOUT="${REPIPE_TESTER_TIMEOUT:-3600}"
MODEL="${REPIPE_TESTER_MODEL:-}"
SANDBOX="${REPIPE_SANDBOX:-auto}"
ENABLE_IDA="${REPIPE_ENABLE_IDA:-1}"

: "${ROUND:?need ROUND}"
: "${HEXID:?need HEXID}"
TESTER_ID="${TESTER_ID:-t-r${ROUND}-${HEXID:0:8}}"
RUN_ID="${RUN_ID:-$TESTER_ID-$(date +%s)}"
ARENA="$STATE_DIR/arena/$ROUND/$HEXID"

export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"
export KUNA_PIPELINE_STATE_DIR="$STATE_DIR"

# The arena's bin/ MUST be first on the tester's PATH: `kuna` and `ida-decompile` there are
# the metering shims. Without this the shims are dead code -- no toolcalls.jsonl, no
# per-call latency signal (kuna emits none of its own), and IDA writes its .i64 databases
# wherever it likes instead of inside the arena.
export PATH="$ARENA/bin:$PATH"
export REPIPE_REAL_KUNA="${REPIPE_REAL_KUNA:-$REPO/decompiler/target/release/kuna}"
export REPIPE_ARENA="$ARENA"

RUN_DIR="$STATE_DIR/runs/$RUN_ID"
LOG_DIR="$STATE_DIR/logs"
mkdir -p "$LOG_DIR" "$RUN_DIR"
LOG="$LOG_DIR/$TESTER_ID.log"
EVENTS="$RUN_DIR/events.jsonl"

log() { echo "[$(date +%H:%M:%S)] tester $TESTER_ID: $*" | tee -a "$LOG"; }

finish() {  # always release the slot, however we exit
  "$KUNA_PY" -m scripts.pipeline.state slot-release --pool tester --id "$TESTER_ID" >/dev/null 2>&1
}
trap finish EXIT

# Ensure the pool has a cap before claiming a slot. The captain sets caps on every tick, but
# tester.sh is also runnable standalone (docs/re-pipeline.md's Level-2 check does exactly
# that), and an unset pool has cap 0 -- which would refuse every slot and look like a bug.
"$KUNA_PY" - <<'PY' >/dev/null 2>&1
from scripts.pipeline import state
from scripts.repipe import config
snap = state.snapshot()
if not (snap.get("slots") or {}).get("tester", {}).get("cap"):
    state.slot_cap("tester", config.agent_split()["testers"])
PY
if ! "$KUNA_PY" -m scripts.pipeline.state slot-acquire --pool tester --id "$TESTER_ID" --pid $$ --kind tester; then
  log "no free tester slot (cap reached); not starting"
  exit 3
fi
"$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --phase setup --status running >/dev/null 2>&1

# --- 1. arena ---------------------------------------------------------------
# REUSE an arena the captain already built. T_WORKSPACE stages every arena for the round and
# verifies it -- that is the whole point of the state -- so rebuilding here is wasted work,
# and worse: the captain's own shim smoke-test writes a toolcalls.jsonl line, which the
# "never clobber tester evidence" guard correctly reads as a run in progress and refuses.
# Careful verification upstream must not look like a failure downstream.
if [ -d "$ARENA/target" ] \
   && "$KUNA_PY" -m scripts.repipe.workspace check "$ARENA" --hexid "$HEXID" >/dev/null 2>&1; then
  log "reusing the arena staged for this round"
else
  log "building arena $ARENA"
  if ! "$KUNA_PY" -m scripts.repipe.workspace build "$HEXID" --round "$ROUND" --force >>"$LOG" 2>&1; then
    log "arena build failed"
    "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status failed --note "arena build failed"
    exit 1
  fi
fi

# The contamination guard must run on the real path, not only in smoke.sh: a bad arena is
# the one failure that quietly poisons a whole round's evidence.
if ! "$KUNA_PY" -m scripts.repipe.workspace check "$ARENA" --hexid "$HEXID" >>"$LOG" 2>&1; then
  log "arena FAILED its contamination check; refusing to launch a tester on it"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status failed --note "arena contaminated"
  exit 1
fi

# --- 2. per-run CODEX_HOME --------------------------------------------------
# ~/.codex/logs_2.sqlite is already 270 MB across 612 rollouts; a 250-challenge sweep would
# inflate it badly and bury this run's transcript. An isolated CODEX_HOME keeps sessions,
# the session index and the sqlite inside the run dir, where harvest reads them and the
# archiver deletes them. NOT --ephemeral: that would destroy the transcript harvest needs.
export CODEX_HOME="$RUN_DIR/codexhome"
mkdir -p "$CODEX_HOME"
# COPY, do not symlink: ~/.codex is tmpfs'd out of the tester's namespace (it holds 600+
# past prompts), so a symlink into it would dangle and codex could not authenticate.
if [ -f "$HOME/.codex/auth.json" ]; then
  cp "$HOME/.codex/auth.json" "$CODEX_HOME/auth.json" && chmod 600 "$CODEX_HOME/auth.json"
fi

# --- 3. render the prompt ---------------------------------------------------
PROMPT_FILE="$RUN_DIR/prompt.md"
"$KUNA_PY" -m scripts.repipe.render_tester_prompt \
  --hexid "$HEXID" --round "$ROUND" --arena "$ARENA" --out "$PROMPT_FILE" >>"$LOG" 2>&1 || {
  log "prompt render failed"; "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status failed --note "prompt render failed"; exit 1
}

# --- 4. launch --------------------------------------------------------------
CODEX_ARGS=(exec
  --cd "$ARENA"
  --skip-git-repo-check
  -s workspace-write
  -c approval_policy=never
  -c sandbox_workspace_write.network_access=false
  --json
  --output-schema "$REPO/tools/repipe/schema/report.schema.json"
  -o "$ARENA/report.json")
[ -n "$MODEL" ] && CODEX_ARGS+=(-m "$MODEL")

# `-a/--ask-for-approval` does NOT exist on `codex exec` (it is interactive-only), which is
# why the approval policy is a `-c` override. network_access=false also removes "search the
# web for this crackme's writeup" as an option, which is a contamination channel bwrap
# cannot close.

SANDBOX_MODE="$SANDBOX"
if [ "$SANDBOX_MODE" = "auto" ]; then
  if command -v bwrap >/dev/null 2>&1; then
    SANDBOX_MODE=bwrap
  else
    # `auto` must NEVER downgrade itself to uncontained. The dataset leaks the answer four
    # ways and $HOME holds credentials; silently proceeding would poison the round's evidence
    # while reporting success. Running without containment has to be an explicit, recorded
    # choice (REPIPE_SANDBOX=none), never an accident of a missing binary.
    log "ERROR: REPIPE_SANDBOX=auto but bwrap is not installed. Refusing to run uncontained."
    log "       Install bwrap, or set REPIPE_SANDBOX=none to accept prompt-only containment."
    "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status failed --note "no bwrap under sandbox=auto"
    exit 1
  fi
fi

if [ "$SANDBOX_MODE" = "bwrap" ]; then
  # Hiding ONE directory is not enough, and assuming it is was a real hole: there is a second
  # full copy of the ground truth (all 250 flags) in the sibling label repo, and $HOME carries
  # SSH keys, a GitHub token and 600+ past codex prompts that a tester has no business reading
  # while it works an adversarial corpus. Everything below is tmpfs'd out of its namespace.
  # REPIPE_SANDBOX_HIDE adds more, colon-separated.
  HIDE=(
    "$DATASET"
    "$HOME/github/crackmes-re-dataset"      # the label repo: crackmes_dataset.jsonl has the flags
    "$HOME/.ssh" "$HOME/.aws" "$HOME/.gnupg"
    "$HOME/.config/gh" "$HOME/.git-credentials"
    "$HOME/.codex" "$HOME/.claude" "$HOME/.anthropic"
  )
  IFS=':' read -r -a EXTRA_HIDE <<< "${REPIPE_SANDBOX_HIDE:-}"
  for h in "${EXTRA_HIDE[@]+"${EXTRA_HIDE[@]}"}"; do [ -n "$h" ] && HIDE+=("$h"); done

  WRAP=(bwrap --dev-bind / /)
  HID=0
  for h in "${HIDE[@]}"; do
    if [ -d "$h" ]; then
      WRAP+=(--tmpfs "$h"); HID=$((HID+1))            # a directory becomes empty
    elif [ -e "$h" ]; then
      WRAP+=(--ro-bind /dev/null "$h"); HID=$((HID+1)) # a FILE cannot be tmpfs'd; blank it
    fi
  done
  # CODEX_HOME lives under the state dir, and $HOME/.codex is hidden above, so re-bind the
  # per-run one explicitly -- codex must still find its auth.
  WRAP+=(--bind "$CODEX_HOME" "$CODEX_HOME" --)
  log "launching under bwrap ($HID path(s) hidden incl. both dataset copies and \$HOME creds), timeout ${TIMEOUT}s"
else
  log "WARNING: no bwrap; dataset is READABLE. Relying on the prompt + the post-hoc tripwire."
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --note "sandbox=none (low trust)" >/dev/null 2>&1
  WRAP=()
fi

"$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --phase solving >/dev/null 2>&1
START=$(date +%s)
"${WRAP[@]+"${WRAP[@]}"}" timeout -k 60 "$TIMEOUT" \
  codex "${CODEX_ARGS[@]}" "$(cat "$PROMPT_FILE")" </dev/null 2>>"$LOG" | tee "$EVENTS" >/dev/null
RC=${PIPESTATUS[0]}
ELAPSED=$(( $(date +%s) - START ))

# --- 5. scrape the thread id + record the attempt ---------------------------
THREAD_ID="$("$KUNA_PY" - "$EVENTS" <<'PY' 2>/dev/null
import json, sys
for line in open(sys.argv[1], errors="replace"):
    try:
        d = json.loads(line)
    except Exception:
        continue
    if d.get("type") == "thread.started" or "thread_id" in d:
        print(d.get("thread_id") or d.get("thread", {}).get("id") or "")
        break
PY
)"

"$KUNA_PY" - "$RUN_DIR/attempt.json" <<PY
import json, sys
json.dump({
    "run_id": "$RUN_ID", "tester_id": "$TESTER_ID", "hexid": "$HEXID",
    "round": int("$ROUND"), "rc": int("$RC"), "elapsed_s": int("$ELAPSED"),
    "timed_out": int("$RC") == 124, "thread_id": "$THREAD_ID",
    "arena": "$ARENA", "events": "$EVENTS", "sandbox": "$SANDBOX_MODE",
    "report": "$ARENA/report.json",
}, open(sys.argv[1], "w"), indent=2)
PY

if [ "$RC" -eq 124 ]; then
  log "timed out after ${TIMEOUT}s (partial report still harvested if present)"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status done --phase timeout --note "timeout ${TIMEOUT}s"
elif [ "$RC" -ne 0 ]; then
  log "codex exited rc=$RC"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status failed --note "codex rc=$RC"
else
  log "done in ${ELAPSED}s (thread $THREAD_ID)"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$TESTER_ID" --status done --phase done
fi
# A timed-out or crashed tester that still filed observations contributed real evidence, so
# harvest runs regardless of rc -- it is the captain's HARVEST state that decides.
exit 0
