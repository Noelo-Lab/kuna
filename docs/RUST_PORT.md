# kuna — the Rust port of Ghidra's decompiler (and SLEIGH compiler)

kuna began as a verbatim extraction of Ghidra's C++ "deep decompiler" (the SLEIGH-based
engine) lifted out of the Java application. This document summarizes the project's second
act: a **complete, line-faithful port of that C++ engine — and the SLEIGH compiler — to
Rust**, and the removal of the C++ tree once the port was proven equivalent.

> **Status:** done. The Rust port passes **675/675** decompiler datatest assertions and
> compiles **148/148** SLEIGH language specs to byte-identical `.sla`. The vendored C++
> source (`decompiler/cpp/`, ~196k LOC) and its C++ unit tests have been removed. The Rust
> workspace under `rust/` is the engine; the XML regression corpus
> (`decompiler/datatests/`) and the recorded oracle (`docs/baseline.json`) remain.

## What was ported

Two C++ programs, both reproduced in Rust with byte-level output parity:

1. **The decompiler** (`decomp_dbg` / `decomp_test_dbg`, the DECCORE library, ~all of
   `decompiler/cpp/*.cc`): lift → flow/CFG → SSA heritage → the ~170-rule simplification
   pipeline → type inference → variable merging → structuring → C emission. Ported into the
   `rust/` cargo workspace:
   - `kuna-base` — types, error model, XML, marshal (packed + XML), opcodes, space/address,
     pcoderaw, globalcontext, partmap/rangemap, crc32, filemanage, compression.
   - `kuna-num` — multiprecision, float/FloatFormat, CircleRange.
   - `kuna-sleigh` — the `.sla` reader + decode runtime (slaformat, context, slghsymbol,
     slghpattern, slghpatexpress, semantics, sleighbase, sleigh) — and now the SLEIGH
     **compiler's** pattern-build machinery (see below).
   - `kuna-decomp` — the whole DECCORE: varnode/op/block/funcdata, type system, fspec,
     database, heritage, merge, dynamic, jumptable, flow, the rule/action engine
     (`coreaction`, `ruleaction`, subflow, transform, blockaction, condexe), printc/print
     stack, and the kuna stage model.
   - `kuna-console` — the interactive command surface + the `decomp_dbg`/`decomp_test_dbg`
     binaries (same console grammar as upstream, incl. the datatest runner).
   - `kuna-harness`, `kuna-lift-diff` — differential dev helpers.

2. **The SLEIGH compiler** (`sleigh_opt`, ~14k LOC: `slgh_compile.cc`, the bison/flex
   grammar `slghparse.y`/`slghscan.l`, `pcodecompile.cc`): compiles processor `.slaspec`
   definitions to the `.sla` binary form the decoder consumes. Ported as the
   **`kuna-slacomp`** crate (binary `slacomp`): a hand lexer + hand recursive-descent parser
   (the established bison/flex → hand-parser pattern), the `SleighCompile` driver, the
   `ConsistencyChecker`, `SleighPcode`/`MacroBuilder`, and the `.sla` encoder; with the
   compile-side pattern/equation/decision-tree construction landed additively in
   `kuna-sleigh`.

Deliberately **not** ported (never linked into the decompiler oracle, or out of scope):
the Ghidra Java client (`ghidra_*`), `printjava`, `rulecompile`/`unify`, and the
graph/callgraph dump commands. These were dropped, not reimplemented.

## Why

The C++ engine is excellent but hard to study, instrument, and extend stage-by-stage inside
the Ghidra build. A faithful Rust port gives a memory-safe, modular, independently-buildable
engine with a first-class stage model and an LLM/human control surface — while the SLEIGH
compiler port is what finally lets `.sla` be produced without C++, removing the last reason
the C++ tree had to exist to *run* the pipeline.

## How — the porting methodology

The same discipline drove every wave of both ports:

- **Differential, oracle-driven.** The untouched C++ binary is the oracle. The decompiler's
  oracle is `decomp_test_dbg` over the XML datatests (and a recorded `baseline.json`); the
  SLEIGH compiler's oracle is `sleigh_opt`, which is byte-deterministic, so its oracle is the
  produced `.sla` itself.
- **Faithful-port-first.** Rule/action bodies, comparators, iteration order, and registration
  order were transcribed line-by-line from a pinned Ghidra commit — because the decompiler's
  output is a function of rule-application order. Ordered containers use `BTreeMap`/`BTreeSet`
  with `Ord` impls transcribing the C++ comparators field-by-field; `HashMap`/`HashSet` are
  banned workspace-wide. The IR uses generational-arena slotmaps (`VarnodeId`/`OpId`/`BlockId`)
  rather than `Rc<RefCell>`, with all mutation through `Funcdata` methods named after the C++
  API. Dependency substitutions (zlib→flate2, libbfd→object, bison/flex→hand parsers,
  std::regex→regex) are recorded as losses.
- **Fan-out, serial-integrate.** Work was decomposed into file-disjoint "waves," each ported
  by a background agent in an isolated git worktree, then integrated **one at a time** by a
  single integrator under a rigorous gate (below). Diagnoses, divergences, and dead-ends are
  recorded append-only in `docs/rust-port/losses.md` and the per-wave review files — including
  the many cases where a seeded diagnosis turned out wrong and had to be re-derived from live
  dual-engine traces.

## How we know the port is valid (the validation gates)

Every integration passed, and the final state satisfies, all of:

1. **Datatest parity — 675/675.** `python -m kuna.run_tests --datatests` runs the Rust
   `decomp_test_dbg` over the 83-file / 675-assertion XML regression corpus; all pass, with
   **no** previously-passing assertion ever regressing (a per-assertion passing-set diff
   enforced monotonicity at every wave). `docs/baseline.json` records the oracle.
2. **The byte-parity fence.** `cargo test --workspace --no-fail-fast` includes ported
   `TEST()` suites (the 207 upstream unit tests, transcribed 1:1), golden differential vectors
   (opbehavior/float/comparator/XML-DOM), per-instruction SLEIGH lift-diff over the corpus, and
   byte-equality probes — the catcher for divergences the assertion-level datatests can mask.
3. **Stage boundaries (B0–B5).** The pipeline registration (`list action`), lift, post-flow
   CFG, post-heritage SSA, final IR, and printed C were each pinned byte-equal to C++ via
   console-command snapshots.
4. **SLEIGH compiler — 148/148 content-identical + the end-to-end backstop.**
   `python -m kuna.slacomp --all` confirms every `.slaspec` compiles to a `.sla` whose
   decompressed element stream is byte-identical to `sleigh_opt`'s. The decisive proof:
   rebuilding **all** specs with the Rust compiler and re-running the full decompiler suite
   yields **675/675** — the Rust-built specs decode identically to C++-built ones. (Per-stage,
   each compiler module was checked against golden token/symbol/pattern/decision-tree dumps
   from an instrumented `/tmp` copy of the C++ tree.)
5. **Oracle untouched.** Throughout both ports the vendored `decompiler/cpp/**` and
   `docs/baseline.json` stayed byte-for-byte unmodified — the parity results are differential
   against an unmoved reference, not self-graded.

The detailed audit trail lives in `docs/rust-port/` (the decompiler port: `losses.md`,
`reviews/`, `stage-*.md`) and `docs/rust-port/sleigh-compiler/` (the compiler port: `STATUS.md`
and the per-wave `ws*.md`).

## The C++ tree removal

With both ports proven, the C++ source was removed:

- **Removed:** `decompiler/cpp/` (the C++ decompiler + SLEIGH compiler source) and
  `decompiler/unittests/` (the C++ unit tests — reproduced as cargo workspace tests), plus the
  now-dead libbfd tooling.
- **Kept:** `decompiler/datatests/` (the XML regression corpus the Rust harness consumes),
  `specs/` (the SLEIGH `.slaspec` definitions; `.sla` are gitignored build artifacts now
  produced by `slacomp`), and `docs/baseline.json` (the recorded oracle, now a frozen
  Rust-validated datatest snapshot).
- **Rewired:** the `Makefile` builds and tests the Rust port only (`make binaries`/`specs`/
  `test`/`rust-test`); `kuna/paths.py` defaults to the Rust engine (`sleigh_opt`→`slacomp`,
  the decompiler binaries keep their names); `kuna/run_tests.py` scopes the baseline check to
  the modes it runs (the unit tests now live in the cargo workspace, not the datatest harness).

### The one cosmetic residual (LOSS-010)

A `.sla` file is a 4-byte magic + a zlib stream. The *decompressed element stream* the Rust
compiler emits is byte-identical to C++; the *compressed bytes* differ because the Rust deflate
backend (`flate2`/miniz_oxide) chooses a different (equally valid) DEFLATE encoding than C zlib.
This is purely the zlib→flate2 dependency substitution — both files decode identically, which is
why the content (not the raw bytes) is the gate. A zlib-bit-compatible deflate backend (e.g.
`zlib-rs`) would yield literal whole-file byte-identity if ever wanted; it is not needed for
correctness.

## Building and testing now

```sh
make binaries     # cargo build the Rust decomp_dbg/decomp_test_dbg + slacomp
make specs        # compile every .slaspec -> .sla with the Rust SLEIGH compiler (slacomp)
make test         # the 675/675 datatest parity (Rust harness + baseline) -- needs no C++
make rust-test    # the full cargo workspace test suite (ported units + golden + .sla parity)
```

`make test` is the self-sufficient correctness gate: it compiles the specs with the Rust
`slacomp` and decodes the corpus with the Rust decompiler, end to end, with no C++.

`python -m kuna.slacomp --all` (the per-spec `.sla` content byte-diff) was the SLEIGH-compiler
validation gate **during** the port; it diffs against the C++ `sleigh_opt`, so re-running it now
requires building that oracle out-of-tree from upstream Ghidra. Its result (148/148) is recorded
and is subsumed by the `make test` backstop (the Rust-built specs decode to 675/675).

The C++ tree is gone; the Rust port is the engine.
