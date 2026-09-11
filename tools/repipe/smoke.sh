#!/usr/bin/env bash
# Prove the RE-friction loop works, without spending a single agent token.
#
#   tools/repipe/smoke.sh            # levels 0 and 1 (~4 min, zero tokens, zero network)
#   tools/repipe/smoke.sh --level 0  # unit + conformance only (~40 s)
#
# Levels 2-4 (one live tester, one live builder, a real round) are in docs/re-pipeline.md
# and cost real money; they are deliberately NOT run from here.
#
# FOUR FALSE-GREEN CANARIES, in the spirit of `make test-ghidra`'s two. Each of these would
# otherwise let a silently broken run report success:
#   - the two-arm gate must find at least one REPRODUCING probe (a broken runner would
#     "pass" everything by failing everything);
#   - the sandbox assertions must actually have run (skipping them because bwrap is missing
#     is not a pass);
#   - the redactor must NOT have dropped every extras file (over-redaction is as bad as
#     under-redaction, and it would hide the fact that the allowlist is broken);
#   - at least one need must have been produced by clustering (zero needs is not "clean").
set -uo pipefail

REPO="${KUNA_REPO:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
PY="${KUNA_PY:-python3}"
LEVEL="${1:-}"
[ "$LEVEL" = "--level" ] && LEVEL="${2:-1}" || LEVEL=1

export PYTHONPATH="$REPO${PYTHONPATH:+:$PYTHONPATH}"
SMOKE_STATE="$(mktemp -d "${TMPDIR:-/tmp}/repipe-smoke-XXXXXX")"
export KUNA_PIPELINE_STATE_DIR="$SMOKE_STATE"
trap 'rm -rf "$SMOKE_STATE"' EXIT

DATASET="${REPIPE_DATASET:-$HOME/github/kuna-re-dataset}"
FIX="$REPO/tools/repipe/fixtures"
PASS=0; FAIL=0
# canaries
SAW_REPRODUCING=0; SAW_SANDBOX=0; SAW_EXTRAS_KEPT=0; SAW_NEED=0

ok()   { PASS=$((PASS+1)); printf '  \033[32mok\033[0m   %s\n' "$1"; }
bad()  { FAIL=$((FAIL+1)); printf '  \033[31mFAIL\033[0m %s\n' "$1"; [ -n "${2:-}" ] && printf '       %s\n' "$2"; }
head_() { printf '\n\033[1m%s\033[0m\n' "$1"; }

# ---------------------------------------------------------------- level 0 ---
head_ "L0  modules import and self-describe"
for m in config probe verify workspace redact sample grade needs cluster select counters mergecheck captain webui render_tester_prompt; do
  if "$PY" -c "import scripts.repipe.$m" 2>/dev/null; then ok "import $m"; else bad "import $m"; fi
done

head_ "L0  aggregate performance regressions require stable evidence"
"$PY" - <<'PY' >/dev/null 2>&1 \
  && ok "near-bound samples and contradictory confirmation cannot requeue a need" \
  || bad "aggregate timing noise can still trigger a regression"
from scripts.repipe import probe, verify

doc = {
    "schema": "re-probe/1", "kind": "timing", "cmd": ["kuna"], "timeout_s": 5,
    "repeat": 3, "expect": {"exit_code": {"eq": 0},
                            "wall_ms": {"stat": "median", "lt": 10}},
}
def observation(samples):
    return {"runs": [{"exit_code": 0, "stdout": "", "stderr": "", "wall_ms": value,
                       "timed_out": False, "error": None} for value in samples],
            "baselines": {}}

crossing = probe.evaluate(probe.normalize(doc), observation([9, 11, 12]))
assert not crossing["passed"] and crossing["flaky"], crossing
assert crossing["flaky_clauses"] == ["wall_ms"], crossing
crossing_pass = probe.evaluate(probe.normalize(doc), observation([9, 9, 11]))
assert not crossing_pass["passed"] and crossing_pass["flaky"], crossing_pass
max_doc = dict(doc, expect={"exit_code": {"eq": 0},
                            "wall_ms": {"stat": "max", "lt": 10}})
extreme = probe.evaluate(probe.normalize(max_doc), observation([9, 11, 12]))
assert not extreme["passed"] and not extreme["flaky"], extreme

def verdict(passed):
    return {"passed": passed, "flaky": False, "unrunnable": False, "error": None,
            "probe_id": "a-smoke", "clauses": [
                {"clause": "exit_code", "ok": True},
                {"clause": "wall_ms", "ok": passed,
                 "expected": {"stat": "median", "lt": 10}},
            ]}

record = {"need_id": "perf-smoke", "front_matter": {"status": "closed", "challenges": []}}
verify.need_records = lambda _ids=None: [record]
verify.acceptance_of = lambda _rec: (doc, "{}", "record")
verify.head_sha = lambda: "abc123"

answers = iter([verdict(False), verdict(True)])
verify.run_probe = lambda *_args, **_kwargs: next(answers)
row = verify.acceptance_suite()["needs"][0]
assert row["transition"] == "indeterminate" and row["flaky"], row
assert row["acceptance"]["batch_passed"] == [False, True], row

answers = iter([verdict(False), dict(verdict(False), clauses=[
    {"clause": "exit_code", "ok": False},
    {"clause": "wall_ms", "ok": True,
     "expected": {"stat": "median", "lt": 10}},
])])
verify.run_probe = lambda *_args, **_kwargs: next(answers)
row = verify.acceptance_suite()["needs"][0]
assert row["transition"] == "indeterminate" and row["flaky"], row

answers = iter([verdict(False), verdict(False)])
verify.run_probe = lambda *_args, **_kwargs: next(answers)
row = verify.acceptance_suite()["needs"][0]
assert row["transition"] == "regressed" and not row["flaky"], row
PY

head_ "L0  agent split matches the documented table"
SPLIT="$("$PY" -c 'from scripts.repipe import config; import json; print(json.dumps({n: config.agent_split(n) for n in (2,4,5,7,9)}))')"
echo "$SPLIT" | grep -q '"7": {"captain": 1, "testers": 3, "builders": 3' \
  && ok "max-agents 7 -> 1 captain + 3 testers + 3 builders" \
  || bad "7 does not split 1/3/3" "$SPLIT"
ROLES="$("$PY" -c 'from scripts.repipe import config as c; print(":".join((c.TESTER_MODEL,c.TESTER_REASONING,c.BUILDER_BACKEND,c.BUILDER_MODEL,c.BUILDER_REASONING)))')"
[ "$ROLES" = "gpt-5.6-sol:low:codex:gpt-5.6-sol:high" ] \
  && ok "role defaults are Sol-low reversers and Sol-high builders" \
  || bad "role defaults weakened" "$ROLES"
CAPTAIN_TIMEOUT="$(env -u REPIPE_CAPTAIN_TIMEOUT "$PY" -c \
  'from scripts.repipe import config; print(config.CAPTAIN_TIMEOUT)')"
[ "$CAPTAIN_TIMEOUT" = 3600 ] \
  && grep -q 'REPIPE_CAPTAIN_TIMEOUT:-3600' "$REPO/tools/repipe/captain.sh" \
  && grep -q 'REPIPE_CAPTAIN_TIMEOUT:-3600' "$REPO/tools/repipe/run.sh" \
  && ok "captain timeout defaults agree at 3600s" \
  || bad "captain timeout defaults disagree" "$CAPTAIN_TIMEOUT"
POLICY="$(REPIPE_TESTER_REASONING=medium REPIPE_BUILDER_MODEL=other \
  REPIPE_BUILDER_REASONING=medium REPIPE_CAPTAIN_BACKEND=other \
  "$PY" -c 'from scripts.repipe import config as c; print("\n".join(c.role_policy_problems()))')"
for expected in "reversers require" "implementation requires" \
                "implementation reasoning must" "CAPTAIN_BACKEND must"; do
  echo "$POLICY" | grep -q "$expected" \
    && ok "preflight role policy rejects: $expected" \
    || bad "preflight role policy missed: $expected" "$POLICY"
done
TESTER_REFUSAL="$(REPIPE_TESTER_MODEL=other bash "$REPO/tools/repipe/tester.sh" 2>&1)"
if [ "$?" -eq 2 ] && echo "$TESTER_REFUSAL" | grep -q 'reversers require'; then
  ok "standalone reverser refuses a weak model before launch"
else
  bad "standalone reverser accepted a weak model" "$TESTER_REFUSAL"
fi
grep -q -- '--disable multi_agent' "$REPO/tools/repipe/tester.sh" \
  && grep -q -- '--disable multi_agent_v2' "$REPO/tools/repipe/tester.sh" \
  && ok "reversers cannot bypass pipeline slots with Codex sub-agents" \
  || bad "reverser launch does not disable Codex multi-agent features"
KUNA_PIPELINE_STATE_DIR="$SMOKE_STATE/spawn-policy" "$PY" - <<'PY' >/dev/null 2>&1 \
  && ok "captain exports the validated Sol-low policy to reversers" \
  || bad "captain does not pin the reverser subprocess environment"
from scripts.repipe import captain

seen = {}
captain.subprocess.Popen = lambda *args, **kwargs: seen.update(kwargs) or object()
captain.spawn_tester(1, "0123456789abcdef")
assert seen["env"]["REPIPE_TESTER_MODEL"] == "gpt-5.6-sol"
assert seen["env"]["REPIPE_TESTER_REASONING"] == "low"
PY

head_ "L0  state machine refuses an illegal transition"
"$PY" -m scripts.repipe.captain --transition test T_PLAN --note smoke >/dev/null 2>&1
if "$PY" -m scripts.repipe.captain --transition test T_READY >/dev/null 2>&1; then
  bad "T_PLAN -> T_READY was ALLOWED (a builder could skip the gates)"
else
  ok "T_PLAN -> T_READY refused (exit 2)"
fi
if "$PY" -m scripts.repipe.captain --transition build B_MERGE >/dev/null 2>&1; then
  bad "B_IDLE -> B_MERGE was ALLOWED (merging without building)"
else
  ok "B_IDLE -> B_MERGE refused"
fi

head_ "L0  counters re-derive from the live tree"
CNT="$("$PY" -m scripts.repipe.counters --rederive --json 2>/dev/null)"
# Assert the MECHANISM, not a snapshot. Pinning "settables = 127" here made every
# option that lands break the smoke, which is a false alarm that teaches people to
# ignore it -- and it never once caught a real drift, because counters --check is
# what does that. So: the derivation must produce a plausible number, and every
# hard-coded site in the tree must agree with it.
SETTABLES="$(echo "$CNT" | "$PY" -c 'import json,sys; d=json.load(sys.stdin); print(d.get("derived", d).get("settables", 0))' 2>/dev/null || echo 0)"
CORPUS="$(echo "$CNT" | "$PY" -c 'import json,sys; d=json.load(sys.stdin); print(d.get("derived", d).get("corpus_files", 0))' 2>/dev/null || echo 0)"
[ "${SETTABLES:-0}" -ge 100 ] && ok "settables derived ($SETTABLES)" \
  || bad "settable derivation implausible" "$(echo "$CNT" | head -3)"
[ "${CORPUS:-0}" -ge 200 ] && ok "stage corpus derived ($CORPUS)" \
  || bad "corpus derivation implausible"
if "$PY" -m scripts.repipe.counters --check >/dev/null 2>&1; then
  ok "every hard-coded counter site agrees with the derived truth"
else
  bad "counter drift" "$("$PY" -m scripts.repipe.counters --check 2>&1 | grep DRIFT | head -4)"
fi
echo "$CNT" | grep -q '"next_element_id"' && ok "next free ElementId derived" || bad "no ElementId derivation"

head_ "L0  mergecheck catches all three silent-merge shapes"
MC="$("$PY" -m scripts.repipe.mergecheck --self-test 2>&1)"
for shape in shape-B-averted shape-C-averted shape-C2-averted; do
  echo "$MC" | grep -q "$shape" && ok "$shape" || bad "$shape not caught"
done

head_ "L0  worker worktree collisions fail closed"
LAUNCH_REPO="$SMOKE_STATE/launcher-repo"
git init -q -b main "$LAUNCH_REPO"
git -C "$LAUNCH_REPO" config user.email smoke@example.invalid
git -C "$LAUNCH_REPO" config user.name smoke
printf 'base\n' > "$LAUNCH_REPO/tracked"
printf 'binaries:\n\t@:\n' > "$LAUNCH_REPO/Makefile"
git -C "$LAUNCH_REPO" add tracked
git -C "$LAUNCH_REPO" add Makefile
git -C "$LAUNCH_REPO" commit -qm base
run_prepare() {
  KUNA_REPO="$LAUNCH_REPO" KUNA_PY="$PY" PIPELINE_STATE_DIRNAME=.state \
    WORKER_PREPARE_ONLY=1 WORKER_ID="$1" OPP_ID=smoke TEST_NAME=smoke \
    SELECTOR=- BINARY=- SLUG=collision ARCH= WORKER_BRANCH="$2" \
    IMPL_PROPOSAL="${3:-0}" RESUME_BRANCH="${4:-}" \
    bash "$REPO/tools/pipeline/worker.sh" >/dev/null 2>&1
}
if WORKER_BACKEND=codex WORKER_MODEL=gpt-5.6-sol WORKER_REASONING=low \
   run_prepare weak-model feat/re-weak-model; then
  bad "Codex builder accepted reasoning below high"
elif [ ! -e "$LAUNCH_REPO/.state/worktrees/weak-model" ] \
     && ! git -C "$LAUNCH_REPO" show-ref --verify --quiet refs/heads/feat/re-weak-model; then
  ok "Codex builder refuses reasoning below high before touching Git state"
else
  bad "weak-model refusal touched Git state"
fi
if WORKER_BACKEND=codex WORKER_MODEL=other WORKER_REASONING=high \
   run_prepare wrong-sol feat/re-wrong-sol; then
  bad "Codex builder accepted a non-Sol model"
elif [ ! -e "$LAUNCH_REPO/.state/worktrees/wrong-sol" ] \
     && ! git -C "$LAUNCH_REPO" show-ref --verify --quiet refs/heads/feat/re-wrong-sol; then
  ok "Codex builder refuses a non-Sol model before touching Git state"
else
  bad "non-Sol refusal touched Git state"
fi
if run_prepare fresh feat/re-collision-r7 \
   && [ "$(git -C "$LAUNCH_REPO/.state/worktrees/fresh" branch --show-current)" = feat/re-collision-r7 ]; then
  ok "explicit WORKER_BRANCH creates an attached fresh worktree"
else
  bad "explicit WORKER_BRANCH did not create the requested branch"
fi
run_prepare fresh feat/re-collision-r7 \
  || bad "exact path+branch reuse was refused"
FRESH_BASE="$(cat "$LAUNCH_REPO/.state/logs/fresh.base" 2>/dev/null)"
printf 'preserved work\n' > "$LAUNCH_REPO/.state/worktrees/fresh/wip"
git -C "$LAUNCH_REPO/.state/worktrees/fresh" add wip
git -C "$LAUNCH_REPO/.state/worktrees/fresh" commit -qm \
  '[AUTOMATED] WIP UNFINISHED, DO NOT MERGE: smoke'
FRESH_WIP="$(git -C "$LAUNCH_REPO/.state/worktrees/fresh" rev-parse HEAD)"
if run_prepare fresh feat/re-collision-r7 \
   && [ "$(cat "$LAUNCH_REPO/.state/logs/fresh.base")" = "$FRESH_BASE" ] \
   && [ "$FRESH_BASE" != "$FRESH_WIP" ]; then
  ok "an exact WIP worktree retry preserves its original base marker"
else
  bad "exact WIP worktree retry replaced its base with the WIP commit"
fi
printf 'not-a-commit\n' > "$LAUNCH_REPO/.state/logs/fresh.base"
if run_prepare fresh feat/re-collision-r7 \
   && [ "$(cat "$LAUNCH_REPO/.state/logs/fresh.base")" = "$FRESH_BASE" ]; then
  ok "a stale base marker is repaired from the branch point, not WIP HEAD"
else
  bad "stale base marker was repaired to the WIP commit"
fi
rm -f "$LAUNCH_REPO/.state/logs/fresh.base"
if run_prepare fresh feat/re-collision-r7 \
   && [ "$(cat "$LAUNCH_REPO/.state/logs/fresh.base")" = "$FRESH_BASE" ]; then
  ok "a missing base marker is restored from the branch point, not WIP HEAD"
else
  bad "missing base marker was initialized to the WIP commit"
fi

git -C "$LAUNCH_REPO" branch feat/re-stale main
STALE_BEFORE="$(git -C "$LAUNCH_REPO" rev-parse feat/re-stale)"
if run_prepare stale feat/re-stale; then
  bad "a stale branch without its worktree was reused"
elif [ ! -e "$LAUNCH_REPO/.state/worktrees/stale" ] \
     && [ "$(git -C "$LAUNCH_REPO" rev-parse feat/re-stale)" = "$STALE_BEFORE" ] \
     && [ "$(git -C "$LAUNCH_REPO" branch --show-current)" = main ]; then
  ok "a stale branch is refused without moving its ref or root checkout"
else
  bad "stale-branch refusal mutated a path, ref, or root checkout"
fi

git -C "$LAUNCH_REPO" worktree add -q -b feat/re-wrong "$LAUNCH_REPO/.state/worktrees/wrong" main
printf 'keep\n' > "$LAUNCH_REPO/.state/worktrees/wrong/untracked"
if run_prepare wrong feat/re-wanted; then
  bad "an existing wrong-branch path was replaced"
elif [ -f "$LAUNCH_REPO/.state/worktrees/wrong/untracked" ] \
     && [ "$(git -C "$LAUNCH_REPO/.state/worktrees/wrong" branch --show-current)" = feat/re-wrong ] \
     && ! git -C "$LAUNCH_REPO" show-ref --verify --quiet refs/heads/feat/re-wanted; then
  ok "a wrong-branch dirty path and its untracked file are preserved"
else
  bad "wrong-path refusal damaged the existing worktree"
fi

FOREIGN="$LAUNCH_REPO/.state/worktrees/foreign"
git init -q -b feat/re-foreign "$FOREIGN"
git -C "$FOREIGN" config user.email smoke@example.invalid
git -C "$FOREIGN" config user.name smoke
printf 'foreign\n' > "$FOREIGN/keep"
git -C "$FOREIGN" add keep
git -C "$FOREIGN" commit -qm foreign
if run_prepare foreign feat/re-foreign; then
  bad "an independent repository with the requested branch was reused"
elif [ "$(git -C "$FOREIGN" rev-parse --show-toplevel)" = "$FOREIGN" ] \
     && [ "$(git -C "$FOREIGN" branch --show-current)" = feat/re-foreign ] \
     && [ "$(cat "$FOREIGN/keep")" = foreign ]; then
  ok "same-named branch in an independent repository is preserved and refused"
else
  bad "foreign repository refusal damaged the independent repository"
fi

git -C "$LAUNCH_REPO" branch feat/re-resume main
if run_prepare missing-resume feat/re-must-not-exist 1; then
  bad "proposal mode accepted an empty RESUME_BRANCH"
elif [ ! -e "$LAUNCH_REPO/.state/worktrees/missing-resume" ] \
     && ! git -C "$LAUNCH_REPO" show-ref --verify --quiet refs/heads/feat/re-must-not-exist; then
  ok "proposal mode without RESUME_BRANCH fails before creating a ref or worktree"
else
  bad "empty proposal resume created or attached Git state"
fi
if run_prepare resume ignored 1 feat/re-resume \
   && [ "$(git -C "$LAUNCH_REPO/.state/worktrees/resume" branch --show-current)" = feat/re-resume ]; then
  ok "IMPL_PROPOSAL and RESUME_BRANCH still attach the approved branch"
else
  bad "explicit proposal resume no longer works"
fi

# Exercise the real Codex branch without spending tokens. The shim emits the two artifacts
# Codex promises: JSONL on stdout and the final response at -o/--output-last-message.
FAKE_BIN="$SMOKE_STATE/fake-bin"
mkdir -p "$FAKE_BIN"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -u' \
  'out=' \
  'while [ "$#" -gt 0 ]; do' \
  '  case "$1" in -o|--output-last-message) shift; out="$1";; esac' \
  '  shift' \
  'done' \
  'printf "mock final response\\n" > "$out"' \
  'printf "%s\\n" '\''{"type":"thread.started","thread_id":"thread-smoke"}'\''' \
  'printf "%s\\n" '\''{"type":"item.completed","item":{"type":"agent_message","text":"event fallback"}}'\''' \
  > "$FAKE_BIN/codex"
chmod +x "$FAKE_BIN/codex"
if PATH="$FAKE_BIN:$PATH" PYTHONPATH="$REPO" KUNA_REPO="$LAUNCH_REPO" KUNA_PY="$PY" \
   PIPELINE_STATE_DIRNAME=.state WORKER_ID=codex-launch OPP_ID='odd&|path\name' TEST_NAME=smoke \
   SELECTOR=- BINARY=- SLUG=codex-launch ARCH= \
   WORKER_BRANCH=feat/re-codex-launch WORKER_BACKEND=codex WORKER_MODEL=gpt-5.6-sol \
   WORKER_REASONING=high WORKER_PROMPT="$REPO/tools/repipe/builder_prompt.md" \
   bash "$REPO/tools/pipeline/worker.sh" >/dev/null 2>&1; then
  CODEX_RESULT="$LAUNCH_REPO/.state/logs/codex-launch.result.json"
  CODEX_EVENTS="$LAUNCH_REPO/.state/logs/codex-launch.events.jsonl"
  CODEX_PROMPT="$LAUNCH_REPO/.state/logs/codex-launch.prompt.md"
  if "$PY" - "$CODEX_RESULT" "$CODEX_EVENTS" "$CODEX_PROMPT" <<'PY' >/dev/null 2>&1
import json, pathlib, sys
result = json.load(open(sys.argv[1]))
assert result["backend"] == "codex"
assert result["session_id"] == "thread-smoke"
assert result["result"] == "mock final response\n"
assert result["return_code"] == 0
assert pathlib.Path(sys.argv[2]).read_text().count("\n") == 2
prompt = pathlib.Path(sys.argv[3]).read_text()
assert "odd&|path\\name" in prompt
assert "Codex implementation worker using gpt-5.6-sol with high reasoning" in prompt
assert "/.state/logs/codex-launch.base" in prompt
assert "{{WORKER_" not in prompt
PY
  then
    ok "Codex worker preserves JSONL/result metadata and renders provider-safe prompts"
  else
    bad "Codex worker artifacts or rendered prompt are incorrect"
  fi
else
  bad "mocked Codex worker launch failed" "$(tail -8 "$LAUNCH_REPO/.state/logs/codex-launch.log" 2>/dev/null)"
fi
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "%s\\n" '\''{"session_id":"claude-smoke","result":"mock final response"}'\''' \
  > "$FAKE_BIN/claude"
chmod +x "$FAKE_BIN/claude"
if PATH="$FAKE_BIN:$PATH" PYTHONPATH="$REPO" KUNA_REPO="$LAUNCH_REPO" KUNA_PY="$PY" \
   PIPELINE_STATE_DIRNAME=.state WORKER_ID=claude-launch OPP_ID=smoke TEST_NAME=smoke \
   SELECTOR=- BINARY=- SLUG=claude-launch ARCH= WORKER_BRANCH=feat/re-claude-launch \
   WORKER_BACKEND=claude WORKER_MODEL=opus \
   WORKER_PROMPT="$REPO/tools/pipeline/worker_prompt.md" \
   bash "$REPO/tools/pipeline/worker.sh" >/dev/null 2>&1; then
  CLAUDE_RESULT="$LAUNCH_REPO/.state/logs/claude-launch.result.json"
  CLAUDE_PROMPT="$LAUNCH_REPO/.state/logs/claude-launch.prompt.md"
  if "$PY" - "$CLAUDE_RESULT" "$CLAUDE_PROMPT" <<'PY' >/dev/null 2>&1
import json, pathlib, sys
assert json.load(open(sys.argv[1]))["session_id"] == "claude-smoke"
prompt = pathlib.Path(sys.argv[2]).read_text()
assert "Claude Code worker using opus" in prompt
assert "spawn a **decider subagent**" in prompt
assert "Co-Authored-By: Claude Opus 5" in prompt
assert "{{WORKER_" not in prompt
PY
  then
    ok "shared worker retains its Claude launch and provider-specific prompt"
  else
    bad "Claude worker result or rendered prompt is incorrect"
  fi
else
  bad "mocked Claude worker launch failed" "$(tail -8 "$LAUNCH_REPO/.state/logs/claude-launch.log" 2>/dev/null)"
fi

CAP_BRANCH="$($PY -c 'from scripts.repipe.captain import _builder_branch; print(_builder_branch(12, "need-name"))')"
[ "$CAP_BRANCH" = feat/re-need-name-r12 ] \
  && ok "RE captain assigns a stable round-specific fresh branch" \
  || bad "RE captain branch is not stable and round-specific" "$CAP_BRANCH"

KUNA_PIPELINE_STATE_DIR="$SMOKE_STATE/resume-test" "$PY" - <<'PY' >/dev/null 2>&1 \
  && ok "operator resume is guarded and clears HALT_REASON only on success" \
  || bad "operator HALTED recovery guard failed"
from scripts.repipe import captain, config

doc = captain.load_round(9)
doc["supervisor"] = "HALTED"
captain.save_round(doc)
reason = config.state_dir() / "HALT_REASON"
reason.write_text("smoke halt\n")
captain.current_round = lambda: 9
captain.pstate.reap = lambda stale_seconds=0: []
captain.preflight = lambda: []
captain.live_agents = lambda pool: []
out = captain.resume_halted()
assert out["ok"], out
assert captain.load_round(9)["supervisor"] == "RUNNING"
assert captain.load_round(9)["notes"][-1].startswith("operator resume after reap"), captain.load_round(9)
assert not reason.exists()
rec = list((config.rounds_dir() / "9" / "transitions.jsonl").read_text().splitlines())[-1]
assert '"from": "HALTED"' in rec and '"operator_resume": true' in rec, rec

doc = captain.load_round(9)
doc["supervisor"] = "HALTED"
captain.save_round(doc)
reason.write_text("keep me\n")
captain.live_agents = lambda pool: ["busy"] if pool == "builder" else []
out = captain.resume_halted()
assert not out["ok"], out
assert captain.load_round(9)["supervisor"] == "HALTED"
assert reason.read_text() == "keep me\n"

captain.live_agents = lambda pool: []
(config.state_dir() / "STOP").write_text("")
out = captain.resume_halted()
assert not out["ok"] and "STOP" in out["problems"][0], out
assert captain.load_round(9)["supervisor"] == "HALTED"
assert reason.read_text() == "keep me\n"
(config.state_dir() / "STOP").unlink()

def controls_during_preflight():
    (config.state_dir() / "ABORT").write_text("")
    return []
captain.preflight = controls_during_preflight
out = captain.resume_halted()
assert not out["ok"] and "ABORT" in out["problems"][0], out
assert captain.load_round(9)["supervisor"] == "HALTED"
assert reason.read_text() == "keep me\n"
(config.state_dir() / "ABORT").unlink()

captain.preflight = lambda: []
real_save = captain.save_round
def later_halt(doc):
    real_save(doc)
    reason.write_text("later halt\n")
captain.save_round = later_halt
out = captain.resume_halted()
assert out["ok"], out
assert captain.load_round(9)["supervisor"] == "RUNNING"
assert reason.read_text() == "later halt\n"
PY

KUNA_PIPELINE_STATE_DIR="$SMOKE_STATE/restart-test" "$PY" - <<'PY' >/dev/null 2>&1 \
  && ok "STOPPED restart is guarded and records the operator edge" \
  || bad "operator STOPPED restart guard failed"
import json
from scripts.repipe import captain, config

doc = captain.load_round(10)
doc["supervisor"] = "STOPPED"
captain.save_round(doc)
captain.current_round = lambda: 10
captain.pstate.reap = lambda stale_seconds=0: []
captain.preflight = lambda: []
captain.live_agents = lambda pool: []
out = captain.restart_stopped()
assert out["ok"], out
restarted = captain.load_round(10)
assert restarted["supervisor"] == "RUNNING", restarted
assert restarted["notes"][-1].startswith("operator restart after reap"), restarted
rec = json.loads((config.rounds_dir() / "10" / "transitions.jsonl").read_text().splitlines()[-1])
assert rec["from"] == "STOPPED" and rec["to"] == "RUNNING", rec
assert rec["operator_resume"] is True, rec

restarted["supervisor"] = "STOPPED"
captain.save_round(restarted)
captain.live_agents = lambda pool: ["busy"] if pool == "tester" else []
out = captain.restart_stopped()
assert not out["ok"] and out["problems"] == ["live agent slots remain"], out
assert captain.load_round(10)["supervisor"] == "STOPPED"

captain.live_agents = lambda pool: []
(config.state_dir() / "STOP").write_text("")
out = captain.restart_stopped()
assert not out["ok"] and "STOP" in out["problems"][0], out
assert captain.load_round(10)["supervisor"] == "STOPPED"
(config.state_dir() / "STOP").unlink()

def controls_during_preflight():
    (config.state_dir() / "ABORT").write_text("")
    return []
captain.preflight = controls_during_preflight
out = captain.restart_stopped()
assert not out["ok"] and "ABORT" in out["problems"][0], out
assert captain.load_round(10)["supervisor"] == "STOPPED"
PY

RUN_REPO="$SMOKE_STATE/run-repo"
RUN_PY="$SMOKE_STATE/fake-run-python"
mkdir -p "$RUN_REPO/tools/repipe" "$RUN_REPO/.state"
printf '%s\n' '#!/usr/bin/env bash' 'exit 0' > "$RUN_REPO/tools/repipe/captain.sh"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'case "$*" in' \
  '  *"--preflight"*) exit 0;;' \
  '  *"--restart-stopped"*)' \
  '    [ ! -e "$KUNA_PIPELINE_STATE_DIR/STOP" ] || exit 1' \
  '    [ ! -e "$KUNA_PIPELINE_STATE_DIR/ABORT" ] || exit 1' \
  '    touch "$KUNA_PIPELINE_STATE_DIR/restarted";;' \
  '  *"--status"*) printf '\''{"states":{"supervisor":"STOPPED"},"round":1}\n'\'';;' \
  '  *'\''["states"]["supervisor"]'\''*) printf '\''STOPPED\n'\'';;' \
  '  *'\''["round"]'\''*) printf '\''1\n'\'';;' \
  '  *"-c"*) printf '\''0\n'\'';;' \
  'esac' > "$RUN_PY"
chmod +x "$RUN_REPO/tools/repipe/captain.sh" "$RUN_PY"
touch "$RUN_REPO/.state/STOP"
if KUNA_REPO="$RUN_REPO" KUNA_PY="$RUN_PY" REPIPE_STATE_DIRNAME=.state \
   bash "$REPO/tools/repipe/run.sh" >/dev/null 2>&1 \
   && [ -e "$RUN_REPO/.state/restarted" ] && [ ! -e "$RUN_REPO/.state/STOP" ]; then
  ok "run.sh consumes a completed STOP and invokes guarded restart"
else
  bad "run.sh did not restart a STOPPED supervisor"
fi
rm -f "$RUN_REPO/.state/restarted"
touch "$RUN_REPO/.state/STOP" "$RUN_REPO/.state/ABORT"
if KUNA_REPO="$RUN_REPO" KUNA_PY="$RUN_PY" REPIPE_STATE_DIRNAME=.state \
   bash "$REPO/tools/repipe/run.sh" >/dev/null 2>&1; then
  bad "run.sh accepted an uncleared ABORT"
elif [ -e "$RUN_REPO/.state/STOP" ] && [ -e "$RUN_REPO/.state/ABORT" ] \
     && [ ! -e "$RUN_REPO/.state/restarted" ]; then
  ok "run.sh refuses ABORT without consuming control files"
else
  bad "run.sh mutated control files while refusing ABORT"
fi

[ "$LEVEL" = "0" ] && { printf '\nL0 only: %d passed, %d failed\n' "$PASS" "$FAIL"; exit $((FAIL>0)); }

# ---------------------------------------------------------------- level 1 ---
head_ "L1  arena is contamination-free"
FLAGCH=6609e458cddae72ae250bf40      # ground_truth.flag = N@n0m4ch1n3sS0n!
TRAPCH=5ab77f5533c5d40ad448c1ea      # extras/.../hints.txt = 19 valid serials, ships_source_code FALSE
for ch in "$FLAGCH" "$TRAPCH"; do
  OUT="$("$PY" -m scripts.repipe.workspace build "$ch" --round 0 2>&1)" || { bad "arena build $ch" "$OUT"; continue; }
  A="$SMOKE_STATE/arena/0/$ch"
  [ -d "$A" ] && ok "arena built $ch" || { bad "no arena at $A"; continue; }
  [ -z "$(find "$A" -name meta.json -o -name verifier.py)" ] && ok "  no meta.json / verifier.py" || bad "  metadata leaked into the arena"
  NONEXEC="$(find "$A/target" -type f ! -perm -u+x 2>/dev/null | wc -l)"
  [ "$NONEXEC" = "0" ] && ok "  every target binary is executable" || bad "  $NONEXEC target file(s) lack +x"
done

# The two challenges above ship no keepable extras (one has none, the other has only
# spoilers), so the over-redaction canary needs a challenge that really does carry an author
# README -- otherwise "kept 0" is correct and the canary would fire on a healthy redactor.
READMECH="$("$PY" - <<'PY'
import json, os
from scripts.repipe import config
man = json.load(open(config.manifest_path()))
for rec in man:
    extras = (rec.get("files") or {}).get("extras") or []
    if any(os.path.basename(e).lower().startswith("readme") for e in extras):
        print(rec["hexid"]); break
PY
)"
if [ -n "$READMECH" ]; then
  ROUT="$("$PY" -m scripts.repipe.workspace build "$READMECH" --round 0 2>&1)"
  KEPT="$(echo "$ROUT" | sed -n 's/.*extras kept: \([0-9]*\).*/\1/p')"
  [ "${KEPT:-0}" != "0" ] && { ok "author README kept for $READMECH (allowlist works)"; SAW_EXTRAS_KEPT=1; }                           || bad "over-redaction: README dropped for $READMECH" "$ROUT"
else
  bad "no challenge with a README in extras -- cannot exercise the allowlist"
fi
grep -rq 'N@n0m4ch1n3sS0n' "$SMOKE_STATE/arena/0/$FLAGCH" 2>/dev/null \
  && bad "THE FLAG IS IN THE ARENA" || ok "flag absent from the arena"
if echo "$OUT" | grep -q 'serial-shape .*hints.txt'; then
  ok "hints.txt dropped by the serial-shape rule (not the flag rule)"
else
  "$PY" -m scripts.repipe.workspace build "$TRAPCH" --round 0 2>&1 | grep -q 'serial-shape' \
    && ok "serial-shape rule fired on the trap challenge" \
    || bad "hints.txt was NOT dropped by serial-shape"
fi

head_ "L1  sandbox actually hides the dataset"
if command -v bwrap >/dev/null 2>&1; then
  N="$(bwrap --dev-bind / / --tmpfs "$DATASET" -- ls "$DATASET" 2>/dev/null | wc -l)"
  [ "$N" = "0" ] && { ok "dataset is empty inside the namespace"; SAW_SANDBOX=1; } || bad "dataset still visible ($N entries)"
  bwrap --dev-bind / / --tmpfs "$DATASET" -- "$REPO/decompiler/target/release/kuna" --version >/dev/null 2>&1 \
    && ok "kuna still runs inside the namespace" || bad "kuna broken inside the namespace"
else
  bad "bwrap missing -- containment is prompt-only and this canary must not be skipped silently"
fi

head_ "L1  the two-arm gate returns each of its three verdicts"
CH="$DATASET/challenges/64f1f7afd931496abf909525"   # a real work dir for the probes below
# This used to gate against a live kuna defect, and the canary caught the day that defect was
# FIXED: the probe stopped reproducing and the smoke failed for the best possible reason. A
# self-test must not decay every time the product improves, so the arms below are constructed
# to hold forever, and the "against truth" job moved to tests/cli/ -- five probes that each
# assert a gap really is closed, checked above and in CI.
GATE_OBS="$SMOKE_STATE/gate-obs.json"
mk_obs() {  # $1 probe-expect, $2 acceptance-expect
  "$PY" - "$1" "$2" > "$GATE_OBS" <<'PY'
import json, sys
arm = lambda e: {"schema": "re-probe/1", "kind": "cli", "cmd": ["{{KUNA}}", "--version"],
                 "cwd": "{{TMP}}", "timeout_s": 30, "repeat": 1, "expect": json.loads(e)}
json.dump({"kind": "bad-ux", "title": "gate self-test", "what_i_wanted": "x",
           "what_kuna_did": "y", "severity": "minor",
           "probe": arm(sys.argv[1]), "acceptance": arm(sys.argv[2])},
          open("/dev/stdout", "w"))
PY
}
gate_verdict() { "$PY" -m scripts.repipe.verify --observation "$GATE_OBS" --json 2>/dev/null \
                   | "$PY" -c 'import json,sys;print(json.load(sys.stdin).get("verdict"))'; }

# probe PASSES (kuna --version exits 0) + acceptance FAILS (it does not exit 99)
mk_obs '{"exit_code":{"eq":0}}' '{"exit_code":{"eq":99}}'
V="$(gate_verdict)"
[ "$V" = "admitted" ] && { ok "probe PASS + acceptance FAIL -> admitted"; SAW_REPRODUCING=1; } \
                      || bad "expected admitted, got $V"
# acceptance ALREADY passes -> the tester was wrong, and that ledger must not be silent
mk_obs '{"exit_code":{"eq":0}}' '{"exit_code":{"eq":0}}'
V="$(gate_verdict)"
[ "$V" = "already-supported" ] && ok "acceptance already passes -> already-supported" \
                              || bad "expected already-supported, got $V"
# the probe does not reproduce -> noise, never dispatched
mk_obs '{"exit_code":{"eq":99}}' '{"exit_code":{"eq":0}}'
V="$(gate_verdict)"
[ "$V" = "not-reproducible" ] && ok "probe does not reproduce -> not-reproducible" \
                             || bad "expected not-reproducible, got $V"
# an arm that arrived as a JSON string codex could not serialise cleanly is UNRUNNABLE, not a
# traceback: one such observation used to take down the gate for the whole round with it
"$PY" - > "$GATE_OBS" <<'PY'
import json
arm = '{"schema":"re-probe/1","kind":"cli","cmd":["{{KUNA}}","--version"],"expect":{"stdout_matches":["\\s+"]}}'
json.dump({"kind": "bad-ux", "title": "gate self-test", "what_i_wanted": "x",
           "what_kuna_did": "y", "severity": "minor",
           "probe": arm, "acceptance": arm}, open("/dev/stdout", "w"))
PY
V="$(gate_verdict)"
[ "$V" = "unrunnable" ] && ok "an arm that will not parse -> unrunnable, not a traceback" \
                       || bad "expected unrunnable, got $V"

for f in probe-zero-functions accept-zero-functions accept-functions-size; do
  [ -f "$FIX/$f.json" ] || bad "missing fixture $f.json"
done

head_ "L1  a probe pointed at the wrong binary REFUSES rather than lying"
WRONG="$("$PY" -m scripts.repipe.probe check "$FIX/probe-zero-functions.json" --json 2>&1)"
echo "$WRONG" | grep -qi 'not found\|mismatch' \
  && ok "target verification refuses a misresolved {{BIN}}" \
  || bad "a misresolved target produced a verdict instead of an error"

head_ "L1  the tester report schema is accepted by strict structured-output mode"
# codex sends --output-schema straight to the API's response_format, which enforces rules
# ordinary JSON Schema does not. Nothing local rejects a bad schema, so the first live tester
# 400s. Every rule below was learned from exactly that.
if "$PY" -m scripts.repipe.strictschema "$REPO/tools/repipe/schema/report.schema.json" >/dev/null 2>&1; then
  ok "report.schema.json is strict-mode clean"
else
  bad "report.schema.json would 400 on a live codex run" "$("$PY" -m scripts.repipe.strictschema "$REPO/tools/repipe/schema/report.schema.json" 2>&1 | head -3)"
fi
"$PY" - <<'PY' >/dev/null 2>&1 && ok "the checker actually catches a violation" || bad "strictschema check is inert"
import sys
sys.path.insert(0, "/home/mahaloz/github/kuna")
from scripts.repipe import strictschema
# an object missing a key from `required`, and a bare enum with no type
bad_schema = {"type": "object", "additionalProperties": False,
              "required": ["a"], "properties": {"a": {"type": "string"},
                                                "b": {"enum": ["x", "y"]}}}
probs = strictschema.check(bad_schema)
assert len(probs) >= 2, probs
assert not strictschema.check(strictschema.strictify(bad_schema))
PY

head_ "L1  a probe cannot execute arbitrary code"
# A probe's argv is authored by an LLM and replayed by verify.py in the MAIN tree, outside
# the tester's sandbox. Without the allowlist this is remote code execution.
EVIL="$SMOKE_STATE/evil.json"
rm -f /tmp/repipe-smoke-pwned
# The last two are the bypasses a basename-only allowlist would miss: a tester has
# workspace-write, so it can drop a file called `kuna` and name it by path.
mkdir -p "$SMOKE_STATE/evil"
printf '#!/bin/sh\ntouch /tmp/repipe-smoke-pwned\n' > "$SMOKE_STATE/evil/kuna"; chmod +x "$SMOKE_STATE/evil/kuna"
ln -sf /bin/bash "$SMOKE_STATE/evil/objdump"
for CMD in '["bash","-c","touch /tmp/repipe-smoke-pwned"]' '["rm","-rf","/tmp/nothing"]' '["curl","http://example.invalid"]' "[\"$SMOKE_STATE/evil/kuna\"]" "[\"$SMOKE_STATE/evil/objdump\",\"-c\",\"touch /tmp/repipe-smoke-pwned\"]"; do
  "$PY" - "$FIX" "$CMD" > "$EVIL" <<'PY'
import json, sys
p = json.load(open(sys.argv[1] + "/probe-zero-functions.json"))
p["cmd"] = json.loads(sys.argv[2]); p.pop("probe_id", None); p.pop("target", None)
json.dump(p, open("/dev/stdout", "w"))
PY
  R="$("$PY" -m scripts.repipe.probe check "$EVIL" --work "$CH" --json 2>&1)"
  echo "$R" | grep -q 'may not execute'     && ok "refused $(echo "$CMD" | cut -c1-32)"     || bad "EXECUTED $(echo "$CMD" | cut -c1-32)" "$R"
done
[ -e /tmp/repipe-smoke-pwned ] && bad "a probe actually executed arbitrary code" || ok "nothing was executed"

head_ "L1  clustering collapses duplicates"
OBS="$SMOKE_STATE/obs.json"
"$PY" - "$FIX" > "$OBS" <<'PY'
import json, sys
F = sys.argv[1] + "/"
p = json.load(open(F + "probe-zero-functions.json"))
a = json.load(open(F + "accept-zero-functions.json"))
def o(title, hexid, tester, kind="silent-failure", wanted="function inventory"):
    return {"kind": kind, "title": title, "what_i_wanted": wanted,
            "what_kuna_did": "count 0 exit 0", "severity": "blocker",
            "probe": p, "acceptance": a, "_hexid": hexid, "_tester": tester, "_round": 1}
json.dump([
    o("kuna functions returns 0 functions on a stripped PIE", "64f1f7af", "t1"),
    o("functions --json is empty for a section-stripped binary", "60be2a60", "t2"),
    o("no functions found and no error reported", "6609e458", "t3"),
    o("kuna has no way to list strings", "64f1f7af", "t1", "missing-capability", "the string inventory"),
    o("cannot enumerate strings from kuna at all", "6883765e", "t2", "missing-capability", "string table"),
], open("/dev/stdout", "w"), indent=2)
PY
CL="$("$PY" -m scripts.repipe.cluster --round 1 --from-file "$OBS" --dry-run --json 2>/dev/null)"
NCL="$(echo "$CL" | "$PY" -c 'import json,sys;print(len(json.load(sys.stdin)))' 2>/dev/null)"
[ "$NCL" = "2" ] && { ok "5 observations -> 2 needs"; SAW_NEED=1; } || bad "expected 2 clusters, got $NCL" "$CL"
echo "$CL" | grep -q '"instances": 3' && ok "the 3-way duplicate merged into one need" || bad "duplicates not merged"

head_ "L1  collision avoidance"
"$PY" - <<'PY'
import sys; sys.path.insert(0, ".")
from scripts.repipe import needs, select
mk = lambda i, t: needs.Need(fields={"need_id": i, "title": i, "track": t, "status": "open",
                                     "severity": "major", "instances": 3, "challenges": ["a"],
                                     "rounds": [1], "touches": []})
picks = select.pick(k=3, needs_list=[mk("q1", "quality"), mk("q2", "quality"), mk("t1", "tooling")])
tracks = [p["need"].track for p in picks]
assert tracks.count("quality") == 1, "two option-adding builders dispatched: %s" % tracks
assert tracks.count("tooling") == 1, "tooling builder was blocked: %s" % tracks
print("OK")
PY
[ $? = 0 ] && ok "at most one quality builder; tooling runs alongside" || bad "lease algebra wrong"

head_ "L1  regressions the review found (each of these shipped broken once)"
# newline in a tester-authored title used to destroy every front-matter field after it
"$PY" - <<'PY' >/dev/null 2>&1 && ok "a newline in a title cannot corrupt front matter" || bad "front-matter escaping regressed"
import sys, tempfile, os
sys.path.insert(0, "/home/mahaloz/github/kuna")
from scripts.repipe import needs
evil = "one\nstatus: HIJACKED\ntrack: quality"
n = needs.Need(fields={"need_id": "x", "title": evil, "track": "tooling", "severity": "major"})
p = tempfile.mktemp(suffix=".md"); open(p, "w").write(needs.render(n))
m = needs.parse(p); os.unlink(p)
assert m.title == evil and m.track == "tooling", (m.title, m.track)
PY
# a probe and its own negation both passing made the gate report already-supported
"$PY" - <<'PY' >/dev/null 2>&1 && ok "exists/absent cannot both pass on a mixed array" || bad "json quantifier regressed"
import sys
sys.path.insert(0, "/home/mahaloz/github/kuna")
from scripts.repipe import probe
obs = {"runs": [{"exit_code": 0, "stdout": '{"f":[{"s":1},{}]}', "stderr": "",
                 "wall_ms": 1, "timed_out": False, "error": None, "stdout_bytes": 10}],
       "baselines": {}}
mk = lambda op: {"schema": "re-probe/1", "kind": "cli", "cmd": ["x"], "timeout_s": 5,
                 "expect": {"json": [{"path": "f[*].s", "op": op}]}}
e = probe.evaluate(probe.normalize(mk("exists")), obs)["passed"]
a = probe.evaluate(probe.normalize(mk("absent")), obs)["passed"]
assert not (e and a), (e, a)
PY
# a catastrophic-backtracking regex used to hang the gate forever
timeout 60 "$PY" - <<'PY' >/dev/null 2>&1 && ok "a pathological regex is time-bounded" || bad "regex budget regressed (hung or errored)"
import sys
sys.path.insert(0, "/home/mahaloz/github/kuna")
from scripts.repipe import probe
obs = {"runs": [{"exit_code": 0, "stdout": "a"*40+"!", "stderr": "", "wall_ms": 1,
                 "timed_out": False, "error": None, "stdout_bytes": 41}], "baselines": {}}
probe.evaluate(probe.normalize({"schema": "re-probe/1", "kind": "cli", "cmd": ["x"],
                                "timeout_s": 5, "expect": {"stdout_matches": [r"(a+)+$"]}}), obs)
PY
# concurrent captain ticks used to race round.json
( for i in 1 2 3 4 5 6; do "$PY" -m scripts.repipe.captain --transition build B_PLAN >/dev/null 2>&1 & done; wait ) 2>/dev/null
NWIN="$(grep -c '"to": "B_PLAN"' "$SMOKE_STATE"/rounds/*/transitions.jsonl 2>/dev/null | head -1)"
[ "${NWIN:-0}" = "1" ] && ok "6 racing transitions -> exactly 1 accepted" || bad "round.json race: $NWIN accepted, expected 1"

head_ "L1  the backlog round-trips, at whatever size it is"
# An EMPTY backlog is the healthy steady state, not a failure: every filed need has been
# built and its acceptance probe promoted into tests/cli/, which is where the durable
# guarantee lives. The records are working notes; the probes are the contract.
NL="$("$PY" -m scripts.repipe.needs list --json 2>/dev/null | "$PY" -c 'import json,sys;print(json.load(sys.stdin)["count"])' 2>/dev/null)"
ok "backlog holds ${NL:-0} need(s)"
CLI_N="$(ls "$REPO"/tests/cli/*.json 2>/dev/null | wc -l)"
[ "${CLI_N:-0}" -ge 1 ] \
  && ok "$CLI_N promoted regression probe(s) in tests/cli/" \
  || bad "tests/cli/ is empty -- nothing guards the gaps the loop has already closed"
"$PY" -m scripts.repipe.clitests >/dev/null 2>&1 \
  && ok "every promoted probe still passes" \
  || bad "a promoted probe regressed" "$("$PY" -m scripts.repipe.clitests 2>&1 | tail -4)"
for f in "$REPO"/docs/re-needs/*.md; do
  [ -e "$f" ] || break
  "$PY" -c "
import sys; sys.path.insert(0,'$REPO')
from scripts.repipe import needs
n = needs.parse('$f')
assert needs.render(n) == open('$f').read(), 'round-trip differs'
" 2>/dev/null && ok "round-trip $(basename "$f")" || bad "round-trip $(basename "$f")"
done

head_ "L1  every status surface agrees with the inventory about a LIVE agent"

# Round 2 shipped with BOTH status surfaces silently wrong on a loop that was running
# perfectly: the CLI read the other pipeline's inventory (KUNA_PIPELINE_STATE_DIR unset,
# so `.kuna-pipeline/` -- empty), and the dashboard read meta["tracks"]["test"]["state"]
# and transition["track"] where captain.py writes flat meta["test"] and ["machine"]. Both
# rendered "0 agents" and "--" while three testers held three slots. Checking routes and
# JSON shape did not catch either, because both answered 200 with a well-formed document
# that was simply about nothing. So: plant a slot, and require every surface to SEE it.
SMOKE_SLOT="smoke-live-$$"; export SMOKE_SLOT
# The smoke's state dir is a fresh mktemp with no pool caps set, so the cap defaults to 0
# and every acquire is refused. The live loop gets its caps from captain.set_caps().
"$PY" -m scripts.pipeline.state slot-cap --pool tester --cap 3 >/dev/null 2>&1
if "$PY" -m scripts.pipeline.state slot-acquire --pool tester --id "$SMOKE_SLOT" --pid $$ \
     >/dev/null 2>&1; then
  ok "planted a live tester slot ($SMOKE_SLOT)"

  if "$PY" -m scripts.repipe.status --json 2>/dev/null \
      | "$PY" -c 'import json,sys,os
d = json.load(sys.stdin)
held = (d.get("slots", {}).get("tester") or {}).get("held") or {}
sys.exit(0 if os.environ["SMOKE_SLOT"] in held else 1)'; then
    ok "scripts.repipe.status sees it"
  else
    bad "scripts.repipe.status does not see a live tester slot" \
        "$("$PY" -m scripts.repipe.status 2>&1 | head -2)"
  fi

  # The bare-invocation case is the one that broke: no KUNA_PIPELINE_STATE_DIR in the
  # environment, which is how docs/re-pipeline.md tells an operator to run it. It cannot be
  # tested by planting a slot -- with the variable unset the module correctly resolves the
  # REPO's state dir, not this smoke's temp one -- so assert the resolution itself: the
  # inventory it would read must sit under the repipe state dir, never `.kuna-pipeline`.
  if ( unset KUNA_PIPELINE_STATE_DIR
       "$PY" -c '
import importlib, sys
from scripts.repipe import config, status          # importing status is what pins the var
from scripts.pipeline import state as pstate
want = str(config.state_dir())
got = str(pstate.inventory_path() if hasattr(pstate, "inventory_path") else pstate._inventory_path())
sys.exit(0 if got.startswith(want) else 1)
' ) 2>/dev/null; then
    ok "... and resolves the repipe inventory with the env unset"
  else
    bad "status resolves the wrong inventory when KUNA_PIPELINE_STATE_DIR is unset" \
        "$( unset KUNA_PIPELINE_STATE_DIR; "$PY" -c '
from scripts.repipe import config, status
from scripts.pipeline import state as pstate
print("want prefix:", config.state_dir())
print("got        :", pstate._inventory_path())' 2>&1 | head -3)"
  fi

  "$PY" -m scripts.pipeline.state slot-release --pool tester --id "$SMOKE_SLOT" >/dev/null 2>&1
else
  bad "could not plant a tester slot to test the status surfaces against"
fi

# A lane state the captain actually wrote must render, not fall back to null. This is the
# dashboard half of the same bug; _collect_rounds is pure, so it needs no server.
"$PY" - <<'PYEOF' 2>/dev/null && ok "webui renders a lane state captain.py wrote" \
  || bad "webui cannot read captain.py's round document"
import json, sys, tempfile, os
from pathlib import Path
sys.path.insert(0, os.environ.get("REPO") or os.environ.get("PYTHONPATH", "").split(":")[0] or ".")
from scripts.repipe import webui
d = Path(tempfile.mkdtemp())
r = d / "rounds" / "7"
r.mkdir(parents=True)
# exactly the shape scripts/repipe/captain.py::save_round writes
(r / "round.json").write_text(json.dumps(
    {"round": 7, "supervisor": "RUNNING", "test": "T_FANOUT", "build": "B_PLAN"}))
(r / "transitions.jsonl").write_text(json.dumps(
    {"ts": 0, "machine": "test", "from": "T_PLAN", "to": "T_FANOUT"}) + "\n")
rounds = webui._collect_rounds(d)
got = (rounds[0].get("test_state"), rounds[0].get("build_state"))
assert got == ("T_FANOUT", "B_PLAN"), got
PYEOF

head_ "L1  dashboard serves every route with no gh call per request"
PORT=$(( 18700 + RANDOM % 900 ))
"$PY" -m scripts.repipe.webui --port "$PORT" --bind 127.0.0.1 --no-checks >"$SMOKE_STATE/webui.log" 2>&1 &
WEB_PID=$!
for _ in $(seq 1 30); do
  curl -s -o /dev/null --max-time 2 "http://127.0.0.1:$PORT/healthz" 2>/dev/null && break
  sleep 1
done
if ! curl -s -o /dev/null --max-time 5 "http://127.0.0.1:$PORT/healthz" 2>/dev/null; then
  bad "dashboard never came up on :$PORT" "$(tail -3 "$SMOKE_STATE/webui.log" 2>/dev/null)"
fi
for r in / /healthz /api/state /api/needs /api/rounds /api/agents /api/corpus /api/acceptance; do
  CODE="$(curl -s -o /dev/null -w '%{http_code}' --max-time 10 "http://127.0.0.1:$PORT$r" 2>/dev/null)"
  [ "$CODE" = "200" ] && ok "GET $r" || bad "GET $r -> $CODE"
done
CODE="$(curl -s -o /dev/null -w '%{http_code}' --max-time 10 "http://127.0.0.1:$PORT/api/agent/..%2f..%2fetc%2fpasswd/log" 2>/dev/null)"
[ "$CODE" = "400" ] || [ "$CODE" = "404" ] && ok "path traversal rejected ($CODE)" || bad "traversal returned $CODE"
curl -s --max-time 10 "http://127.0.0.1:$PORT/api/corpus" 2>/dev/null | grep -q 'N@n0m4ch1n3sS0n' \
  && bad "THE DASHBOARD LEAKS A FLAG" || ok "corpus endpoint leaks no ground truth"
kill "$WEB_PID" 2>/dev/null; wait "$WEB_PID" 2>/dev/null

# --------------------------------------------------------------- canaries ---
head_ "canaries (a silently broken run must not report success)"
[ "$SAW_REPRODUCING" = 1 ] && ok "the gate returned a decisive verdict" || bad "the gate never returned `admitted` -- the runner is broken, not the corpus"
[ "$SAW_SANDBOX" = 1 ]     && ok "sandbox assertions ran"             || bad "sandbox assertions were SKIPPED"
[ "$SAW_EXTRAS_KEPT" = 1 ] && ok "the redactor kept at least one extras file" || bad "the redactor dropped EVERYTHING (over-redaction hides a broken allowlist)"
[ "$SAW_NEED" = 1 ]        && ok "clustering produced needs"          || bad "clustering produced nothing"

printf '\n\033[1msmoke: %d passed, %d failed\033[0m\n' "$PASS" "$FAIL"
exit $(( FAIL > 0 ))
