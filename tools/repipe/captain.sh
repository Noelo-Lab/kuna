#!/usr/bin/env bash
# One captain tick: a bounded headless Claude session that performs ONE state transition.
#
# Invoked by tools/repipe/run.sh. The timeout is the point -- a wedged captain must not stall
# the loop, and because a tick is stateless the cost of killing one is a tick, not a round.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
STATE_DIR="${KUNA_PIPELINE_STATE_DIR:-$REPO/.kuna-repipe}"
TIMEOUT="${REPIPE_CAPTAIN_TIMEOUT:-1200}"
MODEL="${REPIPE_CAPTAIN_MODEL:-opus}"
BUDGET="${REPIPE_CAPTAIN_USD:-5}"

export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"
export KUNA_PIPELINE_STATE_DIR="$STATE_DIR"
mkdir -p "$STATE_DIR/logs"
LOG="$STATE_DIR/logs/captain-session.log"
TICK_ID="cap-$(date +%s)"

"$KUNA_PY" -m scripts.pipeline.state slot-acquire --pool captain --id "$TICK_ID" --pid $$ --kind captain >/dev/null 2>&1 || {
  echo "another captain tick holds the slot; skipping" | tee -a "$LOG"; exit 0; }
trap '"$KUNA_PY" -m scripts.pipeline.state slot-release --pool captain --id "$TICK_ID" >/dev/null 2>&1' EXIT

STATUS="$("$KUNA_PY" -m scripts.repipe.captain --status 2>&1)"
PROMPT="$(cat "$REPO/tools/repipe/captain_prompt.md")

## Current state (read at $(date -Iseconds))

\`\`\`json
$STATUS
\`\`\`
"

# --disallowedTools Task keeps --max-agents honest: every agent must come from a slot, and a
# captain that could fork subagents freely would make the cap a fiction.
# env -u ANTHROPIC_API_KEY: a stale key makes headless `claude -p` fail "Invalid API key".
cd "$REPO" && env -u ANTHROPIC_API_KEY timeout -k 60 "$TIMEOUT" \
  claude -p "$PROMPT" \
    --model "$MODEL" \
    --output-format json \
    --dangerously-skip-permissions \
    --disallowedTools Task \
    --max-budget-usd "$BUDGET" \
    </dev/null >>"$LOG" 2>&1
RC=$?
[ $RC -eq 124 ] && echo "[$(date +%H:%M:%S)] captain tick timed out after ${TIMEOUT}s" >>"$LOG"
exit 0
