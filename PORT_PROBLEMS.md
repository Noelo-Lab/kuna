# Port Problems

Findings from a full audit of the Ghidra → kuna port (2026-06-05). Every finding below
was independently reproduced and survived an adversarial verification pass.

> **Status: ALL 18 PROBLEMS FIXED (2026-06-05).** See the *Resolution* table at the
> bottom for what was done about each one. The problem descriptions below are kept
> as originally written (line numbers refer to the pre-fix tree).

**Headline: the vendored translation itself is clean.** Every problem found lives in
kuna-owned glue (Makefile wrapper, Python package, sync tooling, docs). None affects the
correctness of the current build, test parity, or decompile output.

## Audit method & what is verified clean

| Check | Result |
|---|---|
| Blob-hash sweep of all vendored files vs upstream @ `GHIDRA_REV` (329 decompiler + 710 specs) | byte-identical; 0 missing, 0 extra, 0 differing |
| File modes / symlinks vs upstream | identical |
| Documented exclusions (`zlib/`, `build.gradle`, `.cproject`) | exactly match reality |
| Fresh `make clean` → full rebuild → `make specs` | 148/148 slaspecs compile |
| `kuna.run_tests --all --baseline docs/baseline.json` | 204/204 unit, 675/675 assertions, **PARITY OK**, exit 0 |
| `make test` agreement, exit-code semantics vs `test.cc` | confirmed |
| `decompile.py` console commands vs `consolemain.cc`/`ifacedecomp.cc`/`interface.cc` registrations (incl. `load file [target] <path>` arg order, `openfile write` → `fileoptr` redirect, error strings) | all exact |
| `run_tests.py` parsing regexes vs `test.cc`/`testfunction.cc` output formats and streams (unit→stderr, data→stdout) | all exact |
| E2E decompile: by name, helper fn, stripped-binary by address; 4 error paths (bad fn/file/addr/non-binary) | clean C output, clear errors, correct exit codes |
| `STAGE_MAPPING.md`: 115/115 modules mapped exactly once; every cited `coreaction.cc`/`blockaction.cc` line anchor | all verified accurate |
| Doc counts (115 `.cc` / 114 `.hh` / 204 tests / 83 datatests / 675 assertions / 19 stages) | all correct |
| `.gitignore` coverage of build artifacts | complete (one gap: see P6) |
| `sleigh -a` error propagation into `make specs` | stops at first error, nonzero exit |
| `.bfdlocal` layout vs Makefile `BFD_PREFIX` expectations | matches on this host (see P10) |

---

## Problems — `tools/sync_upstream.py`

These do not affect the current tree; they are risks for **future** syncs.

### P1 (major) — `--dry-run` mutates UPSTREAM.md on the no-vendored-changes path
`tools/sync_upstream.py:240-245`. When `from != to` but the diff restricted to vendored
paths is empty, the script calls `write_ghidra_rev()` and returns **before** the
`args.dry_run` check is ever reached. A "preview" run rewrites `GHIDRA_REV`.
Reproduced: with a fake root, `--dry-run` printed `Updated GHIDRA_REV -> …` and rewrote
the file. Contradicts the `--dry-run` help text ("report and `git apply --check` only")
and PROGRESS.md's "non-mutating dry-run" claim (which was validated only on the
non-empty-diff path).
**Fix:** gate the `write_ghidra_rev()` call in the empty-diff branch on `not args.dry_run`.

### P2 (major) — vendored→non-vendored rename silently plants a stray upstream-path file
`tools/sync_upstream.py:126-129,139,151`. `rewrite_patch()` drops a file section only
when **both** sides are unvendored. For an upstream rename out of a vendored dir
(e.g. `decompiler/cpp/x.cc` → `…/data/manuals/x.cc`), the kept section maps the a-side
but leaves the b-side at the raw upstream path. The patch **passes `git apply --check`**,
then on apply deletes the vendored file and creates e.g.
`kuna/Ghidra/Processors/x86/data/manuals/x.cc` — a foreign file inside the kuna root —
with only a `~ renamed:` notice. Reproduced end-to-end against a synthetic repo.
**Fix:** split boundary-crossing renames into a pure delete (vendored source) or pure
add (vendored target), dropping the unvendored side.

### P3 (minor) — non-vendored→vendored rename aborts an otherwise-valid sync
The mirror of P2: `rename from` keeps a raw upstream path that doesn't exist in kuna, so
`git apply --check` fails (`No such file or directory`) and the whole sync returns 1,
blocking a change that should simply be an add of the vendored target. Reproduced.
**Fix:** same as P2 (emit a pure add).

### P4 (minor) — quoted diff paths bypass rewriting entirely
`tools/sync_upstream.py:91,121-123`. Paths git must quote (non-ASCII, tab, `"` …) are
emitted as `diff --git "a/…" "b/…"`; `_HDR_PATTERNS[0]` doesn't match the quoted form, so
the **entire section passes through unmodified**, checks clean, and applies the file at
its raw upstream path inside the kuna root; `analyze()` records `added: [None]`.
All currently-vendored paths are ASCII, so this is latent. Reproduced with a synthetic
`my file ünicode.cc`.
**Fix:** handle git's quoted-path header form (or set `core.quotepath=false` semantics by
parsing the quoted form), or hard-fail when an unparsable `diff --git` header is seen.

### P5 (minor) — `write_ghidra_rev()` silently no-ops if the `GHIDRA_REV=` line is missing
`tools/sync_upstream.py:61-66`. `re.sub(count=1)` with no check that a substitution
happened; callers print `Updated GHIDRA_REV -> …` regardless. A reworded UPSTREAM.md
would make syncs silently stop recording the base rev.
**Fix:** verify the substitution occurred (compare text, or `re.subn`) and error if not.

### P6 (minor) — `.kuna_sync.patch` is left in the repo root and is not gitignored
`tools/sync_upstream.py:269-282`. Every non-empty `--dry-run` writes
`<root>/.kuna_sync.patch` and leaves it ("patch left at … (not applied)"); it is not in
`.gitignore` (`git check-ignore` exits 1), so it pollutes `git status`.
**Fix:** add `.kuna_sync.patch` to `.gitignore` (and/or write it to a tempdir for dry runs).

### P7 (info) — `analyze()` reports on the raw, pre-filter diff
`tools/sync_upstream.py:248`. Added/deleted/renamed counts and the
`! deleted: … (changes upstream Makefile $(wildcard *.cc); rebuild after)` warning are
computed over the whole `UPSTREAM_PREFIXES` diff, including non-vendored files that
`rewrite_patch()` correctly skips — misleading maintainer output, no corruption.
**Fix:** filter `analyze()` through `map_path()` (or run it on the rewritten patch).

## Problems — top-level `Makefile`

### P8 (minor) — `make -j all` races two concurrent builds of `sleigh_opt`
`Makefile:40,64-78`. `all: binaries specs` has no ordering between its prerequisites and
no `.NOTPARALLEL`. `binaries` builds `sleigh_opt` inside its recipe; `specs` reaches it
via the `$(SLEIGH):` file rule → `$(MAKE) sleigh`. Under top-level `-j`, two sub-makes
compile the same `sla_opt/*.o` and link the same binary concurrently (and both run
`touch-generated`). Serial `make all` (the documented invocation) is safe.
**Fix:** order-only prerequisite (`specs: | binaries`) or `.NOTPARALLEL:`.

### P9 (minor) — hardcoded `-j$(NJOBS)` in sub-makes overrides a parent jobserver
`Makefile:65,68-71`. Each recursive call forces `-j$(NJOBS)`, so `make -jN` at the top
prints `warning: -jN forced in submake: resetting jobserver mode` and ignores the
parent's job budget (over-subscription; compounds P8).
**Fix:** don't pass `-j` in recipes; let callers' jobserver flow through (or pass it only
when `MAKEFLAGS` has no jobserver).

### P10 (minor) — `BFD_TRIPLET` from `gcc -dumpmachine` may not match the dpkg multiarch dir
`Makefile:30-32` + `tools/fetch_bfd.sh:36-44`. `.bfdlocal` is populated from Debian
`.deb`s, whose libs live under `DEB_HOST_MULTIARCH`; `gcc -dumpmachine` can differ
(e.g. `x86_64-pc-linux-gnu` toolchains, `i686-` vs `i386-`). Neither `check-deps` nor
`fetch_bfd.sh` validates the **lib** path (both only test `bfd.h`, which is
triplet-independent) — a mismatch surfaces only as a confusing `-lbfd` link failure,
and `fetch_bfd.sh`'s success banner prints the wrong `.so` path.
**Fix:** derive the libdir from `dpkg-architecture -qDEB_HOST_MULTIARCH` (fallback to
dumpmachine) and have both guards test for `libbfd.so`/`libbfd.a` in it.

### P11 (minor) — missing `nproc` turns `-j$(NJOBS)` into unbounded `-j`
`Makefile:22`. `NJOBS ?= $(shell nproc)` becomes empty if `nproc` is absent, so recipes
expand to bare `-j` = unlimited jobs (fork-bomb/OOM risk on a 150-file tree).
**Fix:** `NJOBS ?= $(shell nproc 2>/dev/null || echo 1)`.

### P12 (minor) — `make test` does not depend on the test binary or specs
`Makefile:83-84` (upstream's own `test:` depends on `decomp_test_dbg`,
`decompiler/cpp/Makefile:260`). On a fresh clone, `make test` fails with
`./decomp_test_dbg: not found` (exit 127) instead of building, and nothing ensures
`.sla` files exist for the datatests.
**Fix:** `test: binaries specs` (order-only is fine), or document the constraint.

## Problems — `kuna/` Python package

### P13 (minor) — `_DATA_ERR`'s `No output for …` branch is dead
`kuna/run_tests.py:35-39` vs `decompiler/cpp/testfunction.cc:332-333,391`. The harness
only ever prints `No output for <file>` **indented two spaces** inside its 10-capped
failures summary; the regex is anchored at column 0 and can never match. Worse, upstream
`runTests` early-returns on the no-output path before counting, so applied=0 and the
process exit code stays 0: a no-output datatest regression is invisible without
`--baseline` (and with `--baseline` shows up only as disappeared keys — which the parity
check does catch). Reproduced against the real harness.
**Fix:** match the indented form (and/or treat baseline-disappearance as the primary
detector, which already works).

### P14 (minor) — signal-killed harness yields a misleading exit code without `--baseline`
`kuna/run_tests.py:256`. `min(returncode, 255)` doesn't clamp negative (signal) codes:
SIGSEGV → `sys.exit(-11)` → shell status 245. Nonzero (so not a false success), but not
the documented "failed-test count". The `--baseline` path correctly returns 1.
**Fix:** `return 1 if rc < 0 else min(rc, 255)` (or `max(1, min(rc, 255))` for nonzero rc).

### P15 (minor) — baseline cannot name a regression in a duplicate-named upstream assertion
`kuna/run_tests.py:142-146` + `docs/baseline.json` + upstream
`decompiler/datatests/partialsplit.xml:85-86` (two assertions both named
`Partial splitting #9` — an upstream quirk). The set-based key space collapses them:
baseline holds 674 `data:` keys for 675 passing assertions. If exactly one of the pair
regressed, `regressed` stays empty and the failure is caught **only** by the nonzero
harness exit code — it cannot be named in the parity report. No false PARITY OK is
possible.
**Fix (optional):** disambiguate duplicate names when building keys (e.g. suffix `#2`),
or document the limitation.

### P16 (info) — library `run()` silently ignores `names` in `mode='all'`
`kuna/run_tests.py:114-125,190-191`. The CLI guards `--name` against `--all`, but the
importable `run('all', names=[…])` drops the names and runs everything. CLI unaffected.

### P17 (info) — `decomp_dbg -s <specs>` is inert; `SLEIGHHOME` is the load-bearing mechanism
`kuna/decompile.py:107,111-113` vs `consolemain.cc:194-206`, `sleigh_arch.cc`,
`libdecomp.cc`. `SLEIGHHOME` is scanned recursively for `…/data/languages` dirs, while
`-s` paths are added literally (non-recursively) and so find nothing under kuna's
`specs/` root. decompile.py works correctly **because it also sets `SLEIGHHOME`** (and
safely overrides a hostile pre-existing value); the `-s` argument is redundant and the
docstring framing around it is misleading. Behavior verified both ways empirically.

## Problems — docs

### P18 (minor) — UPSTREAM.md describes an apply strategy the code never had
`UPSTREAM.md:63-64` says the patch is applied with `git apply -p1 --3way --reject` and
discusses `.rej` files. The code (`tools/sync_upstream.py:273-296`) runs `--check`, then
plain `git apply -p1`, then `--3way` as fallback; `--reject` appears nowhere, so `.rej`
files can never be produced. (Forensics: the code never used `--reject` in any revision;
the doc was stale from inception, and commit `d79d0c5`'s message describes a code change
whose diff is actually only a file-mode change.)
**Fix:** rewrite the UPSTREAM.md paragraph to match the real strategy.

---

## Severity roll-up

| Severity | Count | IDs |
|---|---|---|
| major | 2 | P1, P2 |
| minor | 13 | P3–P6, P8–P15, P18 |
| info | 3 | P7, P16, P17 |

None are regressions in decompiler behavior; parity with pristine upstream holds exactly.
The two majors are both future-sync safety issues in `tools/sync_upstream.py`.

---

## Resolution (2026-06-05)

All 18 fixed; each fix verified by re-running the original reproduction. Vendored files
untouched; PARITY OK re-confirmed after the changes (baseline regenerated for P15).

| ID | Fix |
|---|---|
| P1 | Empty-diff branch now checks `args.dry_run` before `write_ghidra_rev()`; dry run prints what *would* happen. |
| P2, P3 | The sync diff is now taken with `--no-renames`, so renames arrive as plain delete+add and can never straddle the vendored boundary; `rewrite_patch()` additionally hard-fails (`PatchRewriteError`) on any rename/copy/similarity marker and on `diff --git` headers naming two different paths. Header rewriting and marker scanning stop at the first `@@` hunk, so hunk content — even lines byte-identical to a `--- a/…` header — passes through verbatim. Verified end-to-end on synthetic repos: rename-out → clean delete, rename-in → clean add, no stray files. |
| P4 | Diff taken with `core.quotepath=false` (non-ASCII paths arrive unquoted and rewrite correctly — verified with a `ünicode.cc`); any header git still quotes (quote/control chars) is a hard error instead of a silent pass-through. Unparseable preamble is also a hard error. |
| P5 | `write_ghidra_rev()` uses `re.subn` and raises if no substitution happened; both call sites surface the error (the post-apply site notes the patch *was* applied). |
| P6 | `.kuna_sync.patch` added to `.gitignore`. |
| P7 | `analyze()` now filters through `map_path()` and reports kuna paths — only files that will actually be applied are counted/warned about. Also fixed adjacent gap: a diff touching only non-vendored files now reports "no changes to vendored paths" instead of failing `git apply --check` on an empty patch. |
| P8 | `.NOTPARALLEL:` — the wrapper's targets are orchestration that must run serially (parallelism lives inside each sub-make). |
| P9 | Sub-makes use `$(SUBJOBS)`, which expands to `-j$(NJOBS)` only when no parent jobserver exists in `MAKEFLAGS` (recipe-time expansion); under `make -jN` the inner builds share the parent's budget — no more "forced in submake" warnings. Serial builds keep the parallel inner default; `NJOBS=1 make` forces fully serial. |
| P10 | `BFD_LIBDIR` is located by `$(wildcard …/usr/lib/*/libbfd.so)` instead of guessing a gcc triplet; `check-deps` and `fetch_bfd.sh` both verify `libbfd.so` exists (not just `bfd.h`) and print the real path. |
| P11 | `NJOBS ?= $(shell nproc 2>/dev/null || echo 1)`. |
| P12 | `make test` builds `binaries`/`specs` first **only if missing** (binary not executable / no `.sla` found), keeping the fast path fast while fixing the fresh-clone footgun. |
| P13 | `_DATA_ERR` accepts the indented summary forms and the previously-unparsed `Execution failed for …`; errors are deduped (col-0 + summary) and must name a `*.xml` file, so a failing *test name* that mimics an error string is never miscounted. `main()` forces exit 1 when file-level data errors (or any parsed failure) are present despite harness exit 0. Verified with fake harnesses both ways. |
| P14 | Negative (signal) return codes map to shell-style `128+signal` (SIGSEGV → 139). |
| P15 | `_pass_keyset()` disambiguates repeated test names with a deterministic ` @dupN` suffix; `docs/baseline.json` regenerated — now 879 keys (204 unit + 675 data, matching the true assertion count), PARITY OK re-verified. |
| P16 | `run()` raises `ValueError` for `names` with `mode='all'` (CLI guard retained). |
| P17 | Accurate comment at the invocation site: `SLEIGHHOME` (recursively scanned) is the load-bearing mechanism; `-s` is kept for direct-languages-dir usage. |
| P18 | UPSTREAM.md rewritten to describe the real strategy (`--check` → plain apply → `--3way` fallback, no `.rej` ever) plus the new `--no-renames`/hard-fail behavior; manual-fallback recipe updated; CLAUDE.md sync paragraph aligned. |
