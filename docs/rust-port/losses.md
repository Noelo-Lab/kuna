# Rust port loss ledger

**Append-only.** Every deliberate departure of the Rust port from the C++
oracle gets a numbered entry here at the moment the decision is made (porter or
verifier appends; nobody edits or deletes an existing entry — supersede with a
new entry that references the old id). Verifiers cite entries by id in
`ACCEPT-WITH-LOSSES` verdicts.

Entry schema (every field required):

```
## LOSS-NNN: <one-line title>
- date: YYYY-MM-DD
- kind: deferral | exclusion | dependency-substitution | paradigm-degradation
- cpp-anchor: <file(:line) in decompiler/cpp/ that embodies the C++ behavior>
- rust-anchor: <file/crate in rust/ that embodies the departure, or 'none yet'>
- surface: <what user-visible or test-visible surface is affected>
- why: <why the departure was taken>
- restoration criteria: <what would have to be true/built to close the loss>
```

---

## LOSS-001: SLEIGH compiler stays C++
- date: 2026-06-10
- kind: deferral (permanent-by-design candidate)
- cpp-anchor: decompiler/cpp/slgh_compile.cc, slghparse.cc, slghscan.cc (upstream Makefile `SLACOMP` group; grammars slghparse.y/slghscan.l)
- rust-anchor: none yet
- surface: the `.slaspec -> .sla` build step (`make specs`); Rust binaries consume `.sla` only
- why: the compiler is build-time tooling, not part of any oracle binary (`decomp_dbg`/`decomp_test_dbg` link `CORE+DECCORE+SLEIGH+EXTRA`, never `SLACOMP`); porting ~11.5k LOC of bison/flex-generated compiler buys zero parity
- restoration criteria: a pure-Rust toolchain requirement materializes; port slgh_compile plus hand-written grammar replacements, gated on byte-identical `.sla` output for all vendored `.slaspec` modules

## LOSS-002: ghidra_* IPC client unported
- date: 2026-06-10
- kind: exclusion
- cpp-anchor: decompiler/cpp/ghidra_arch.cc, ghidra_context.cc, ghidra_process.cc, ghidra_translate.cc, inject_ghidra.cc, comment_ghidra.cc, cpool_ghidra.cc, database_ghidra.cc, loadimage_ghidra.cc, string_ghidra.cc, typegrp_ghidra.cc, signature_ghidra.cc (upstream Makefile `GHIDRA` group)
- rust-anchor: none yet
- surface: the Ghidra-plugin decompile server (`ghidra_dbg`/`ghidra_opt` binaries)
- why: not linked into any oracle binary kuna tests; kuna's harness drives the console binaries only
- restoration criteria: a Ghidra-plugin binding for the Rust engine is wanted; port the IPC marshalling against a live Ghidra and validate with Ghidra's Java-side protocol tests

## LOSS-003: Band-B coupling forces one kuna-decomp crate
- date: 2026-06-10
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/ruleaction.cc / coreaction.cc cross-stage reads (S3 rules read/write S5 type facts etc.; see docs/stage-mapping.md straddler notes and docs/stage-model.md Band B)
- rust-anchor: rust/kuna-decomp (none yet)
- surface: crate graph: stages S2-S9 share one crate instead of one-crate-per-stage; stage modularity is expressed via the stage registry (`kuna_stages` port), not crate boundaries
- why: Band B is a genuine fixed-point coupling — slicing it into crates would force either circular dependencies or a trait soup that falsifies the oracle's structure
- restoration criteria: Band B interfaces narrowed to explicit traits with no loss of parity; then split kuna-decomp along the S3/S4/S5/S6 seams and have `cargo` enforce what the stage registry currently only documents

## LOSS-004: zlib -> flate2
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/compression.cc:29-53 (zlib deflate/inflate streams)
- rust-anchor: none yet (rust/kuna-base, W1 item w1-base-util)
- surface: compressed `.sla` payload read (and write, for cache regeneration)
- why: flate2 is the standard Rust DEFLATE implementation; binding C zlib would add an FFI dependency for an identical wire format
- restoration criteria: not needed if round-trip stays byte-identical; gate: decompress-then-recompress every shipped `.sla` and compare payload bytes against the C++ path

## LOSS-005: libbfd -> object crate (W11)
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/bfd_arch.cc, loadimage_bfd.cc (BfdArchitecture/LoadImageBfd)
- rust-anchor: none yet (rust/kuna-decomp, W11 item w11-loader-object)
- surface: real-ELF loading in the console (`load file`), i.e. `kuna.decompile` against actual binaries; datatests are unaffected (XML loadimage)
- why: libbfd is a GPL C dependency that already needs `tools/fetch_bfd.sh` gymnastics; the `object` crate is the Rust ecosystem standard and covers the formats kuna exercises
- restoration criteria: not needed if M4 holds: identical bytes loaded, identical symbol/entry resolution, identical decompilation on the real-ELF smoke corpus vs the libbfd-linked C++ oracle

## LOSS-006: bison/flex grammars -> hand-written parsers
- date: 2026-06-10
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/xml.y -> xml.cc; grammar.y -> grammar.cc (`parse line` C declarations); pcodeparse.y -> pcodeparse.cc (pcode injection snippets). (slghparse.y/slghscan.l fall under LOSS-001; ruleparse.y is conditionally compiled, CPUI_RULECOMPILE.)
- rust-anchor: none yet (rust/kuna-base xml; rust/kuna-sleigh pcodeparse; rust/kuna-console grammar)
- surface: XML datatest/save-file parsing; console `parse line` declarations; `<pcode>` compilable injections in .cspec/.pspec
- why: no bison/flex in the Rust toolchain, and transliterating generated parser tables would be unmaintainable and unreviewable; hand-written recursive-descent parsers replace the generated automata
- restoration criteria: per parser, a grammar-conformance corpus extracted from the oracle (all vendored specs, all datatests, error cases) with identical accept/reject behavior and identical error text wherever the harness or datatests can observe it

## LOSS-007: std::regex -> regex crate in the datatest runner
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/testfunction.hh:39 + testfunction.cc:31-75 (FunctionTestProperty, ECMAScript std::regex over output lines)
- rust-anchor: none yet (rust/kuna-harness, W9 item w9-harness-runner)
- surface: matching of the 675 datatest assertion patterns against decompiler output
- why: std::regex (ECMAScript dialect) has no Rust equivalent; the regex crate differs in dialect (no backreferences/lookaround)
- restoration criteria: corpus proof — every pattern in all 83 datatest files (and tests/stages) shown to produce the same match decision on the oracle's captured output under both engines; any pattern relying on unsupported syntax is flagged here and rewritten upstream-compatibly

## LOSS-008: goldens B3 boundary anchored at `break start paramdouble`, not heritage completion
- date: 2026-06-10
- kind: pinned-by-fiat
- cpp-anchor: decompiler/cpp/coreaction.cc (universalAction mainloop: ActionParamDouble immediately follows ActionHeritage); decompiler/cpp/action.cc (break_action only fires after an active transformation — ActionHeritage registers no change count, so `break action heritage` provably never triggers; verified empirically)
- rust-anchor: kuna/goldens.py (B3 capture)
- surface: differential-testing boundary only; no product behavior
- why: the intended anchor "first heritage completion" has no directly breakable event; `break start paramdouble` is the next action start in the same mainloop iteration, which is exactly first-heritage-completion (snapshots show fresh SSA def links)
- restoration criteria: none needed for parity; the Rust engine must replicate the paramdouble registration point so inv2 replay lines up. If the mainloop ordering ever changes, re-derive the anchor.

## LOSS-009: `print tree varnode` snapshots normalized (heap-address scrub + line-block sort)
- date: 2026-06-10
- kind: pinned-by-fiat
- cpp-anchor: decompiler/cpp printing of varnode trees: `(internal=0x...)` per varnode; AddrSpace*-valued constants (LOAD/STORE space pointers) print and SORT by raw heap pointer
- rust-anchor: kuna/goldens.py (_normalize for B3/B4)
- surface: differential harness only — B3/B4 varnode-tree comparison is multiset-style per line block (op/block ORDER stays exact via print raw / print tree block)
- why: process heap addresses are nondeterministic across runs/engines; scrubbing (regex `internal=`, 12-hex 0x[5-7]... constants -> `#(spaceptr)`) + per-block sorting makes the snapshot deterministic (proven: cpp-vs-cpp 131/131 identical). Slight diff-discrimination loss: a genuine 48-bit constant matching the scrub pattern would be masked.
- restoration criteria: a deterministic C++-side dump printing space NAMES instead of pointers (natural home: the `golden` console command in kuna_goldengen.cc); then drop the normalization.
