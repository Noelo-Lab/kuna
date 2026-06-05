# kuna Progress Log

## Session goals (2026-06-05)

- [x] Find the Ghidra decompiler testcases (deep-decompiler only) and create a way to run them
- [x] Identify the C/C++ decompiler source in Ghidra and how to extract it
- [x] Extract it into kuna and wrap it in a simple Python interface (binary + function → C)
- [x] Port the testcases to the stripped-down decompiler
- [x] Record a procedure for porting future upstream Ghidra decompiler changes
- [x] Map decompiler source files to STAGES.md (document mismatches/new stages)

## Results

- **Tests:** `decompiler/unittests/` (204 C++ unit tests) + `decompiler/datatests/` (83 XML
  regression tests → 675 assertions). Java GUI tests excluded. Harness = upstream
  `decomp_test_dbg`, wrapped by `kuna.run_tests` (parses unit/stderr + data/stdout, parity diff).
- **Parity:** kuna build reproduces the pristine-ghidra baseline EXACTLY — 204/204 unit,
  675/675 datatest assertions, identical pass set. `docs/baseline.json` is the recorded oracle;
  `python -m kuna.run_tests --baseline docs/baseline.json` → **PARITY OK**.
- **Wrapper:** `python -m kuna.decompile <bin> <func|0xaddr> [--addr]` drives `decomp_dbg`
  (`load file`/`read symbols`/`load function|addr`/`decompile`/`print C`); C captured via
  `openfile write` so prompts never leak in. Verified on a compiled binary by name and address.
- **Upstream sync:** `tools/sync_upstream.py` (diff vendored paths → rewrite prefixes → git apply
  → bump GHIDRA_REV). Validated clean forward-apply + non-mutating dry-run on two historical revs.
- **Stage mapping:** `STAGE_MAPPING.md` places all 115 .cc exactly once across the 19 stages +
  infrastructure, anchored to `universalAction`. Documented gaps: stages 06 Dataflow, 10 Memory,
  12 Aggregate Types, 15 Interproc have no dedicated module (emergent / folded into others).

## Build note (libbfd without root)

`decomp_dbg`/`decomp_opt`/`decomp_test_dbg` link libbfd. The proper prerequisite is
`sudo apt-get install -y binutils-dev`. This session had no sudo, so libbfd was fetched into
`./.bfdlocal` via `tools/fetch_bfd.sh` and the build run as `make BFD_PREFIX=$(pwd)/.bfdlocal`
(the Makefile injects it through the upstream `ADDITIONAL_FLAGS`/`BFDLIB` without editing
upstream files). `sleigh_opt` and all 148 `.sla` build without libbfd.

## Log

### 2026-06-05 — Session start, exploration & plan
- Located decompiler source: `ghidra/Ghidra/Features/Decompiler/src/decompile/`
  (cpp: 115 .cc / 114 .hh; upstream Makefile builds `decomp_dbg`, `decomp_test_dbg`,
  `sleigh_dbg`, `ghidra_dbg`, libs). Pure C++11; deps: libbfd (console/test binaries
  only), libz; bison/flex outputs are committed upstream.
- Located the deep-decompiler-only tests: `unittests/` (7 files, 204 TEST()s, pure C++)
  and `datatests/` (83 XML regression tests with embedded binaries + regex assertions).
  Java-side tests (`src/test.slow/java/...`) are GUI/integration — explicitly out of scope.
  Harness: `decomp_test_dbg [-sleighpath D] [-path DIR] [unittests|datatests [names]]`,
  exit code = #failures.
- Key runtime fact: no compiled `.sla` exist in the repo — specs must be compiled with
  the `sleigh` binary (`sleigh_opt -a <dir>`).
- Plan approved: vendor decompile/{cpp,unittests,datatests} + ALL processor spec modules,
  zero-modification policy, top-level Makefile driver, Python package (`kuna`),
  `tools/sync_upstream.py`, STAGE_MAPPING.md. Upstream rev: cef869af04c474...1644.
- Scaffolding written (.gitignore, README.md, UPSTREAM.md, this file).
- Baseline build of the pristine tree started in /tmp/kuna-baseline (parity oracle).
