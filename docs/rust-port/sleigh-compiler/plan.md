# Porting `sleigh_opt` to Rust — the SLEIGH compiler (the last C++ dependency)

The decompiler core is fully ported (M3, 675/675 datatests). The remaining thing that
keeps the C++ tree load-bearing is **`sleigh_opt`**, the SLEIGH compiler: it compiles
`.slaspec → .sla`, and the `.sla` are gitignored build artifacts the Rust decoder
*consumes* but cannot *produce*. This effort ports the compiler so `.sla` can be built
without C++ — the final step toward removing the C++ tree.

## The oracle (settled, and it's a clean one)

C++ `sleigh_opt` is **byte-deterministic**: compiling a `.slaspec` twice yields a
byte-identical `.sla` (verified). So the gold gate is the strongest possible
differential test: **the Rust compiler's `.sla` must be byte-identical to
`sleigh_opt`'s, for every one of the 148 vendored `.slaspec`.**

- Final gate: `python -m kuna.slacomp --all` → **148/148 byte-identical**.
- End-to-end backstop: rebuild all specs with the Rust compiler, then re-run the
  decompiler suite (`run_tests --datatests`) → **still 675/675** (Rust-built specs
  decode identically to C++-built specs).
- Intermediate (per-stage) verification: instrument a **/tmp copy** of the C++ tree
  (the vendored `decompiler/cpp/**` stays byte-untouched) to dump token streams,
  symbol tables, pattern tables, and the pre-compression `.sla` element stream, and
  unit-test each Rust stage against those golden dumps. Plus the ported `TEST()`
  suites where they exist.

The byte-diff oracle is monolithic (you need lexer→parser→compile→encode all working
to byte-match one `.sla`), so the methodology is: interface-freeze skeleton → fill the
pipeline stage by stage with golden-dump unit tests → drive the whole pipeline to
byte-match, spec by spec.

## Scope — what's new vs already ported

`sleigh_opt` links `LIBSLA_NAMES` (the shared base, **already ported** on the consumer
side in `kuna-sleigh`: slaformat/sleigh/sleighbase/slghsymbol/slghpattern/
slghpatexpress/semantics/context/translate/types/space/address/xml/marshal) plus the
**compiler-specific** `SLACOMP = slgh_compile slghparse slghscan` and the *encode/build*
side of the shared modules. The new work:

| C++ source | LOC | Rust target | Notes |
|---|---|---|---|
| `slghscan.l` (flex) | 22 KB | hand lexer | bison/flex → hand parser (the established kuna pattern) |
| `slghparse.y` (bison) | 34 KB / 3805 gen | hand RD parser | builds the symbol/constructor AST |
| `slgh_compile.cc` | 4091 | `SleighCompile` driver | the bulk: symbol-table build, constructor compile, pattern resolution, ConsistencyChecker, MacroBuilder, SleighPcode, `.sla` emit orchestration |
| `pcodecompile.cc` | 781 | extend `pcodecompile.rs` | PcodeCompile semantic actions (partly present for `parse line`) |
| encode side | — | extend `slaformat.rs` + `slgh*` | `SleighSymbol::encode`, pattern/patexpress/semantics encode (`.sla` writer; reader already exists) |

CLI contract the Rust binary (`slacomp`) must match: `slacomp <file.slaspec>` writes
`<file>.sla` next to it (and `-a <dir>` recurses). Mirrors `sleigh_opt`.

## Crate

New `kuna-slacomp` crate (compiler), depending on `kuna-sleigh` (shared symbol/pattern/
semantics types) + `kuna-base`. Modules mirror the C++ files 1:1. Binary `slacomp`.
Resolved by `kuna/slacomp.py` as `rust/target/release/slacomp` (or `$KUNA_SLACOMP`).

## Waves (dependency-ordered, fan-out by module)

- **WS0** (foundation): differential harness (`kuna/slacomp.py`, done) + crate skeleton
  (interface freeze: module files + signatures + `todo!()` bodies) + golden-dump
  tooling (instrumented /tmp C++) + this doc.
- **WS1**: lexer (`slghscan.l` → hand lexer) — golden token-stream unit tests.
- **WS2**: parser (`slghparse.y` → hand RD parser) — golden symbol-table/AST dumps.
- **WS3**: PcodeCompile semantic actions (extend `pcodecompile.rs`).
- **WS4**: `SleighCompile` driver — symbol resolution, constructor compile, pattern
  resolution, ConsistencyChecker, MacroBuilder, with-blocks, context.
- **WS5**: the `.sla` encode/writer side (slaformat encode + all `SleighSymbol::encode`
  + pattern/patexpress/semantics encode).
- **WS6**: the `slacomp` binary + wire `make specs` to optionally use it (`SLEIGH`
  override) + the end-to-end backstop (rebuild specs with Rust, re-run 675/675).
- **WS7**: parity grind — drive `kuna.slacomp --all` to 148/148 byte-identical, spec by
  spec, simplest-ISA first (e.g. 6502/AVR) → x86/ARM.

## Methodology (same as the decompiler port)

Orchestrated background porter agents (Opus, worktree-isolated, file-disjoint),
serially integrated by a single integrator through the rigorous gate: build clean →
`cargo test --workspace --no-fail-fast` (0 failures) → the relevant differential
(`kuna.slacomp` byte-diff for landed specs; golden-dump unit tests per stage) →
the decompiler suite stays 675/675 and C++ oracle PARITY OK → C++ tree byte-untouched.
Each wave's diagnosis/divergence recorded in `docs/rust-port/sleigh-compiler/`.
