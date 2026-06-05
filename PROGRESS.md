# kuna Progress Log

## Session goals (2026-06-05)

- [ ] Find the Ghidra decompiler testcases (deep-decompiler only) and create a way to run them
- [ ] Identify the C/C++ decompiler source in Ghidra and how to extract it
- [ ] Extract it into kuna and wrap it in a simple Python interface (binary + function → C)
- [ ] Port the testcases to the stripped-down decompiler
- [ ] Record a procedure for porting future upstream Ghidra decompiler changes
- [ ] Map decompiler source files to STAGES.md (document mismatches/new stages)

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
