#!/usr/bin/env bash
# Launch ONE feature worker: an isolated git worktree + a headless agent session
# that implements one angr-inspired kuna feature end-to-end and opens a PR.
#
# Invoked by tools/pipeline/run.sh, but runnable standalone for a single iteration:
#   WORKER_ID=w1 OPP_ID='test_x::main' TEST_NAME=test_x SELECTOR=main \
#   BINARY=/path/bin SLUG=myfeat-ab12cd ARCH= tools/pipeline/worker.sh
#
# The worktree and the agent transcript are PRESERVED after the run. Claude sessions can be
# resumed directly; Codex runs also retain their JSONL event stream and thread id.
# tools/pipeline/run.sh garbage-collects a worktree only once its PR is merged/closed.
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
KUNA_PY="${KUNA_PY:-$HOME/.virtualenvs/kuna/bin/python}"
BACKEND="${WORKER_BACKEND:-claude}"
if [ "$BACKEND" = codex ]; then
  MODEL="${WORKER_MODEL:-gpt-5.6-sol}"
  REASONING="${WORKER_REASONING:-high}"
else
  MODEL="${WORKER_MODEL:-opus}"
  REASONING="${WORKER_REASONING:-}"
fi
WORKER_TIMEOUT="${WORKER_TIMEOUT:-7200}"   # seconds; hard cap per feature
BASE_BRANCH="${BASE_BRANCH:-main}"

: "${WORKER_ID:?need WORKER_ID}"
: "${OPP_ID:?need OPP_ID}"
: "${TEST_NAME:?need TEST_NAME}"
: "${SELECTOR:?need SELECTOR}"
: "${BINARY:?need BINARY}"
: "${SLUG:?need SLUG}"
ARCH="${ARCH:-}"

case "$BACKEND" in
  claude)
    WORKER_AGENT="Claude Code worker using $MODEL"
    WORKER_COMMAND="claude -p <this entire prompt>"
    WORKER_DECIDER_INSTRUCTIONS='For any genuine judgment call (which stage to hook, whether the construct generalizes, scope), spawn a **decider subagent** (use the Task/Agent tool) to make and justify the call, and record its decision verbatim in `docs/features/'"$SLUG"'/record.json` under `"decisions"`. Write `docs/features/'"$SLUG"'/plan.md`.
- **Scope check (the proposal gate, Hard rule 7).** Ask the decider to return `scope: small|large`.'
    WORKER_TRAILER="Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
    WORKER_GENERATED_WITH='🤖 Generated with [Claude Code](https://claude.com/claude-code)'
    ;;
  codex)
    if [ "$MODEL" != "gpt-5.6-sol" ]; then
      echo "worker $WORKER_ID: Codex implementation requires model gpt-5.6-sol (got '$MODEL')" >&2
      exit 2
    fi
    case "$REASONING" in
      high|xhigh|max) ;;
      *) echo "worker $WORKER_ID: Codex implementation requires high, xhigh, or max reasoning (got '$REASONING')" >&2; exit 2 ;;
    esac
    WORKER_AGENT="Codex implementation worker using $MODEL with $REASONING reasoning"
    WORKER_COMMAND="codex exec <this entire prompt>"
    WORKER_DECIDER_INSTRUCTIONS='Codex multi-agent features are disabled so every agent process remains accounted for by a pipeline slot. Make and justify each judgment call yourself, record the decision verbatim in `docs/features/'"$SLUG"'/record.json` under `"decisions"`, and write `docs/features/'"$SLUG"'/plan.md`.
- **Scope check (the proposal gate, Hard rule 7).** Return your own explicit `scope: small|large` decision.'
    WORKER_TRAILER="Co-Authored-By: OpenAI Codex <noreply@openai.com>"
    WORKER_GENERATED_WITH='🤖 Generated with [OpenAI Codex](https://openai.com/codex/)'
    ;;
  *)
    echo "worker $WORKER_ID: unknown WORKER_BACKEND '$BACKEND' (expected claude or codex)" >&2
    exit 2
    ;;
esac

# Seams so a second pipeline reuses this driver without a fork (docs/re-pipeline.md).
# Unset == the angr fleet's behaviour, byte for byte.
PROMPT_TMPL="${WORKER_PROMPT:-$REPO/tools/pipeline/worker_prompt.md}"
BRANCH_PREFIX="${WORKER_BRANCH_PREFIX:-feat/angr-}"
STATE_DIRNAME="${PIPELINE_STATE_DIRNAME:-.kuna-pipeline}"
WORKER_BUDGET_USD="${WORKER_BUDGET_USD:-}"      # empty = no --max-budget-usd flag
WORKER_SESSION_ID="${WORKER_SESSION_ID:-}"      # empty = let claude allocate one
WORKER_EXTRA_PROMPT="${WORKER_EXTRA_PROMPT:-}"  # file spliced in at {{SIBLINGS}}
WORKER_BRANCH="${WORKER_BRANCH:-}"              # empty = BRANCH_PREFIX + SLUG
WORKER_PREPARE_ONLY="${WORKER_PREPARE_ONLY:-0}" # test seam: stop after worktree setup

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
if [ "$IMPL_PROPOSAL" = 1 ] && [ -z "$RESUME_BRANCH" ]; then
  echo "worker $WORKER_ID: IMPL_PROPOSAL=1 requires RESUME_BRANCH" >&2
  exit 2
fi
if [ "$IMPL_PROPOSAL" = 1 ]; then
  BRANCH="$RESUME_BRANCH"
elif [ -n "$WORKER_BRANCH" ]; then
  BRANCH="$WORKER_BRANCH"
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

git_common_dir() {
  local root="$1" raw
  raw="$(git -C "$root" rev-parse --git-common-dir 2>/dev/null)" || return 1
  case "$raw" in
    /*) (cd "$raw" 2>/dev/null && pwd -P) ;;
    *)  (cd "$root/$raw" 2>/dev/null && pwd -P) ;;
  esac
}

# --- 1. isolated worktree (fresh branch, or an approved proposal's branch) --
if [ "$IMPL_PROPOSAL" = 1 ]; then
  log "resuming approved proposal on existing branch $BRANCH"
  git -C "$REPO" worktree add "$WT" "$BRANCH" >>"$LOG" 2>&1 || {
    "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "resume worktree add failed"
    exit 1
  }
elif [ -d "$WT" ] \
     && [ "$(git -C "$WT" rev-parse --show-toplevel 2>/dev/null)" = "$WT" ] \
     && [ "$(git_common_dir "$WT")" = "$(git_common_dir "$REPO")" ] \
     && [ "$(git -C "$WT" symbolic-ref --quiet --short HEAD 2>/dev/null)" = "$BRANCH" ]; then
  # A killed builder leaves its worktree behind on purpose ("for inspection"), and
  # `worktree add` FAILS on an existing path -- so before this arm, the second attempt at a
  # need could not even start, let alone resume the first attempt's work. Reuse it.
  #
  # Only this exact path+branch case is reusable. Matching the branch anywhere in `worktree
  # list` is insufficient: the expected path may be a different, dirty worktree.
  log "reusing the existing worktree $WT, already on $BRANCH"
elif [ -e "$WT" ]; then
  log "refusing to replace ambiguous existing path $WT; inspect it and choose explicit recovery"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "ambiguous worktree path exists" >>"$LOG" 2>&1
  exit 1
elif git -C "$REPO" show-ref --verify --quiet "refs/heads/$BRANCH"; then
  log "refusing to reuse ambiguous existing branch $BRANCH without its exact worktree; set IMPL_PROPOSAL=1 and RESUME_BRANCH to resume it, or WORKER_BRANCH to create a fresh branch"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "ambiguous branch exists without worktree" >>"$LOG" 2>&1
  exit 1
else
  log "creating worktree $WT on $BRANCH (base $BASE_BRANCH)"
  git -C "$REPO" worktree add -b "$BRANCH" "$WT" "$BASE_BRANCH" >>"$LOG" 2>&1 || {
    log "worktree add failed; preserving every existing path and ref"
    "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "worktree add failed" >>"$LOG" 2>&1
    exit 1
  }
fi

[ "$WORKER_PREPARE_ONLY" = 1 ] && { log "prepare-only: worktree ready"; exit 0; }

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
"$KUNA_PY" - "$PROMPT_TMPL" "$PROMPT_FILE" "$WORKER_EXTRA_PROMPT" \
  "$WORKER_ID" "$OPP_ID" "$TEST_NAME" "$SELECTOR" "$BINARY" "${ARCH:-none}" \
  "$SLUG" "$BRANCH" "$WT" "$KUNA_PY" "$DATE" "$WORKER_AGENT" "$WORKER_COMMAND" \
  "$WORKER_DECIDER_INSTRUCTIONS" "$WORKER_TRAILER" "$WORKER_GENERATED_WITH" \
  "$RESUME_PROPOSAL_LINE" <<'PY' || {
import pathlib, re, sys

template_path, output_path, extra_path = sys.argv[1:4]
names = (
    "WORKER_ID", "OPPORTUNITY_ID", "TEST_NAME", "SELECTOR", "BINARY", "ARCH",
    "SLUG", "BRANCH", "WORKTREE", "KUNA_PY", "DATE", "WORKER_AGENT",
    "WORKER_COMMAND", "WORKER_DECIDER_INSTRUCTIONS", "WORKER_TRAILER",
    "WORKER_GENERATED_WITH", "RESUME_PROPOSAL",
)
values = dict(zip(names, sys.argv[4:]))
values["SIBLINGS"] = (pathlib.Path(extra_path).read_text(errors="replace")
                      if extra_path and pathlib.Path(extra_path).is_file() else "")
template = pathlib.Path(template_path).read_text(errors="strict")
token = re.compile(r"\{\{([A-Z0-9_]+)\}\}")
missing = sorted(set(token.findall(template)) - set(values))
if missing:
    raise SystemExit("unknown worker prompt placeholder(s): " + ", ".join(missing))
pathlib.Path(output_path).write_text(token.sub(lambda match: values[match.group(1)], template))
PY
  log "prompt render failed"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "prompt render failed" >>"$LOG" 2>&1
  exit 1
}

# --- 5. headless implementation session, in the worktree -------------------
log "launching $BACKEND (model $MODEL${REASONING:+, reasoning $REASONING}, timeout ${WORKER_TIMEOUT}s)"
"$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --phase analyze >>"$LOG" 2>&1

RESULT_JSON="$LOG_DIR/$WORKER_ID.result.json"
if ! : > "$RESULT_JSON"; then
  log "cannot initialize result file $RESULT_JSON"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "result file initialization failed" >>"$LOG" 2>&1
  exit 1
fi
case "$BACKEND" in
  claude)
    CLAUDE_EXTRA=()
    # A caller-supplied session id makes the transcript path deterministic BEFORE the run,
    # which is what lets a harness tail a live builder. Codex has no equivalent.
    [ -n "$WORKER_SESSION_ID" ] && CLAUDE_EXTRA+=(--session-id "$WORKER_SESSION_ID")
    [ -n "$WORKER_BUDGET_USD" ] && CLAUDE_EXTRA+=(--max-budget-usd "$WORKER_BUDGET_USD")
    # A stale ANTHROPIC_API_KEY makes headless Claude fail instead of using session auth.
    ( cd "$WT" && env -u ANTHROPIC_API_KEY timeout "$WORKER_TIMEOUT" claude -p "$(cat "$PROMPT_FILE")" \
        --model "$MODEL" \
        --output-format json \
        --dangerously-skip-permissions \
        "${CLAUDE_EXTRA[@]+"${CLAUDE_EXTRA[@]}"}" \
        < /dev/null > "$RESULT_JSON" 2>>"$LOG" )
    RC=$?
    ;;
  codex)
    EVENTS_JSONL="$LOG_DIR/$WORKER_ID.events.jsonl"
    FINAL_TEXT="$LOG_DIR/$WORKER_ID.final.txt"
    if ! : > "$EVENTS_JSONL" || ! : > "$FINAL_TEXT"; then
      log "cannot initialize Codex transcript files under $LOG_DIR"
      "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "transcript initialization failed" >>"$LOG" 2>&1
      exit 1
    fi
    # Disable Codex's own multi-agent feature: every concurrent worker must hold a pipeline
    # slot. The builder needs full worktree and network access to build, test, push, and merge.
    ( cd "$WT" && timeout -k 60 "$WORKER_TIMEOUT" codex exec \
        --cd "$WT" \
        --sandbox danger-full-access \
        -c approval_policy=never \
        -c "model_reasoning_effort=$REASONING" \
        --model "$MODEL" \
        --disable multi_agent \
        --disable multi_agent_v2 \
        --json \
        -o "$FINAL_TEXT" \
        "$(cat "$PROMPT_FILE")" \
        < /dev/null > "$EVENTS_JSONL" 2>>"$LOG" )
    RC=$?
    # Preserve the same result.json contract used by the captain and recovery notes while
    # retaining Codex's full JSONL stream separately.
    if ! "$KUNA_PY" - "$EVENTS_JSONL" "$FINAL_TEXT" "$RESULT_JSON" "$RC" <<'PY'
import json, pathlib, sys
events_path, final_path, result_path, rc = sys.argv[1:]
thread_id = ""
errors = []
last_message = ""
for line in pathlib.Path(events_path).read_text(errors="replace").splitlines():
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        continue
    if event.get("type") == "thread.started":
        thread_id = event.get("thread_id") or event.get("threadId") or ""
    if event.get("type") == "error":
        errors.append(str(event.get("message") or event.get("error") or event))
    item = event.get("item") or {}
    if event.get("type") == "item.completed" and item.get("type") == "agent_message":
        last_message = str(item.get("text") or "")
final = pathlib.Path(final_path).read_text(errors="replace") if pathlib.Path(final_path).exists() else ""
result = final or last_message or "\n".join(errors)
pathlib.Path(result_path).write_text(json.dumps({
    "backend": "codex", "session_id": thread_id, "result": result,
    "return_code": int(rc), "events": events_path, "final_output": final_path,
    "errors": errors,
}) + "\n")
PY
    then
      log "could not preserve Codex result metadata; raw JSONL remains at $EVENTS_JSONL"
      [ "$RC" -ne 0 ] || RC=1
    fi
    ;;
esac

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
  # An account quota kill is indistinguishable from a crash by exit code alone: Claude exits a
  # bare 1 and the result JSON reads `"subtype": "success"` with `"is_error": true`. The only
  # discriminator is the `result` string, and both of round 2's builders died this way
  # ("You've hit your session limit"). Read it ONLY on the failure path: a finished session can
  # mention a limit in its own prose, and treating that as a quota kill would mislabel good work.
  QUOTA="$("$KUNA_PY" - "$RESULT_JSON" <<'QEOF' 2>/dev/null
import json, re, sys
try:
    d = json.load(open(sys.argv[1]))
except Exception:
    d = {}
# Two DIFFERENT stops, and only one of them says anything in `result`.
#
#   account/session limit  ->  subtype "success", is_error true, and the reason ONLY in
#                              `result` ("You've hit your session limit ...").
#   --max-budget-usd cap   ->  subtype "error_max_budget_usd" and `result` is NULL.
#
# Reading `result` alone therefore classified a budget cap as a generic `claude rc=N`,
# and round 4's captain had to diagnose "the budget-cap salvage case" by opening the
# result JSON itself. Check `subtype` first: where it is informative it is exact, and
# it is the only signal the budget case has.
sub = str(d.get("subtype") or "")
if sub.startswith("error_max_budget"):
    print("builder budget cap ($%s over %s turns)"
          % (d.get("total_cost_usd") or "?", d.get("num_turns") or "?"))
else:
    r = str(d.get("result") or "")
    pat = r"(session|usage|rate)\s+limit|limit\s+(reached|exceeded)"
    print(r.strip() if re.search(pat, r, re.I) else "")
QEOF
)"
  if [ -n "$QUOTA" ]; then
    log "STOPPED BY A CAP, not a build failure: $QUOTA"
    NOTE="capped: $QUOTA ($BACKEND rc=$RC)"
  else
    NOTE="$BACKEND rc=$RC"
  fi

  # Preserve what the session had written. Round 2 lost 618 insertions across 19 files -- a new
  # subcommand, a promoted tests/cli probe, a console verify test -- because a killed builder's
  # work is uncommitted and RESUME_BRANCH only ever sees commits.
  #
  # Guarded, because committing blind is worse than not committing: `git commit` lands on
  # whatever HEAD is, so a session that died mid-rebase or on a detached HEAD would put the
  # commit somewhere the branch cannot reach while the log claimed otherwise. Deliberately not
  # a trap -- an EXIT trap fires on SIGTERM without waiting for the agent subshell, and would
  # stage a tree still being written.
  WT_HEAD="$(git -C "$WT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo '')"
  WT_GITDIR="$(git -C "$WT" rev-parse --git-dir 2>/dev/null || echo '')"
  if [ "$(git -C "$WT" rev-parse --show-toplevel 2>/dev/null)" != "$WT" ]; then
    log "not preserving work: $WT is no longer its own git worktree"
  elif [ "$WT_HEAD" != "$BRANCH" ]; then
    log "not preserving work: $WT is on '$WT_HEAD', not $BRANCH (detached or switched)"
  elif [ -n "$WT_GITDIR" ] && { [ -d "$WT_GITDIR/rebase-merge" ] || [ -d "$WT_GITDIR/rebase-apply" ]; }; then
    log "not preserving work: a rebase is in progress in $WT"
  elif [ -z "$(git -C "$WT" status --porcelain 2>/dev/null)" ]; then
    log "nothing uncommitted to preserve in $WT"
  else
    WT_PHASE="$("$KUNA_PY" -m scripts.pipeline.state list --json 2>/dev/null \
      | "$KUNA_PY" -c 'import json,sys,os
d = json.load(sys.stdin)
w = (d.get("workers") or {}).get(os.environ.get("WORKER_ID","")) or {}
print(w.get("phase") or "unknown")' 2>/dev/null || echo unknown)"
    git -C "$WT" add -A >>"$LOG" 2>&1
    if git -C "$WT" commit -q \
         -m "[AUTOMATED] WIP UNFINISHED, DO NOT MERGE: $SLUG stopped in phase $WT_PHASE" \
         -m "Crash-preservation snapshot written by tools/pipeline/worker.sh, not by the builder. The session ended before finishing: $NOTE" \
         -m "NOTHING HERE IS VERIFIED. No build, no make test / test-stages / rust-test / check-spec, no counters --fix, no mergecheck, no acceptance probe. Raw material for a resuming builder, never a reviewable change." \
         >>"$LOG" 2>&1; then
      log "preserved uncommitted work as WIP commit $(git -C "$WT" rev-parse --short HEAD) on $BRANCH"
    else
      log "WARNING: could not commit the uncommitted work in $WT; it is left in place"
    fi
  fi

  log "$BACKEND session exited rc=$RC (timeout=124); leaving worktree for inspection"
  "$KUNA_PY" -m scripts.pipeline.state update --worker "$WORKER_ID" --status failed --note "$NOTE" >>"$LOG" 2>&1
  exit $RC
fi

log "$BACKEND session complete (rc=0). worktree $WT and session $SID preserved for PR review."
# the worker itself records `state done --pr <url>` when it opens the PR
exit 0
