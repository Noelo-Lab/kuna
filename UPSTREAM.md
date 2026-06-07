# Upstream provenance and sync

kuna vendors Ghidra's C++ decompiler and its SLEIGH processor specifications.

## Source

- Repo:   https://github.com/NationalSecurityAgency/ghidra.git
- Commit: GHIDRA_REV=cef869af04c4740a71ad31a55704045b1b0d1644
  (2026-06-01, "Merge ... GP-6874_ryanmkurtz_security--SQUASHED")

## Vendored path map (upstream → kuna)

| Upstream (in the ghidra repo) | kuna |
|---|---|
| `Ghidra/Features/Decompiler/src/decompile/cpp/` | `decompiler/cpp/` |
| `Ghidra/Features/Decompiler/src/decompile/unittests/` | `decompiler/unittests/` |
| `Ghidra/Features/Decompiler/src/decompile/datatests/` | `decompiler/datatests/` |
| `Ghidra/Processors/<P>/data/languages/` | `specs/Ghidra/Processors/<P>/data/languages/` |

Intentionally **excluded**: `src/decompile/zlib/` (the default Makefile build uses the
system `<zlib.h>` + `-lz`; the bundled copy is only used when `LOCAL_ZLIB` is defined,
which kuna never does), and `src/decompile/build.gradle` (Gradle/Eclipse glue).

## Local-modifications policy

**Until 2026-06-06** files under `decompiler/` and `specs/` were byte-identical to
upstream. As of the GH-558 stage-model prototype, kuna **intentionally diverges**
(authorized decision: kuna is now its own decompiler, not a tracking mirror). New
kuna logic still prefers new files (`kuna_*.cc/.hh`, auto-linked by the upstream
Makefile's `$(wildcard *.cc)`), with minimal anchor edits in vendored files.

### Divergence (vendored files modified)

| File | What |
|---|---|
| `decompiler/cpp/op.hh` | new `addlflags` bit `canonical_lessequal` (0x1000) + accessors |
| `decompiler/cpp/funcdata_op.cc` | `replaceLessequal` records provenance on the rewritten op |
| `decompiler/cpp/coreaction.cc` | `RuleIntLessEqual` regrouped `analysis`→`canonicalcompare`; `ActionPresentCompareForm` registered after the last branch-flip pass; group lists updated |
| `decompiler/cpp/architecture.hh/.cc` | `present_lessequal` flag (default false) |
| `decompiler/cpp/options.cc` | registers kuna options (`compareform`, `arraynotation`) |
| `decompiler/cpp/printc.hh/.cc` | `option_arraynotation` (+ getter) + `&base[index]` mode in `opPtradd` |
| `decompiler/cpp/heritage.cc` | include + 2 recorder calls in `bumpDeadcodeDelay` (restart observability) |
| `decompiler/cpp/jumptable.cc` | include + 1 recorder call at the multistage-jump restart site |
| `decompiler/cpp/fspec.cc` | include + 2 recorder calls (`deindirect`/`forceSet` late-prototype restarts) |
| `decompiler/cpp/printlanguage.cc` | GH-2786 default-flip: parenthesize adjacent identical sign tokens (`docs/divergences.md` DIV-1) |
| `decompiler/cpp/ruleaction.cc` | GH-8471: include + 1-line guard in `RulePtrsubUndo::applyOp` (preserve Thumb funcptr) |
| `decompiler/cpp/coreaction.cc` | GH-6930: include + 1-line guard in `ActionConstantPtr::isPointer` (function-entry inference) |
| `specs/.../V850/data/languages/V850.cspec` | register-name case fix `ctbp`→`CTBP` (matches SLEIGH `CTBP`; the case-sensitive C++ register lookup otherwise aborts V850 at arch init) |

kuna-owned additions in the vendored directory: `kuna_compareform.{hh,cc}`,
`kuna_arraynotation.{hh,cc}`, `kuna_stages.{hh,cc}` (stage registry),
`kuna_console.{hh,cc}` (self-registering `IfaceKunaCapability` — console commands
added with zero upstream edits), `kuna_assert.{hh,cc}` (typed assertion API),
`kuna_restartlog.{hh,cc}` (restart observability side table),
`kuna_thumbfuncptr.{hh,cc}` (GH-8471), `kuna_inferfuncentry.{hh,cc}` (GH-6930)
(new files, not upstream edits).

`sync_upstream.py`'s clean-apply guarantee **no longer holds** for the files in the
table: a future sync touching them will need `--3way` or manual conflict resolution
(added kuna `kuna_*` files and new-file additions are unaffected). The sync remains
clean for `specs/`, `decompiler/unittests/`, `decompiler/datatests/`, and the ~110
unmodified `.cc` files.

Notes inherited from upstream:
- The bison/flex outputs (`grammar.cc`, `xml.cc`, `pcodeparse.cc`, `slghparse.cc/.hh`,
  `slghscan.cc`) are committed upstream and vendored here. The top-level Makefile's
  `touch-generated` target keeps their mtimes newer than the `.y`/`.l` sources so
  `make` never tries to regenerate them (bison/flex then aren't needed).
- `ruleparse.cc` is intentionally absent upstream (CPUI_RULECOMPILE is off by default).

## Build prerequisites

- g++ (C++11) and GNU make
- `binutils-dev` (libbfd) — required to link `decomp_dbg` / `decomp_opt` / `decomp_test_dbg`
- `zlib1g-dev` (libz)
- `bison` / `flex` — ONLY if a `.y`/`.l` grammar file is edited

## Porting upstream changes

```bash
# 1. Fetch the new upstream state in your ghidra checkout
git -C ~/github/ghidra fetch origin

# 2. Dry-run to see what would change
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master --dry-run

# 3. Apply, rebuild, re-test
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master
make test
python -m kuna.run_tests --baseline docs/baseline.json

# 4. Review `git diff`, then commit. The script updates GHIDRA_REV above on success.
```

The script diffs the ghidra repo between `GHIDRA_REV` and `--to` restricted to the
vendored upstream paths. The diff is taken with `--no-renames` (renames arrive as plain
delete+add pairs, so they can never straddle the vendored boundary) and
`core.quotepath=false`. Path prefixes are rewritten per the table above, then the patch
is applied at the kuna root: `git apply -p1 --check` first, then a plain
`git apply -p1`; if the plain apply fails (only possible if someone violated the
no-edits policy), it retries with `--3way`, which works because unmodified vendored
files have the same blob hashes as upstream. No `.rej` files are ever produced. The
rewritten patch is written to `.kuna_sync.patch` (gitignored) and removed after a
successful apply; `--dry-run` stops after the `--check` and leaves it for inspection.
`GHIDRA_REV` above is updated only on a successful apply (or an empty vendored diff)
and never during a dry run.

The script hard-fails rather than guessing on anything unusual in the patch:
quoted/unparseable diff headers, rename/copy lines, or binary file changes — handle
those manually. It also warns:

- if a `.y`/`.l` changed without its regenerated `.cc` in the same diff (you would then
  need bison/flex locally),
- if an added datatest references a processor whose module is not vendored,
- about added/deleted vendored files (a deleted `.cc` changes the upstream Makefile's
  `$(wildcard *.cc)` behavior — rebuild and re-run tests after every sync).

### Manual fallback

```bash
git -C ~/github/ghidra -c core.quotepath=false diff --no-renames <GHIDRA_REV>..<newrev> -- \
    Ghidra/Features/Decompiler/src/decompile/cpp \
    Ghidra/Features/Decompiler/src/decompile/unittests \
    Ghidra/Features/Decompiler/src/decompile/datatests > /tmp/up.patch
sed -i 's|\([ab]\)/Ghidra/Features/Decompiler/src/decompile/|\1/decompiler/|g' /tmp/up.patch
git apply -p1 --3way /tmp/up.patch          # from the kuna repo root
# (specs: same idea with Ghidra/Processors/<P>/data/languages → specs/Ghidra/Processors/<P>/data/languages)
# then update GHIDRA_REV in this file.
```
