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

Files under `decompiler/` and `specs/` are **byte-identical to upstream — never edit
them**. All kuna-specific logic lives in NEW files (top-level `Makefile`, `kuna/`,
`tools/`, `*.md`). This keeps the sync below a clean diff/apply.

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
vendored upstream paths, rewrites the path prefixes per the table above, and applies the
patch at the kuna root with `git apply -p1 --3way --reject`. Conflicts (`.rej` files)
should only ever appear if someone violated the no-edits policy. The script also warns:

- if a `.y`/`.l` changed without its regenerated `.cc` in the same diff (you would then
  need bison/flex locally),
- if an added/changed datatest references a processor whose module is not vendored,
- about added/deleted/renamed files (a deleted `.cc` changes the upstream Makefile's
  `$(wildcard *.cc)` behavior — rebuild and re-run tests after every sync).

### Manual fallback

```bash
git -C ~/github/ghidra diff <GHIDRA_REV>..<newrev> -- \
    Ghidra/Features/Decompiler/src/decompile/cpp \
    Ghidra/Features/Decompiler/src/decompile/unittests \
    Ghidra/Features/Decompiler/src/decompile/datatests > /tmp/up.patch
sed -i 's|\([ab]\)/Ghidra/Features/Decompiler/src/decompile/|\1/decompiler/|g' /tmp/up.patch
git apply -p1 --3way /tmp/up.patch          # from the kuna repo root
# (specs: same idea with Ghidra/Processors/<P>/data/languages → specs/Ghidra/Processors/<P>/data/languages)
# then update GHIDRA_REV in this file.
```
