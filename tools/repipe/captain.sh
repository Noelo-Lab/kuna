#!/usr/bin/env bash
# One captain tick: a bounded headless agent session that performs ONE state transition.
#
# Invoked by tools/repipe/run.sh. The timeout is the point -- a wedged captain must not stall
# the loop, and because a tick is stateless the cost of killing one is a tick, not a round.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
STATE_DIR="${KUNA_PIPELINE_STATE_DIR:-$REPO/.kuna-repipe}"
TIMEOUT="${REPIPE_CAPTAIN_TIMEOUT:-3600}"
BACKEND="${REPIPE_CAPTAIN_BACKEND:-claude}"
if [ "$BACKEND" = codex ]; then
  MODEL="${REPIPE_CAPTAIN_MODEL:-gpt-5.6-sol}"
else
  MODEL="${REPIPE_CAPTAIN_MODEL:-opus}"
fi
REASONING="${REPIPE_CAPTAIN_REASONING:-low}"
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

# Keep --max-agents honest: every worker must come from a pipeline slot. Claude removes its
# Task tool; Codex disables the multi-agent feature that provides its collaboration tools.
# Codex also runs ephemerally so bounded ticks do not accumulate unrelated session history.
case "$BACKEND" in
  claude)
    # env -u ANTHROPIC_API_KEY: a stale key makes headless `claude -p` fail "Invalid API key".
    cd "$REPO" && env -u ANTHROPIC_API_KEY timeout -k 60 "$TIMEOUT" \
      claude -p "$PROMPT" \
        --model "$MODEL" \
        --output-format json \
        --dangerously-skip-permissions \
        --disallowedTools Task \
        --max-budget-usd "$BUDGET" \
        </dev/null >>"$LOG" 2>&1
    ;;
  codex)
    cd "$REPO" && timeout -k 60 "$TIMEOUT" \
      codex exec \
        --cd "$REPO" \
        --sandbox danger-full-access \
        -c approval_policy=never \
        -c "model_reasoning_effort=$REASONING" \
        --model "$MODEL" \
        --disable multi_agent \
        --disable multi_agent_v2 \
        --ephemeral \
        --json \
        "$PROMPT" \
        </dev/null >>"$LOG" 2>&1
    ;;
  *)
    echo "unknown REPIPE_CAPTAIN_BACKEND: $BACKEND (expected claude or codex)" >>"$LOG"
    exit 2
    ;;
esac
RC=$?
[ $RC -eq 124 ] && echo "[$(date +%H:%M:%S)] captain tick timed out after ${TIMEOUT}s" >>"$LOG"
exit 0
