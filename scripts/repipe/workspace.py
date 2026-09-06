"""Build one tester arena: everything a codex tester may see, and nothing else.

The arena is the tester's whole world (`codex exec --cd $ARENA`). Everything the dataset knows
about the answer -- `meta.json` and its plaintext `ground_truth.flag`, `verifier.py`,
`original.zip`, the `solutions/` writeups -- stays outside it by construction, and every
`extras/` file is filtered by `redact.classify` before it is copied. Containment is structural,
not a request: a prompt-level "please do not look" would be worthless because codex's
`-s workspace-write` restricts writes, not reads.

    <out>/
      target/<primary>             meta.json -> detected.primary.path, chmod 0755
      target/<other binaries>      the rest of files.binaries[], chmod 0755
      extras/<surviving files>     whatever redact.classify said "copy" to
      bin/kuna  bin/ida-decompile  logging shims; the launcher prepends bin/ to PATH
      bin/_shimlog.py              the shims' JSON-line appender
      TASK.md  AGENTS.md           the statement and the tool protocol; no ground truth
      notes/toolcalls.jsonl        written by the shims, one line per tool call
      .repipe/build.json           what was copied and what was dropped, and why

Two dataset facts drive the copying. **Exec bits are broken**: 4 of 287 shipped binaries are
mode 600 and 54 are 644, so every copy is chmod 0755 -- otherwise the tester's first `./target`
run fails for a reason that has nothing to do with kuna. **`bin/` preserves recursive-extraction
path shapes** (`bin/GiveMeMoney.zip.__x/33bits/KeyVal2.exe`), and two binaries in one challenge
routinely share a basename, so the tree under `bin/` is mirrored under `target/` and the primary
is always resolved through `detected.primary.path`, never by globbing.

**The shims are load-bearing.** kuna emits no timing at all in any of its JSON, so
`notes/toolcalls.jsonl` is the pipeline's only per-call latency signal; it also turns tool usage
counts and the kuna-vs-IDA time split into measurements rather than model self-report, and lets
a probe be auto-derived from a real recorded argv instead of one a model transcribed.

`sanity_check()` is the assertion side of all of the above, so the smoke test and the captain
can both refuse to dispatch a tester into a contaminated arena.

CLI:
    python -m scripts.repipe.workspace build HEXID [--round N] [--out DIR] [--force] [--json]
    python -m scripts.repipe.workspace check ARENA [--hexid H] [--json]
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import stat
import subprocess
import sys
import time
from pathlib import Path

from . import config
from . import redact

BIN_MODE = 0o755
SHIM_MODE = 0o755

# Never inside an arena, at any depth, under any name.
FORBIDDEN_NAMES = ("meta.json", "verifier.py", "original.zip")
FORBIDDEN_DIRS = ("solutions",)

# sanity_check reads whole files; a target binary can be hundreds of MB and is exempt anyway.
MAX_SCAN_BYTES = 16 * 1024 * 1024

# Exempt from the leak scan entirely: the binaries themselves and any decompiler database
# derived from them. For many challenges the flag *is* in the binary -- the binary is the task.
DERIVED_DIRS = ("target", ".declib")

# Written by the tester, not by the build. A correct solve legitimately names the flag here, so
# only the answer-independent tags (dataset path, writeup archive) are violations in these.
TESTER_PATHS = ("notes", "report.json")


# --- dataset access (read-only) ---------------------------------------------

def challenge_dir(hexid: str) -> Path:
    return config.dataset_root() / "challenges" / hexid


def load_meta(hexid: str) -> dict:
    with open(challenge_dir(hexid) / "meta.json") as fh:
        return json.load(fh)


def arena_path(hexid: str, round_n: int) -> Path:
    return config.arena_dir() / str(round_n) / hexid


def primary_rel(meta: dict) -> str:
    return ((meta.get("detected") or {}).get("primary") or {}).get("path") or ""


def _target_rel(rel: str) -> str:
    """Map a dataset-relative binary path to its place under target/.

    Strips the leading `bin/` (the arena has no `bin/` for binaries -- that name is the shim
    directory) while keeping every level below it, because the extraction shape disambiguates
    two same-named binaries in one challenge.
    """
    rel = rel.replace("\\", "/").lstrip("/")
    parts = [p for p in rel.split("/") if p not in ("", ".", "..")]
    if parts and parts[0] == "bin":
        parts = parts[1:]
    return "/".join(parts)


def _extras_rel(rel: str) -> str:
    rel = rel.replace("\\", "/").lstrip("/")
    parts = [p for p in rel.split("/") if p not in ("", ".", "..")]
    if parts and parts[0] == "extras":
        parts = parts[1:]
    return "/".join(parts)


# --- writing ----------------------------------------------------------------

def _write(path: Path, text: str, mode: int = 0o644) -> None:
    """Atomic write: temp sibling, then os.replace."""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = str(path) + ".tmp"
    with open(tmp, "w") as fh:
        fh.write(text)
    os.chmod(tmp, mode)
    os.replace(tmp, path)


def _copy_exec(src: Path, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(src, dst)
    os.chmod(dst, BIN_MODE)


# --- the shims --------------------------------------------------------------

_SHIMLOG_PY = '''"""Append one JSON line to notes/toolcalls.jsonl. Called by the bin/ shims.

Separate from the shims because correct JSON escaping of an arbitrary argv is not something a
POSIX shell can do; the flock keeps two concurrent tool calls from interleaving a line.
"""
import argparse
import fcntl
import json
import os
import sys
import time


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--log", required=True)
    ap.add_argument("--tool", required=True)
    ap.add_argument("--argv-file", required=True)
    ap.add_argument("--exit", type=int, default=0)
    ap.add_argument("--seconds", type=float, default=0.0)
    ap.add_argument("--stdout-bytes", type=int, default=0)
    ap.add_argument("--stderr-bytes", type=int, default=0)
    ap.add_argument("--stdout-sha1", default="")
    a = ap.parse_args(argv)

    with open(a.argv_file, "rb") as fh:
        raw = fh.read()
    args = [x.decode("utf-8", "replace") for x in raw.split(b"\\0") if x]

    rec = {
        "t": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "tool": a.tool,
        "argv": args,
        "cwd": os.getcwd(),
        "exit": a.exit,
        "seconds": round(a.seconds, 4),
        "stdout_bytes": a.stdout_bytes,
        "stderr_bytes": a.stderr_bytes,
        "stdout_sha1": a.stdout_sha1,
    }
    os.makedirs(os.path.dirname(a.log) or ".", exist_ok=True)
    with open(a.log, "a") as fh:
        fcntl.flock(fh, fcntl.LOCK_EX)
        fh.write(json.dumps(rec, separators=(",", ":")) + "\\n")
        fh.flush()
        fcntl.flock(fh, fcntl.LOCK_UN)
    return 0


if __name__ == "__main__":
    sys.exit(main())
'''

_SHIM_HEAD = '''#!/bin/sh
# {title}
#
# Transparent: same stdout, same stderr, same exit code as the real tool. It cannot literally
# exec(2) the real tool because the accounting -- exit code, wall seconds, stdout size and
# digest -- only exists after the tool returns, so stdout/stderr are captured to temp files and
# replayed verbatim. ARENA is derived from this script's own location so the arena stays
# relocatable; $REPIPE_ARENA overrides it.
set -u
ARENA=${{REPIPE_ARENA:-$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)}}
LOG="$ARENA/notes/toolcalls.jsonl"
REAL=${{{real_var}:-{real_default}}}
'''

_SHIM_TAIL = '''
TMPD=$(mktemp -d "${{TMPDIR:-/tmp}}/repipe-{tool}.XXXXXX") || exec "$REAL" "$@"
printf '%s\\0' "$@" > "$TMPD/argv"
START=$(date +%s.%N)
{invoke} >"$TMPD/out" 2>"$TMPD/err"
RC=$?
END=$(date +%s.%N)
SECS=$(awk -v a="$START" -v b="$END" 'BEGIN {{ printf "%.4f", b - a }}')
OB=$(wc -c < "$TMPD/out" | tr -d ' ')
EB=$(wc -c < "$TMPD/err" | tr -d ' ')
SHA=$(sha1sum "$TMPD/out" | cut -d' ' -f1)
python3 "$ARENA/bin/_shimlog.py" --log "$LOG" --tool {tool} --argv-file "$TMPD/argv" \\
    --exit "$RC" --seconds "$SECS" --stdout-bytes "$OB" --stderr-bytes "$EB" \\
    --stdout-sha1 "$SHA" >/dev/null 2>&1 || true
cat "$TMPD/out"
cat "$TMPD/err" >&2
rm -rf "$TMPD"
exit $RC
'''

# `--project-dir` is a `load`-only flag in the declib CLI (verified: no other subcommand
# accepts it), so it is injected only there, and only when the caller did not pass one.
_IDA_SETUP = '''
# EVERYTHING declib writes must land inside the arena. The tester runs under codex's
# `workspace-write` sandbox, which blocks writes outside its workspace -- and declib puts its
# unix socket under TMPDIR (observed: /tmp/declib_server_<id>/decompiler.sock), so with the
# default TMPDIR the server cannot start at all and every reference call fails with rc=1.
DECLIB_SERVER_REGISTRY="$ARENA/.declib/servers"

# TMPDIR cannot live in the arena, and this is a hard limit rather than a preference.
# declib binds an AF_UNIX socket at $TMPDIR/declib_server_<10 hex>/decompiler.sock -- 41
# bytes of suffix -- and sun_path is 108 bytes. The arena root is already ~71
# (.kuna-repipe/arena/<round>/<24-char hexid> under the repo), so even `$ARENA/t` computes
# to 114 and bind(2) fails. The server then exits before registering, having logged nothing
# past "Using headless interface", which is why this read for two rounds as "IDA is broken".
# So: a short directory under the system temp, one per arena, cleaned with the round.
# If codex's workspace-write sandbox refuses it, the reference call fails exactly as it does
# today -- this cannot regress anything -- and the guard below says so in one line instead
# of leaving an empty server log.
TMPDIR="${TMPDIR:-/tmp}/kr-$(printf '%s' "$ARENA" | cksum | cut -d' ' -f1)"
_SOCK_LEN=$(( ${#TMPDIR} + 41 ))
if [ "$_SOCK_LEN" -gt 108 ]; then
  echo "ida-decompile: TMPDIR=$TMPDIR makes the declib socket path ${_SOCK_LEN} bytes," \
       "over the 108-byte AF_UNIX limit; set TMPDIR to something shorter" 1>&2
fi
XDG_STATE_HOME="$ARENA/.declib/state"
XDG_CACHE_HOME="$ARENA/.declib/cache"
# XDG_CONFIG_HOME too, and this one is not optional: declib takes a lock on
# $XDG_CONFIG_HOME/declib/DecLibConfig.lock at startup, and bwrap mounts $HOME read-only to
# keep the tester away from credentials -- so without this every reference call died with
#   Failed to start server: [Errno 30] Read-only file system:
#   '/home/mahaloz/.config/declib/DecLibConfig.lock'
# and the IDA comparison leg was silently unavailable for two whole rounds.
XDG_CONFIG_HOME="$ARENA/.declib/config"
export DECLIB_SERVER_REGISTRY TMPDIR XDG_STATE_HOME XDG_CACHE_HOME XDG_CONFIG_HOME
mkdir -p "$ARENA/.declib/servers" "$ARENA/.declib/projects" "$TMPDIR" \
         "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME/declib"
# Seed the config from the operator's real one rather than letting declib regenerate a
# default: it carries [headless_binary_paths] and [plugins_paths], and an empty config makes
# the backend unresolvable -- a different failure with the same result. `save_location` is
# rewritten so declib does not write back to the read-only original. Copy once; a rerun in a
# live arena must not clobber whatever the round has already written.
if [ ! -f "$XDG_CONFIG_HOME/declib/DecLibConfig.toml" ]; then
  # No backslash escapes here on purpose. This text is a Python string inside a Python
  # string, so a `\\"` written in the generator collapses to a bare `"` by the time the
  # shim is on disk -- which closed the sed expression and emitted
  #   save_location = /path/to/file
  # with the quotes gone, i.e. invalid TOML. Holding the quote in $_Q survives both passes.
  _Q='"'
  _SL="$XDG_CONFIG_HOME/declib/DecLibConfig.toml"
  if [ -f "$HOME/.config/declib/DecLibConfig.toml" ]; then
    sed "s|^save_location = .*|save_location = $_Q$_SL$_Q|" \
        "$HOME/.config/declib/DecLibConfig.toml" > "$_SL"
  else
    : > "$_SL"
  fi
fi

PROJDIR="$ARENA/.declib/projects"

# `load` without an explicit --backend silently falls back to angr, which would make the
# whole "compare against IDA" leg a comparison against something else entirely. IDA is the
# reference the design asks for, so it is the default here and the caller can still override.
INJECT=""
BACKEND=""
if [ "${1:-}" = "load" ]; then
    case " $* " in
        *" --project-dir "*|*" --project-dir="*) ;;
        *) INJECT=1 ;;
    esac
    case " $* " in
        *" --backend "*|*" --backend="*) ;;
        *) BACKEND="${REPIPE_IDA_BACKEND:-ida}" ;;
    esac
fi
'''


def _shim_kuna(real: str) -> str:
    head = _SHIM_HEAD.format(
        title="Shim for kuna. Logs every call to notes/toolcalls.jsonl -- kuna emits no timing\n# of its own, so this file is the pipeline's only per-call latency signal.",
        real_var="REPIPE_REAL_KUNA", real_default=real)
    tail = _SHIM_TAIL.format(tool="kuna", invoke='"$REAL" "$@"')
    return head + tail


def _shim_ida(real: str) -> str:
    head = _SHIM_HEAD.format(
        title="Shim for the declib/IDA reference CLI. IDA is a logged last resort, never a\n"
              "# parallel track: every call here auto-drafts an observation, because \"kuna made\n"
              "# testers leave N times this round\" is a headline metric.\n#\n"
              "# The server registry and the project DBs (IDA's .i64 among them) are forced inside\n"
              "# the arena so they die with the round, as does declib's unix socket -- codex's\n"
              "# workspace-write sandbox blocks writes outside the workspace, and the default\n"
              "# TMPDIR put the socket in /tmp, so every reference call failed with rc=1.\n"
              "# HOME is deliberately NOT re-pointed: IDA's registration lives in\n"
              "# ~/.idapro/ida.reg and moving HOME would break the backend.",
        real_var="REPIPE_REAL_DECOMPILER", real_default=real)
    invoke = ('if [ -n "$INJECT" ]; then\n'
              '    shift\n'
              '    set -- load --project-dir "$PROJDIR" "$@"\n'
              'fi\n'
              'if [ -n "$BACKEND" ]; then\n'
              '    set -- "$@" --backend "$BACKEND"\n'
              'fi\n'
              '"$REAL" "$@"')
    tail = _SHIM_TAIL.format(tool="ida", invoke=invoke)
    return head + _IDA_SETUP + tail


# --- TASK.md / AGENTS.md ----------------------------------------------------

def _goal(meta: dict) -> tuple:
    gt = meta.get("ground_truth") or {}
    if gt.get("verifier_interface") == "name+serial":
        return ("a username and a serial that the binary accepts for it",
                "Report both in `report.json` as `solution.name` and `solution.serial`.")
    if gt.get("has_unique_flag"):
        return ("the single flag string the binary accepts",
                "Report it in `report.json` as `solution.flag`, exactly, with no wrapper.")
    return ("whatever the binary accepts as correct -- a flag, or a name and matching serial",
            "Report it in `report.json` under `solution`, and say which form it is.")


def _task_md(meta: dict, primary_t: str, others: list, extras: list) -> str:
    d = meta.get("declared") or {}
    det = meta.get("detected") or {}
    prim = det.get("primary") or {}
    obf = (meta.get("obfuscation") or {}).get("classes") or []
    goal, how = _goal(meta)
    minutes = max(1, int(config.TESTER_TIMEOUT) // 60)

    def clean(text):
        return redact.sanitize(text, meta)

    rows = [
        ("platform", clean(d.get("platform") or det.get("formats") and ", ".join(det["formats"]) or "unknown")),
        ("arch", clean(prim.get("arch") or d.get("arch") or "unknown")),
        ("format", clean(prim.get("format") or "unknown")),
        ("size", "%d bytes" % prim.get("size", 0)),
        ("file(1)", clean(prim.get("file") or "unknown")),
        ("obfuscation", clean(", ".join(obf)) if obf else "none detected"),
        ("author difficulty", "%s / 6" % d.get("difficulty") if d.get("difficulty") else "unstated"),
    ]
    lines = ["# Task: %s" % clean(meta.get("name") or meta["hexid"]), ""]
    lines += ["| | |", "|---|---|"]
    lines += ["| %s | %s |" % (k, v) for k, v in rows]
    lines += ["", "## Target", "",
              "Primary: `target/%s`" % primary_t, ""]
    if others:
        lines += ["Also shipped (same challenge, may or may not matter):", ""]
        lines += ["- `target/%s`" % o for o in others]
        lines += [""]
    lines += ["Every binary is a verbatim copy of the author's, `chmod 0755` (the archive's",
              "exec bits are unreliable). Nothing has been patched.", ""]
    lines += ["## What to produce", "",
              "Recover **%s**." % goal, "",
              how, "",
              "If you cannot, say so: `outcome: gave_up` with an honest `gave_up_reason` is a",
              "**successful** run for this pipeline. An invented answer is not.", ""]
    lines += ["## Budget", "",
              "%d minutes of wall clock. Spend it on the parts that teach you something about" % minutes,
              "the tooling; a partial solve with good friction evidence beats a rushed guess.", ""]
    lines += ["## What you have", ""]
    if extras:
        lines += ["Author-supplied text, redacted (spoiler files were withheld):", ""]
        lines += ["- `extras/%s`" % e for e in extras]
    else:
        lines += ["No author text survived redaction, so the binary is the entire statement."]
    lines += ["", "## What you do not have", "",
              "The challenge metadata, the author's solution, and any hint, serial, key or source",
              "file are deliberately absent, and so is the dataset they came from. Do not go",
              "looking for them; do not fetch anything from the network. If you find yourself",
              "holding an answer you did not derive from the binary, stop and say so in the report.",
              ""]
    return "\n".join(lines)


_AGENTS_MD_HEAD = """# Tool protocol for this arena

`bin/` is on your PATH and holds the only two tools you should reach for. Call them by bare
name (`kuna`, `ida-decompile`) -- never by an absolute path to the real binary, because the
bare name is the shim and the shim is what records the run.

## kuna -- your primary tool

`kuna --help` lists its {n} subcommands: {subcommands}. `kuna docs` is the embedded manual and
`kuna catalog --json` the full option surface; the repo's `docs/cli.md` and `docs/options.md`
document both.

That list is read out of the binary in this arena at build time, so it is the surface you
actually have. Do NOT assume a subcommand is absent because a previous round said so -- this
pipeline ships CLI capability every round, and a brief that froze the surface is how a tester
re-files a gap that was closed two rounds ago. Check `kuna <subcommand> --help` before
concluding something cannot be done.

What is structurally true of every invocation, and worth not rediscovering:

- There is no server mode and no persistence between invocations. Every call is a cold load of
  the whole binary, so ten calls pay the load ten times; prefer one `decompile-all` over a loop.
- kuna reports no timing of its own. If a call felt slow, the number lives in
  `notes/toolcalls.jsonl` and nowhere else.
- An empty or successful-looking result is not proof: subcommands can exit 0 with
  `{{"count": 0}}` on a binary whose functions kuna simply failed to find. Cross-check anything
  surprising against `ida-decompile` before you believe it -- a wrong answer delivered
  confidently is the single most valuable thing you can find here.
"""

_AGENTS_MD_TAIL = """## ida-decompile -- the reference, and a last resort

`ida-decompile` is the declib CLI over IDA Pro. It can do things kuna cannot, and it keeps its
servers and databases inside this arena.

**Try kuna first, every time.** Every `ida-decompile` call is logged and automatically drafts an
observation against kuna, because "how often did kuna push a tester to the reference tool" is a
headline number for this pipeline. Reaching for it is allowed and sometimes correct -- reaching
for it *silently*, or *first*, defeats the measurement.

## Everything is logged

Both shims append one line per call to `notes/toolcalls.jsonl`:
`{"t","tool","argv","cwd","exit","seconds","stdout_bytes","stderr_bytes","stdout_sha1"}`.
kuna reports no timing of its own, so this file is the only latency evidence that exists. Two
consequences: do not edit or truncate it, and prefer one deliberate invocation over a shell loop
that fires forty, because each line is a datum.

## The point of the exercise

Solving the challenge is the vehicle. The deliverable is **every place kuna was missing, wrong,
slow, or more expensive than it should have been**, each with a command someone else can re-run
to see the same thing. An observation without a replayable command cannot enter the backlog.

## Scope

Write only inside this directory. `notes/` and `report.json` are yours; `target/` and `extras/`
are inputs -- copy a binary before you patch it. Do not read outside the arena and do not use
the network.
"""


def _agents_md(kuna_bin) -> str:
    """The tool protocol, with kuna's subcommand list read out of the binary being shipped.

    Frozen prose rots one round after it is written: round 4's arenas still told testers there
    was no `xrefs`, `strings` or `disassemble` subcommand and that `decompile` rejected
    `--json`, all four of which had shipped by then. Deriving the list means a brief can only
    ever be as wrong as the binary in the arena.
    """
    names = []
    try:
        r = subprocess.run([str(kuna_bin), "--help"], capture_output=True, text=True,
                           timeout=60)
        for line in (r.stdout + r.stderr).splitlines():
            if line.startswith("usage: kuna <") and ">" in line:
                names = line[line.index("<") + 1:line.index(">")].split("|")
                break
    except (OSError, subprocess.SubprocessError):
        names = []
    if not names:
        raise RuntimeError("cannot read kuna's subcommand list from %s --help" % kuna_bin)
    head = _AGENTS_MD_HEAD.format(n=len(names),
                                  subcommands=", ".join("`%s`" % n for n in names))
    return head + _AGENTS_MD_TAIL


# --- build ------------------------------------------------------------------

def build(hexid: str, round_n: int = 0, out=None, force: bool = False) -> Path:
    """Create the arena for one challenge and return its path.

    Rebuilding is destructive, so an arena that already holds a tester's WORK is refused
    unless ``force``. A re-entered T_WORKSPACE tick (a captain crash and resume is the normal
    way that happens) would otherwise delete a live tester's report.json and its
    toolcalls.jsonl -- the only record of what that agent actually did -- and the round would
    look merely empty rather than damaged.
    """
    cdir = challenge_dir(hexid)
    if not cdir.is_dir():
        raise FileNotFoundError("no such challenge: %s" % cdir)
    meta = load_meta(hexid)

    dest = Path(out) if out else arena_path(hexid, round_n)
    dest = dest.resolve()
    if not force and dest.exists():
        evidence = [q for q in ("report.json", "notes/toolcalls.jsonl")
                    if (dest / q).exists() and (dest / q).stat().st_size > 0]
        if evidence:
            raise FileExistsError(
                "%s already holds tester evidence (%s); refusing to rebuild over it. "
                "Pass force=True / --force only if you mean to discard that run."
                % (dest, ", ".join(evidence)))
    staging = dest.parent / (dest.name + ".building.%d" % os.getpid())
    if staging.exists():
        shutil.rmtree(staging)
    staging.mkdir(parents=True)

    prim_rel = primary_rel(meta)
    if not prim_rel:
        raise ValueError("%s: meta.json has no detected.primary.path" % hexid)
    bins = [b.get("path") for b in (meta.get("files") or {}).get("binaries") or []]
    ordered = [prim_rel] + [b for b in bins if b and b != prim_rel]

    copied_bins = []
    for rel in ordered:
        src = cdir / rel
        if not src.is_file():
            raise FileNotFoundError("%s: listed binary is missing: %s" % (hexid, rel))
        trel = _target_rel(rel)
        _copy_exec(src, staging / "target" / trel)
        copied_bins.append({"dataset_rel": rel, "arena_rel": "target/" + trel,
                            "size": src.stat().st_size})

    decisions = []
    kept_extras = []
    for rel in (meta.get("files") or {}).get("extras") or []:
        src = cdir / rel
        if not src.is_file():
            decisions.append({"path": rel, "decision": "drop", "reason": "missing", "reasons": []})
            continue
        with open(src, "rb") as fh:
            data = fh.read()
        d = redact.explain(rel, data, meta)
        decisions.append({"path": rel, "decision": d["decision"], "reason": d["reason"],
                          "reasons": d["reasons"]})
        if d["decision"] == "copy":
            erel = _extras_rel(rel)
            dst = staging / "extras" / erel
            dst.parent.mkdir(parents=True, exist_ok=True)
            with open(dst, "wb") as fh:
                fh.write(data)
            os.chmod(dst, 0o644)
            kept_extras.append(erel)

    _write(staging / "bin" / "_shimlog.py", _SHIMLOG_PY, 0o644)
    _write(staging / "bin" / "kuna", _shim_kuna(str(config.kuna_bin())), SHIM_MODE)
    _write(staging / "bin" / "ida-decompile", _shim_ida(str(config.decompiler_cli())), SHIM_MODE)

    (staging / "notes").mkdir(exist_ok=True)
    _write(staging / "notes" / "toolcalls.jsonl", "")

    primary_t = _target_rel(prim_rel)
    others = [c["arena_rel"][len("target/"):] for c in copied_bins[1:]]
    _write(staging / "TASK.md", _task_md(meta, primary_t, others, kept_extras))
    _write(staging / "AGENTS.md", _agents_md(config.kuna_bin()))

    record = {
        "schema": "repipe/arena/1",
        "hexid": hexid,
        "round": round_n,
        "built_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "primary": "target/" + primary_t,
        "binaries": copied_bins,
        "flag_in_path": flag_in_path(meta),
        "extras_kept": ["extras/" + e for e in kept_extras],
        "extras_dropped": [d for d in decisions if d["decision"] == "drop"],
        "tools": {"kuna": str(config.kuna_bin()), "ida": str(config.decompiler_cli())},
    }
    _write(staging / ".repipe" / "build.json", json.dumps(record, indent=2) + "\n")

    if dest.exists():
        shutil.rmtree(dest)
    dest.parent.mkdir(parents=True, exist_ok=True)
    os.replace(staging, dest)
    return dest


# --- sanity check -----------------------------------------------------------

def _walk_files(root: Path):
    for dp, dirs, files in os.walk(root):
        dirs.sort()
        for f in sorted(files):
            yield Path(dp) / f


def flag_in_path(meta) -> bool:
    """True when the challenge's own filenames give the answer away.

    Exactly one challenge does this today: 6442366033c5d43938912a85 ships `bin/Cube.exe` and its
    flag is `Cube`. That spoiler is inherent to the dataset -- renaming the binary would change
    the task -- so the arena cannot fix it, and `sanity_check` must not report it as if the build
    had introduced it. It is recorded in `.repipe/build.json` instead, so grading can discount
    the challenge.
    """
    if not meta:
        return False
    flag = ((meta.get("ground_truth") or {}).get("flag") or "").lower()
    if len(flag) < redact.MIN_FLAG_LEN:
        return False
    paths = [b.get("path") or "" for b in (meta.get("files") or {}).get("binaries") or []]
    paths.append(primary_rel(meta))
    return any(flag in p.lower() for p in paths)


def sanity_check(arena, meta=None) -> list:
    """Every way this arena could be wrong, as strings. Empty list = safe to dispatch.

    `target/` and `.declib/` are exempt from the leak scan on purpose: for many challenges the
    flag *is* in the binary, and the binary is the task. Tester-written paths (`notes/`,
    `report.json`) are scanned only for the answer-independent tags, because a solved run
    legitimately writes the flag there. Everything the build itself produced is fully scanned.
    """
    arena = Path(arena)
    bad = []
    if not arena.is_dir():
        return ["arena does not exist: %s" % arena]

    record = None
    rp = arena / ".repipe" / "build.json"
    if rp.is_file():
        try:
            with open(rp) as fh:
                record = json.load(fh)
        except (OSError, ValueError) as exc:
            bad.append("unreadable .repipe/build.json: %s" % exc)
    if meta is None and record and record.get("hexid"):
        try:
            meta = load_meta(record["hexid"])
        except (OSError, ValueError) as exc:
            bad.append("cannot load meta for %s: %s" % (record["hexid"], exc))

    for rel in ("TASK.md", "AGENTS.md", "bin/kuna", "bin/ida-decompile", "bin/_shimlog.py"):
        if not (arena / rel).is_file():
            bad.append("missing: %s" % rel)
    for rel in ("target", "notes"):
        if not (arena / rel).is_dir():
            bad.append("missing directory: %s" % rel)
    for rel in ("bin/kuna", "bin/ida-decompile"):
        p = arena / rel
        if p.is_file() and not (p.stat().st_mode & stat.S_IXUSR):
            bad.append("shim not executable: %s" % rel)

    target = arena / "target"
    if target.is_dir():
        n = 0
        for p in _walk_files(target):
            n += 1
            mode = stat.S_IMODE(p.stat().st_mode)
            if mode != BIN_MODE:
                bad.append("target file not mode 0755 (is 0%o): %s"
                           % (mode, p.relative_to(arena)))
        if n == 0:
            bad.append("target/ is empty")
    if meta:
        want = "target/" + _target_rel(primary_rel(meta))
        if not (arena / want).is_file():
            bad.append("primary missing from arena: %s" % want)

    for dp, dirs, files in os.walk(arena):
        for d in list(dirs):
            if d in FORBIDDEN_DIRS:
                bad.append("forbidden directory present: %s"
                           % (Path(dp, d).relative_to(arena)))
        for f in files:
            if f in FORBIDDEN_NAMES:
                bad.append("forbidden file present: %s" % (Path(dp, f).relative_to(arena)))

    extras = arena / "extras"
    if extras.is_dir():
        for p in _walk_files(extras):
            with open(p, "rb") as fh:
                data = fh.read(MAX_SCAN_BYTES)
            decision, reason = redact.classify(p, data, meta)
            if decision != "copy":
                bad.append("extras file should have been dropped (%s): %s"
                           % (reason, p.relative_to(arena)))

    inherent = flag_in_path(meta)
    for p in _walk_files(arena):
        rel = p.relative_to(arena)
        if not rel.parts or rel.parts[0] in DERIVED_DIRS:
            continue
        if p.is_symlink() or not p.is_file():
            continue
        if p.stat().st_size > MAX_SCAN_BYTES:
            continue
        with open(p, "rb") as fh:
            hits = redact.scan_for_leak(fh.read(), meta)
        if rel.parts[0] in TESTER_PATHS or inherent:
            hits = [h for h in hits if h != "literal-flag"]
        if hits:
            bad.append("spoiler leak in %s: %s" % (rel, ", ".join(hits)))

    return bad


# --- CLI --------------------------------------------------------------------

def _emit(payload, as_json, lines):
    if as_json:
        print(json.dumps(payload, indent=2))
    else:
        for line in lines:
            print(line)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.workspace",
                                 description="Build and audit tester arenas.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    b = sub.add_parser("build", help="create an arena for one challenge")
    b.add_argument("hexid")
    b.add_argument("--round", type=int, default=0, dest="round_n")
    b.add_argument("--out")
    b.add_argument("--force", action="store_true",
                   help="rebuild even over an arena that already holds tester evidence")
    b.add_argument("--json", action="store_true")

    c = sub.add_parser("check", help="audit an existing arena")
    c.add_argument("arena")
    c.add_argument("--hexid")
    c.add_argument("--json", action="store_true")

    args = ap.parse_args(argv)

    if args.cmd == "build":
        try:
            arena = build(args.hexid, args.round_n, args.out, force=args.force)
        except (FileNotFoundError, ValueError, FileExistsError) as exc:
            print("error: %s" % exc, file=sys.stderr)
            return 1
        meta = load_meta(args.hexid)
        bad = sanity_check(arena, meta)
        with open(arena / ".repipe" / "build.json") as fh:
            record = json.load(fh)
        record["arena"] = str(arena)
        record["violations"] = bad
        lines = ["arena: %s" % arena,
                 "primary: %s" % record["primary"],
                 "binaries: %d  extras kept: %d  extras dropped: %d"
                 % (len(record["binaries"]), len(record["extras_kept"]),
                    len(record["extras_dropped"]))]
        lines += ["  dropped %-16s %s" % (d["reason"], d["path"])
                  for d in record["extras_dropped"]]
        lines += ["violations: %s" % ("none" if not bad else "")] + ["  " + v for v in bad]
        _emit(record, args.json, lines)
        return 3 if bad else 0

    meta = load_meta(args.hexid) if args.hexid else None
    bad = sanity_check(args.arena, meta)
    _emit({"arena": args.arena, "ok": not bad, "violations": bad}, args.json,
          ["ok" if not bad else "VIOLATIONS:"] + ["  " + v for v in bad])
    return 3 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
