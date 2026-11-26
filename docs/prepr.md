# Before opening a PR: `aritydiff` and `difflint`

Two optional helpers for what review most often sends back: call arguments lost
on real binaries, and diff hygiene. Neither is a gate and CI does not run them;
the four gates in `docs/agents.md` are unchanged. They are worth running before
opening a PR that changes call recovery. What goes in the PR body is still
decided by `docs/pr-bodies.md`: a line or two of what they showed, not their output.

## `difflint`: the diff itself

```
python -m scripts.prepr.difflint            # against origin/main
python -m scripts.prepr.difflint upstream/main
```

Seconds. Errors fail the run:

- a binary added under `tests/fixtures/` that no README, note or source beside
  it (or in a directory above it, up to `fixtures/`) names: that is where its
  recipe and SHA-256 are recorded;
- a local path (`/home/<user>/`, `C:\Users\`, `.claude/jobs/`) inside an added
  binary or an added test line;
- an added `.obj`/`.o`/`.lib` that no test or script names;
- a `<stringmatch>` changed in a stage test that already exists on the base,
  unless a commit carries `Assertion-change: <test>: <why the new expectation is
  right>` (`kuna-catalog.xml`'s registry counts are exempt).

A new `kuna_*.rs` with no `phases.d/` row prints a warning: that is right
only for a strict correctness fix (`docs/agents.md`, "Adding features").

## `aritydiff`: call arities over real binaries

```
python -m scripts.prepr.aritydiff BASE_KUNA HEAD_KUNA --corpus msvc32 [BINARY ...] [-v]
python -m scripts.prepr.aritydiff BASE_KUNA HEAD_KUNA --corpus msvc32 --option NAME on
```

Both builds run `decompile-all` over every binary; each call whose argument
count changed is reported per binary, with `-v` listing them. Build `BASE_KUNA`
from the merge base (`git worktree add`, a separate `CARGO_TARGET_DIR`).
Each build reads the specs it finds itself; export `KUNA_SPECS` to give both
the same tree. `--option` applies to the head only, so a default-off option can
be measured.

The run fails when a call loses an argument, a function stops decompiling, or
a binary prints fewer calls. A failure is not a verdict: classify every lost
argument against the disassembly (a stray slot that should go, or a real
argument that must not), and do the same for gained ones, which pass the gate
but are as often a stray slot as a recovered argument. Summarize the
classification in the PR body.

`--corpus msvc32` is five public 32-bit MSVC executables, downloaded on first
use and pinned by SHA-256 (`scripts/prepr/corpus.py`): setuptools 59.6.0
`cli-32.exe`/`gui-32.exe`, pip 21.3.1's distlib `t32.exe`/`w32.exe`, and
Ghidra's `WinHelloCPP.exe`. They carry the MSVC CRT shapes the repo's fixtures
lack: `/GS` checks, SEH prologs, `_chkstk`. About 30 seconds per pair of
builds. Nothing is committed; the cache is `$KUNA_PREPR_CACHE` (default
`~/.cache/kuna-prepr`). Pass other binaries as arguments: decbench's i386
PEs are MinGW only (`docs/decbench-loop.md`), so they complement the corpus
rather than replace it, and its ELF slices are the off-target check.

Binaries that cannot be shared (a game you are reconstructing, a customer
image) go in `--corpus local`: one path per line in `$KUNA_PREPR_LOCAL`
(default `~/.config/kuna-prepr/local.txt`). They stay on your machine; quote at
most short excerpts of their output in a PR.

What it would have shown on recent x86-32 changes, base to head:

| Change | Functions changed | Arguments lost (to none) | Gained |
|---|---|---|---|
| callee purge as first submitted | 99 | 85 (33) | 6 |
| the same after review | 0 | 0 | 0 |
| ECX `__thiscall` inference, always on | 179 | 49 (19) | 305 |
| the same after review (option off) | 0 | 0 | 0 |
| callee stack-argument extent as first submitted | 215 | 257 (164) | 57 |
| the same after review | 20 | 14 (5), each classified | 11 |
