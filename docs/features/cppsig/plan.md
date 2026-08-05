# cppsig — implementation plan

## 1. The source — `demangle_raw`, already present and unused

`analyzers/demangle/mod.rs` exposes `demangle_raw` (full c++filt form, signature retained)
with no production caller; the production path throws the signature away via
`.no_params().no_return_type()` + `strip_bracket_groups`. This feature parses that retained
form into `PrototypePieces` and applies it. No new demangler dependency: the vendored
`cpp_demangle` / `msvc-demangler` crates already produce everything needed.

## 2. Where it runs — load time, analysis tier

The mangled name comes from `.dynsym`/`.symtab`, so this is a **loader/analyzer-tier** fact
established at `load file`, not a decompile pass. It composes with the DWARF path (#264)
rather than competing:

| binary | source of the prototype |
|---|---|
| unstripped, with DWARF | `cppproto` (#264) — real types, real parameter names |
| stripped, exported C++ | **`cppsig`** — declared types from the mangling |
| stripped, non-exported | neither; unchanged |

DWARF takes precedence where both exist: it carries ground truth, the mangling carries only
a *declaration*. State this precedence explicitly in the spec chapter.

## 3. The gate — a three-valued option

`cppsig = off | proven | inferred`, default **`proven`**. Three-valued because the
`this`-detection rule is the whole risk surface (analysis.md §3), and the precision/recall
split is real rather than cosmetic:

- `off` — byte-identical to the pre-PR tree (the two-pass test's "bug" arm).
- `proven` — precision 1.0000, recall 0.7093. The shipped default.
- `inferred` — precision 0.9278, recall 1.0000. Scores higher on leveldb (38 vs 35) and is
  offered for users who prefer coverage over never being wrong.

Full `settableTable` row in `phases.toml` (every field, incl. `tier` + `symptoms`) plus
registration in `src/p0_knowledge/options.rs`, with the analysis-tier env-var bridge used by
`relocobjects` / `cortexmvectors`.

## 4. Refusals — the cases the parser must decline rather than guess

Each of these was a measured false-positive source, not a theoretical one:

- **explicit function-template specializations** — `cpp_demangle` drops a parameter
  (measured harm 8 functions -> 2 once refused); also sidesteps the `maxof<int>` /
  `maxof<double>` name collision.
- **overloaded operators** — break the depth-tracking parse; densest false-positive source.
- **32-bit MSVC `__thiscall`** — `this` arrives in ECX, not argument slot 0. Refuse rather
  than mis-place it.

A refusal must leave the function exactly as `off` would.

## 5. Tests

- Two-pass end-to-end: `cppsig off` = untyped `a0`; default = the typed signature with
  `this`. The XML datatest bootstrap never produces an `AnalysisOutput`, so this is a Rust
  integration test in `kuna-console/tests/` — precedent `verify_cppcallnames.rs` (#262) and
  `verify_fdeinterior.rs` (#263).
- Static-member and namespaced-free-function cases asserted to gain **no** `this` — this is
  the parameter-shift regression guard.
- A reference-parameter case asserting `const Slice&` renders as `Slice *`.

## 6. Verification

- Four gates + `kuna catalog --check`, counters **derived** from a fresh green build.
- Speed: interleaved min-of-N, N >= 13 (an n=13 run showed +4.01% on a 108 ms case that
  resolved to +0.19% at n=31 — contention, not regression).
- Sweep: `decompile-all` before/after over C **and** C++ binaries, classifying every hunk,
  with the wrongness axis measured as **new read-before-write of an argument register**
  rather than raw arity change.
- Benchmark delta re-scored on decbench `main` @ `325046f` (the corrected scorer,
  `cache_version` 5) with `DECBENCH_NO_CACHE=1`.
