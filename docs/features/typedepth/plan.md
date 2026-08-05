# typedepth — implementation plan

## 1. The change — restore upstream's guard, do not raise the cap

Replace `build_datatype`'s `depth: u32` parameter with a recursion guard threaded
through the walk (`analyzers/dwarf/kuna_typedepth.rs`):

```rust
pub(super) enum TypeWalk {
    Depth { depth: u32 },                                   // the pre-fix budget
    Cycle { counts: BTreeMap<usize, u32>, nesting: u32 },   // trackRecursion
}
```

`build_datatype` brackets every recursive step with `enter(off)` / `leave(off)`; a
refusal yields `void` exactly where the cap used to. `Cycle` is upstream's rule — a
DIE offset may be re-entered twice and the third entry is refused — plus
`MAX_NESTING = 64` as a native-stack backstop that upstream (Java) does not need.
`Depth` is the old budget verbatim, so `--option typedepth off` is byte-identical.

**Termination**: every recursive step is bracketed, an offset already twice on the
current path is refused, and a compilation unit has finitely many offsets — so no
walk is infinite. The nesting bound additionally caps a *long* (never repeating)
forged chain before it can exhaust the stack.

## 2. Two consequences that ride along

- The qualifier collapse #264 added for C++ (`strip_qualifiers`) now runs for the C
  callers too. Under the cycle guard it is no longer needed to survive a budget, but
  it is what carries an anonymous aggregate's typedef name onto it (`mbstate_t`, not
  the shared `anon_struct`).
- Therefore the borrowed name can collide: `intern_aggregate` falls back to the
  anonymous name when the alias does not yield an aggregate (analysis.md §4). Only
  an aggregate with no name of its own falls back.

## 3. The gate — an env-var bridge, and why it cannot be a commit gate

`cppproto` gates at the analysis **commit** because it selects between two fact
sets. This one changes how a *single* fact set is built, inside `load file`, which
runs upstream of every `option` command — so the live gate is a process env var
(`kuna_decomp::kuna_typedepth::TYPEDEPTH_ENV`), the `relocobjects` /
`i386_pie_plt` pattern:

- `p0_knowledge/kuna_typedepth.rs` — `TYPEDEPTH_ENV`, `typedepth_enabled()`,
  `set_typedepth_env()`.
- `Architecture::analysis_typedepth` + a `set_kuna_option` arm that also writes the
  env var (for `option typedepth off` *before* `load file` in one process).
- `kuna decompile` exports it onto the `decomp_dbg` subprocess;
  `kuna decompile-all` adds it to `apply_loadtime_env` + `is_loadtime_gate`.
- `kuna_console.rs` maps it for `phase catalog`'s live `current` field.

Full `settableTable` row in `phases.toml` (every field incl. `tier` + `symptoms`)
plus registration in `p0_knowledge/options.rs`.

## 4. Default-ON, with a DIV row

Every truncation the budget produced was a concrete DWARF type replaced by `void`,
the types come from the debug info rather than from inference, and the parity gates
are structurally untouched (the XML datatest bootstrap never runs the analyzer
tier). DIV row in `docs/history.md`; `--option typedepth off` restores the old
mapping byte for byte.

## 5. Tests

- `kuna-console/tests/verify_typedepth.rs` — six two-pass e2e assertions against a
  new `-g -O0` C fixture (`typedepth_x86_64`, source vendored): the local
  `const char *const *`, three pointer hops, the anonymous-typedef alias, the
  `code` name collision, the global's extent, and termination on self-referential
  and mutually recursive structs. A Rust integration test rather than stage XML
  because the XML `<binaryimage>` bootstrap never runs the analyzer tier
  (precedent: `verify_cppcallnames.rs` #262, `verify_fdeinterior.rs` #263,
  `verify_cppsig.rs` #266). The gate is set through the env var **before**
  `bootstrap_from_object`, serialized by a mutex, because the mapping happens at
  load.
- Hermetic unit tests in `kuna_typedepth.rs` (the guard's own algebra) and in
  `dwarf/mod.rs` — a **forged** `.debug_info` whose type chain closes on itself
  (self-pointer, mutually pointing `typedef`/`const`, self-referential array) must
  terminate under both arms, and the four-DIE `const int *const *` must resolve
  under the guard and truncate under the budget.

## 6. Verification

- Four gates + `kuna catalog --check`, every counter **derived** from a fresh green
  build (`settable` 92 -> 93, tier analysis 31 -> 32, catalog JSON rows 91 -> 92,
  bytecompat fixture regenerated).
- Speed: interleaved min-of-N, N >= 13, on the load-dominated surface (the mapper
  runs once per binary at `load file`).
- Sweep: `decompile-all` off vs on over `-g` C binaries **and** a C++ one, every
  changed line classified, with the wrongness axis being a wrong concrete type
  asserted where `void *` was honest.
- Benchmark: the standard (stripped) decbench corpus carries no DWARF, so the
  delta there is structurally zero — verified, not assumed. The measured arm is
  decbench `type_match` over the same projects' unstripped `compiled/` copies,
  scorer `main` @ `325046f`, `DECBENCH_NO_CACHE=1`.
