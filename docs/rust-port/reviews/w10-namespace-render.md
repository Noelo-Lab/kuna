# Namespace-qualified rendering (w10-namespace-render) — ACCEPT, +3

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-namespace-render-v2` @ `35f3a92`,
base 0684760). Verdict: **ACCEPT** (porter self-gated; re-gated at integration onto the 410 tree).

## What landed

The `namespace` datatest maps three globals all named `spam` (global, `a::`, `c::`). Rust dropped
both the namespace *resolution* (the two namespaced globals rendered as raw `dat_601034`/
`dat_601038`) and the *qualifier* (the global `spam` rendered bare instead of `::spam`). Two
faithful roots, no special-casing:

1. **Resolution** — `Database::build_global_query` (`database.rs`) only flattened the global
   scope's own maptable, so symbols in namespace child scopes were invisible to the frozen
   global-scope snapshot. Now it descends every non-functional scope rooted at global (mirroring
   C++ `getGlobalScope()->queryContainer` via `mapScope`/`resolvemap`), and `IfcMapaddress`
   (`ifacedecomp.rs`) calls `addRange` on namespace scopes per `ifacedecomp.cc:573-576`. Each
   snapshot entry carries its scope display-name chain (`GlobalEntry::scope_path` in `seams.rs`).
2. **Qualification** — `name_local_highs_angr` (`coreaction_cleanup.rs`) qualifies the bound
   global name per `PrintC::pushSymbolScope` + `Symbol::getResolutionDepth` (printc.cc:203,
   database.cc:324) under default MINIMAL_NAMESPACES: a parameter/local shadowing the global
   drives the `::`/`ns::` prefix via the new `ScopeLocal::local_name_used` (`varmap.rs`).

## Effect

408 → 411 on its base; re-confirmed +3 on the integrated 410 tree → **413**. Gained: Namespace
#2 (`::spam`), #3 (`a::spam`), #4 (`c::spam`).

## Gate (re-run at integration onto 410)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 413]`; full per-assertion passing-set diff vs base 410: **regressed-set
  EMPTY** (gained exactly Namespace #2/#3/#4).
- switch cluster: switchind 16/16, switchmulti 8/9 — unchanged.
- `boolless`/`condconst` byte-pass; oracle **PARITY OK**; C++ tree byte-untouched.

## Note

`kuna.run_tests` resolves binaries from the editable-install root (the main worktree), so the
worktree gate must be run with `KUNA_ROOT=<worktree>`; at integration the merge is in the main
tree so the harness exercises the correct binaries directly.
