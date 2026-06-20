# Rust port plan

Condensed from the approved port plan. The backlog that implements this plan is
`docs/rust-port/checklist.json` (validated by `tools/rust-port/check_checklist.py`);
behavioral concessions are ledgered in `losses.md`; review protocol in
`verification.md`; live progress in `status.md`. Architecture decisions land in
`docs/rust-port/adr/` (owned by a separate work stream).

## Strategy

- **Parallel Rust workspace under `rust/`; the C++ tree is the oracle and stays
  green throughout.** Nothing in `decompiler/`, `specs/`, or the Makefile changes
  for the port's sake. Oracle status at plan time: 207/207 unit tests, 675/675
  datatest assertions, PARITY OK.
- **Console-surface compatibility.** The Rust binaries speak the same console
  command surface as `decomp_dbg`/`decomp_test_dbg` (same commands, same
  stdout/stderr stream split), so all 83 XML datatests and the Python harness
  (`kuna/run_tests.py`, `kuna/decompile.py`, `kuna/catalog.py`) verify the port
  **unchanged** via an `--engine {cpp,rust}` switch.
- **Faithful-port-first bodies, redesigned shell.** Function bodies are
  line-faithful transliterations: same algorithms, same iteration order
  (`BTreeMap`/`BTreeSet`, never `HashMap`; `sort` never `sort_unstable`), explicit
  integer widths and casts. The *shell* — crate/module layout, ownership model,
  `Result` instead of exceptions — is redesigned idiomatically. Every deliberate
  departure is a numbered entry in `losses.md`.
- **Porter/verifier separation.** Every port item has a paired verify item
  executed by a different agent under `verification.md`; a wave gate requires all
  its verifies ACCEPTed.
- **SLEIGH compiler deferred** (LOSS-001). The C++ `sleigh_opt` remains the
  `.slaspec -> .sla` builder; the Rust side only *reads* `.sla` (the compressed
  slaformat). Likewise the `ghidra_*` IPC client is unported (LOSS-002): it is
  not linked into any oracle binary.

## Workspace: 6 crates

| Crate | Contents (C++ provenance) |
|---|---|
| `kuna-base` | error/types, XML decoder, marshal streams, address/space, util (crc32, compression via flate2, filemanage), generic containers (partmap, rangemap) |
| `kuna-num` | float emulation, multiprecision, raw pcode + opcodes + constant op semantics (opbehavior) |
| `kuna-sleigh` | SLEIGH runtime decoder: `.sla` reader, patterns/symbols/context, pcode snippet parsing & compile-for-inject, emulator, loadimage |
| `kuna-decomp` | the decompiler engine: IR (S2), Band B analyses (S3–S6), structuring (S7/S8), rendering (S9), framework + P0 (action, architecture, database, options, kuna stage registry). One crate **by design** — Band B coupling makes finer crate boundaries dishonest (LOSS-003); stage modularity is expressed via the stage registry, not crates |
| `kuna-console` | interface/ifacedecomp/ifaceterm/codedata/kuna_console + the `decomp` bin (decomp_dbg-compatible surface) |
| `kuna-harness` | ported unit tests, XML datatest runner (test/testfunction), the `decomp_test` bin, lift-diff tooling |

## Waves

91 port items + 91 paired verify items + 18 infra items = 200 checklist items,
182,926 LOC of port scope (per-file `wc -l`, recorded with `git hash-object`
blob shas in `checklist.json`). Items within a wave parallelize; a wave's gate
item (`W<n>-gate`) blocks the next wave.

| Wave | Scope | Port LOC | Gate |
|---|---|---|---|
| W0 | workspace scaffold (6 crates), `port_audit` tool, checklist validator | — | `cargo build --workspace` + `check_checklist.py` |
| W1 | foundations: error/generics, xml, marshal, space/address, float/multiprec, pcoderaw/opcodes/opbehavior, util; testmarshal/testmultiprec | 13,405 | `cargo test -p kuna-base -p kuna-num` |
| W2 | SLEIGH runtime: translate, context, patterns, symbols, semantics/pcodecompile, pcodeparse, sleigh+slaformat, loadimage, emulator; testfloatemu | 21,264 | `cargo test -p kuna-sleigh` incl. `.sla` decode + lift fixtures vs C++ capture |
| W3 | core IR: varnode, op, block, funcdata*, flow, jumptable, userop/inject, kuna flow pack | 23,813 | IR construction fixtures green |
| W4 | framework + P0: action, options, database, architecture, override/comment, cpool/graph/callgraph, arch frontends, kuna stage registry/kassert/restartlog | 14,863 | arch init + P0 round-trip fixtures green |
| W5 | S3 Band B: heritage, **ruleaction in 8 line-range batches**, subflow, condexe/expression, transform, coreaction 1/5, kuna rule pack; **infra: kuna-lift-diff tool** | 27,448 | `cargo test -p kuna-decomp` + lift-diff self-test |
| W6 | S4+S5: fspec (3 splits), modelrules, type (3 splits), typeop, unionresolve, rangeutil, double, bitfield, constseq/prefersplit, coreaction 2/5, kuna S4/S5 pack; testfuncproto/testparamstore/testtypes/testcirclerange | 36,879 | those four unit-test suites green |
| W7 | S6+S7/S8: varmap, variable/cover, merge, dynamic pack, blockaction, kuna regionid/regiongraph/loweredswitch, coreaction 3/5; testkunaregion | 16,145 | testkunaregion green |
| W8 | S9 + assembly: printlanguage/cast/printjava, printc, prettyprint, stringmanage pack, coreaction 4/5 + 5/5 (`universalAction`), signatures | 13,219 | **M1: `cargo test --workspace` = 207/207 ported unit tests** |
| W9 | console: interface/ifaceterm/consolemain, ifacedecomp, codedata, kuna_console, grammar (hand parser), rulecompile/unify, harness runner; **infra: console bins + `--engine rust` plumbing** | 15,262 | **M2: `python -m kuna.run_tests --all --engine rust` runs end-to-end** |
| W10 | **infra: parity grind** — iterate divergences (via kuna-lift-diff) until full parity | — | **M3: PARITY OK 675/675 datatest assertions + 150/150 `tests/stages` + `catalog --check`, all `--engine rust`** |
| W11 | real-ELF loader on the `object` crate (replaces libbfd, LOSS-005); **infra: pipeline binding** | 628 | **M4: real-ELF decompile (`kuna.decompile --engine rust`) + `tools/pipeline` engine selection** |

### Milestones

- **M1** (W8 gate): all 207 upstream+kuna unit tests ported and passing under `cargo test`.
- **M2** (W9 gate): datatests run end-to-end through the Rust console (`--engine rust`); not necessarily green.
- **M3** (W10 gate): `PARITY OK` — 675/675 datatest assertions, 150/150 stage tests (`docs/baseline-stages.json`), `python -m kuna.catalog --check` against the Rust engine.
- **M4** (W11 gate): real-ELF decompile parity on a smoke corpus; pipeline worker can select the Rust engine.

## Splitting notes (oversized files)

- `ruleaction.cc` (11,032 LOC): split into 8 batches **by line range in
  definition order** — registration order in `ActionDatabase::universalAction`
  (`coreaction.cc:5776-5995`) interleaves rule groups non-contiguously across the
  file, so a registration-order split is not expressible as ranges. Boundaries
  fall on rule-class boundaries (recorded per item in `checklist.json`).
- `coreaction.cc` (6,034 LOC): 5 stage-aligned items — early flow/stack prep
  (W5), call/prototype (W6), CFG cleanup/deadcode/switch (W7), casts/naming/types
  (W8), `universalAction` registration (W8, depends on every action/rule item).
- `fspec.cc` (5,983) and `type.cc` (5,201): 3 line-range items each, split at
  class boundaries.
- Single coherent files over the 2.5k budget stay whole (block, jumptable,
  database, heritage, subflow, typeop, rangeutil, double, printc, ifacedecomp,
  grammar) — noted per item.

## Out of scope (excluded from the checklist)

- `slgh_compile.cc`, `slghparse.cc`, `slghscan.cc` — SLEIGH compiler (LOSS-001,
  deferred; permanent-by-design candidate).
- `ghidra_*` / `*_ghidra.cc` (12 files) — Ghidra-IPC client, not linked into any
  oracle binary (LOSS-002).
- `sleighexample.cc` — doc example (upstream `SPECIAL` group, never linked into
  the oracle binaries).
- `kuna_goldengen.cc/.hh` — in-flight oracle-side golden-fixture generator from a
  concurrent work stream (uncommitted when this checklist was built); it stays on
  the C++ side. Revisit for a checklist item once it lands.
- `doccore.hh`/`docmain.hh` — Doxygen-only.
