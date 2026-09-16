# F4 elfmain — state

- slug: elfmain / branch: feat/elfmain / worktree /home/mahaloz/kwt/elfmain
- PR: (not opened yet)

## Done
- read COMMON_BRIEF + plan Stage 1/orchestration
- specs symlinked (148 .sla), SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs
- baseline `make binaries` green
- baseline witness captured: `kuna decompile-all <decbench O2 fmt> --addr 0x26a0`
  -> `unsigned long sub_26a0(int a0,char **a1)`
- surveyed machomain/armlibcmain, entry oracle 4, commit seam, all count sites

## Next
1. write kuna-analysis/src/analyzers/entry/kuna_elfmain.rs
2. register pass (passes.rs), gate (engine.rs, architecture.rs, kuna_console.rs,
   options.rs, phases.toml, PASS_GATES tests.rs)
3. verify_elfmain.rs + stages XML + tests/cli probes audit
4. counters --rederive/--fix, options.md, baseline-stages re-record
5. spec 01-program-prep.md, docs/features/elfmain/
6. gates
