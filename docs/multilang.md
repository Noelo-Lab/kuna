# Multi-language source support (Rust / Go / Swift)

Status: **draft architecture / [PROPOSAL] material** — not yet implemented.

kuna decompiles every binary as if a C compiler produced it: NUL-terminated
strings, one return value, the format-derived cspec, C struct layout, and C
control-flow idioms. Rust, Go and Swift binaries violate every one of those
assumptions, each in its own way. This document is the top-down plan for making
the *source language* a first-class, per-run-flippable dimension of the
pipeline — the same move kuna already made for the *output* language
(`p9_emit/kuna_lang.rs`, PRs #320–#323), applied to the input side.

Everything here builds on seams that already exist. The repeated finding from
the survey below is that kuna has grown five or six *partial* language planes
independently; the refactor is mostly consolidation plus three genuinely new
mechanisms (the idiom-reduction engine, the type-facet table, and
length-carried strings).

## 1. What exists today (survey)

| Area | What exists | Where | Gap |
|---|---|---|---|
| Detection | `detect_compiler` → `Compiler{Gcc,Clang,Rustc,Go,Unknown}` | `kuna-analysis/src/analyzers/sourcelang/mod.rs` | Recomputed at **7 call sites**; reaches the engine as only two bools (`Architecture::source_is_rust/source_is_go`, set at `kuna-console/src/engine.rs:4343`); no Swift variant |
| ABI | `rustabi` option (ScalarPair returns); vendored **golang cspecs** for x86-64/x86-32/AArch64 (`abi-internal`, `abi0`, `gcwrite_*`, `duffzero/duffcopy`) | `p4_calls/kuna_rustabi.rs`; `specs/.../x86-64-golang.cspec` etc. | The golang cspec loads only via an explicit `--target …:golang`; detection never selects it; no Swift models anywhere; no way to bind a prototype *model* to a symbol without a full signature |
| CFG idioms | Rust panic/drop handling: `cleanupcode`, `securitycheck`; plus the stripper family (`stackguard`, `msvcstackguard`, `devectorize`, `msvcstrappend`, `strlenscan`, `outline`) | `p2_lift/kuna_cleanupcode.rs`, `p7_regions/kuna_securitycheck.rs`, … | All hand-coded; **no generic subgraph matcher**; nothing for Go (`morestack`, write barriers, `panicIndex`) or Swift (ARC, exclusivity) |
| Types | Rust DWARF enum layouts parsed into `VariantLayout` side table | `analyzers/dwarf/kuna_dwarfvariants.rs`, `p0_knowledge/kuna_dwarfvariants.rs` | **Nothing reads the table yet**; no notion of slices/fat pointers/interfaces/tuples; zero-sized fields dropped; sizes are `int4` so unsized types unrepresentable |
| Strings | NUL-terminated scan (`StringLiteralPass`), NUL-terminated render (`StringManagerUnicode::get_string_data`) | `analyzers/strings/mod.rs`, `p9_emit/stringmanage.rs` | Go/Rust strings carry length, not NUL; Go rodata is one unterminated blob. BUT: a bounded, NUL-free literal path already exists — `Funcdata::get_internal_string` → `BUILTIN_STRINGDATA` (used only by `constseq.rs`) |
| Symbols | pclntab pass (names + extents, go1.2–1.20); Rust v0/legacy + Itanium + MSVC demangling; per-language no-return lists | `analyzers/pclntab/`, `analyzers/demangle/`, `loader/noreturn.rs` + `data/{Rust,Golang}FunctionsThatDoNotReturn` | No Go runtime type info (typelinks/moduledata); Swift demangler done (`kuna_swift`, `$s` + `_Tt`); `__swift5_*` metadata naming done (`analyzers/swift`, option `swiftmetadata`); struct/class layouts from static type metadata done (`swifttypes`; symbolic references resolved); `__swiftcall` / `__swiftcall_throws` cspec models with model-only bindings and an unlocked `self` hint (`swiftcall`), throwing calls detected by the caller's zeroing of the error register; the error value is still an anonymous local, not a second return |
| Output | Full outlang plane: `OutLang{C,Rust}`, `LangProfile`, `TypeSpeller`, `LangAbi` (stub: `extern_marker` only, doc names Go multi-return as the known omission) | `p9_emit/kuna_lang.rs`, `kuna_langrust.rs`, `p4_calls/kuna_langabi.rs` | Rust output has no type definitions renderer; no tuple/multi-return form |

Two structural facts shape the whole design:

1. **Detection runs inside `load file`, before any `option` line** (the
   stash→flags→commit protocol, `docs/spec/00-overview.md` §0.1). A user
   override of the detected language therefore has to be a *loadtime-tier*
   option (env binding via `kuna-cli/src/loadtime.rs`), like `relocobjects`.
2. **`type_metatype` is effectively closed** (~3.7k references across 380
   files, mostly non-exhaustive matches; `kuna_dwarfvariants.rs` already
   examined and rejected adding a variant). Language type semantics must ride
   *beside* the C-shaped type, not inside the metatype lattice.

## 2. Architecture: one fact, five workstreams

```
            ┌───────────────────────────────────────────────────────┐
            │ W1  SourceLang plane (P0 fact: detect once, override) │
            └──────┬──────────┬───────────┬───────────┬─────────────┘
                   │          │           │           │
            W2 idiom      W3 type      W4 ABI/     per-language packs (W5)
            reduction     facets       strings     Go / Rust / Swift rule
            engine        (factory     (models,    tables + metadata
            (generic)     side table)  joins,      harvesters
                                       (ptr,len))
```

The generic mechanisms (W1–W4) are language-neutral and land first; each
language is then a *pack*: a rule table + a metadata harvester + a handful of
options, with no new mechanism of its own.

### W1 — The source-language plane

- **`SourceLang` enum** `{Unknown, C, Cpp, Rust, Go, Swift}` as a P0 image
  fact: detected once at bootstrap (extend `sourcelang` with Swift:
  `__swift5_types`/`__swift5_proto` Mach-O sections, `.swift5_*` ELF sections,
  `$s`/`_T0` mangled symbols, `swift_release` imports), stored on
  `Architecture` and copied into `ArchContext` — replacing the
  `source_is_rust`/`source_is_go` bools and the 7 scattered `detect_compiler`
  re-parses (engine bootstrap, `decompile_all.rs:1812` outlang auto-policy,
  `runtime_hints.rs`, wasm `resolve_language`, `kuna_addrtakenentry.rs`, …).
- **Override**: `sourcelang auto|c|cpp|rust|go|swift` as a loadtime-tier
  option (env binding), because pass selection (`passes_for(compiler,format)`)
  happens during `load file`. The existing Mach-O `_R`-prefix false-positive
  probe (`tests/cli/a-c-function-whose-name-starts-with-r-…`) becomes a
  regression test for the consolidated detector.
- **Image-fact stream**: `AnalysisOutput` gains whole-image facts (language,
  Go version from buildinfo/pclntab magic, pclntab presence) so analysis
  passes publish them like any other fact instead of `engine.rs` writing
  `Architecture` fields directly at line 4350.
- The `auto` outlang policy and wasm duplicate then read this one fact.

### W2 — Generic node-reduction engine (control flow)

The survey found no generic matcher, but every needed *edit primitive* exists
and is battle-tested: `remove_branch` (+phi patching), `remove_unreachable_blocks`,
`op_destroy`+`delete_call_specs`, synthesized `CALL` with locked
`FuncCallSpecs` (outline/msvcstrappend), typed `CALLOTHER` builtins
(strlenscan), `node_split` (the expand direction), and `structure_reset` as
the universal invalidation. There are also three proven timing windows:

- **pre-SSA** (`get_heritage_pass()==0`, top of mainloop — where
  `cleanupcode`/`loweredswitch-install` run): for idioms that must vanish
  before heritage sees them (Go `morestack` entry loop).
- **post-SSA fullloop tail** (where `securitycheck`/`stackguard`/`devectorize`
  run): for idioms needing dataflow proof (write-barrier diamonds).
- **sblocks/emit-time** (gotoreduce family, iteregion flags): for rendering
  choices that keep the IR intact.

**Proposal: `infra/idiom.rs` + one `ActionIdiomReduce` per window**, driven by
declarative per-language rule tables:

```rust
struct IdiomRule {
    lang: SourceLang,            // gate (Unknown = any)
    window: Window,              // PreSsa | PostSsa | Region
    shape: Shape,                // the subgraph pattern (below)
    callee: CalleeClass,         // name set / wildcard list / runtime class
    action: Reduce,              // what N nodes become
    option: &'static str,        // every rule is option-addressable
}
enum Shape {
    GuardedColdCall { rejoin: bool },   // cmp+CBRANCH → cold arm calls X (noreturn or not), rejoin/halt
    EntryGuardLoop,                     // entry: cmp SP/limit → call X → back-edge to entry
    CallStraightLine,                   // bare call to X in fallthrough code
    DiamondStore,                       // if (flag) { call X } else { plain op }; both arms converge
}
enum Reduce {
    DeleteColdArm,                       // securitycheck-style: remove_branch + unreachable sweep
    DeleteCall,                          // cleanupcode-style: op_destroy + delete_call_specs
    ToBuiltin { op: u32 },               // strlenscan-style: CALLOTHER(builtin), typed, survives DCE
    ToCall { name: &'static str },       // outline/msvcstrappend-style: synthesized locked call
    Annotate { flag: u32 },              // iteregion-style: mark, re-match at emit
}
```

This is deliberately a *shape vocabulary*, not a general graph grammar: four
shapes cover every Rust/Go/Swift idiom identified below, each shape compiles
to one hand-verified matcher over `bblocks` + def-use (the same walks the
existing strippers do), and new shapes are added only with a motivating idiom.
A general subgraph isomorphism engine would be slower than the four bespoke
matchers and much harder to prove safe. The existing hand-coded options stay
as they are (no churn); the engine is the template for all *new* reductions,
and each table row still ships behind its own named option per the standing
requirements.

The reverse direction (1→N expansion) already exists as `node_split` /
`return_split_apply` and needs no new machinery; the engine exposes it as a
`Reduce::Split` arm only if a concrete language idiom ever needs it.

### W3 — Type system: facets beside C shapes, never new metatypes

Follow the three extension patterns the codebase already proved (flag bits,
factory side tables, non-interned overlays). Core move: a **`LangFacet` side
table on `TypeFactory`** (exactly the `VariantLayout` pattern, generalized),
keyed by type name/id, describing what a C-shaped struct *means*:

```rust
enum LangFacet {
    GoString,                          // {ptr,len}
    GoSlice { elem: TypeRef },         // {ptr,len,cap}
    GoIface { eface: bool },           // {itab/type, data}
    GoMap, GoChan,
    RustSlice { elem: TypeRef },       // &[T] = {ptr,len}
    RustStr,                           // &str
    RustDyn { trait_name: String },    // {data, vtable}
    RustEnum(VariantLayout),           // already exists — becomes a facet arm
    SwiftString, SwiftExistential,
    Tuple(Vec<TypeRef>),
    MultiReturn(Vec<TypeRef>),         // W4's joined-return bundle
}
```

The underlying `Datatype` stays an ordinary sized struct (`{ptr,len}` etc.),
so **all 380 files of metatype consumers work unchanged**; the facet is read
only where language behavior differs:

- **P5 propagation**: a facet-carrying struct seeds field types (`GoSlice.elem`
  propagates through the data pointer) — a small number of new arms in
  `propagate_*`, gated on facet presence.
- **P9 printing**: the `TypeSpeller` consults facets — Rust outlang prints
  `&[T]`, `&str`, `Option<T>` (via the until-now-unread `VariantLayout` niche
  info); C outlang prints the struct but names it (`go_string`,
  `rust_slice_u8`) and can render variant-tagged union reads with the variant
  name instead of `._0_8_`.
- **Unsized types** (`str`, `[T]`, `dyn Trait`) are never represented
  directly — facets live on the *fat pointer / header struct*, which is
  sized. This sidesteps the `int4`-size and zero-sized-type limits entirely.

**Providers** (all through the existing `AnalysisPass → AnalysisOutput →
typed_data/prototypes` seam):
- DWARF (Rust and unstripped Go both emit it; `kuna_dwarfvariants` already
  parses the hard part), tagging facets from `DW_AT_language` + shape.
- **Go runtime type metadata** (moduledata → typelinks → `_type` graph): full
  struct/interface/signature recovery on *fully stripped* Go binaries. This is
  the single highest-value type provider in the plan and is pure analysis-tier
  code (new `analyzers/gotypes/`).
- Swift `__swift5_*` reflection metadata (field descriptors + mangled type
  names) — **done** as `swifttypes` (`analyzers/swift/kuna_swifttypes.rs`):
  layouts from the static type metadata, field types through
  `kuna_swift::demangle_swift_type` with its symbolic references resolved.
- **Synthesized well-known shells** registered by each pack even with no
  debug info (`runtime.g`, `go.string`, `go.slice`, `go.iface`,
  `swift.refcounted`), so P5 recognizers can type a `(ptr,len)` pair loaded
  and stored together as a string/slice without metadata.

### W4 — ABI, calls, strings

**Model selection.** The golang cspecs are already vendored for x86-64/x86-32/
AArch64 and never chosen. Plan:
1. `SourceLang::Go` + option `goabi auto|off` composes the `:golang` compiler
   field in `language_id_for` (`loadimage_object.rs`), so the vendored
   `abi-internal` default, `abi0`, write-barrier and duff models load.
   Go ≤1.17 and `.abi0`-suffixed pclntab names select `abi0` **per symbol** —
   which requires closing a real gap the survey found: *there is no way to
   bind a prototype model to a symbol without a full signature*
   (`declared_proto_models` only applies alongside parked
   `PrototypePieces`). Fix: let `AnalysisOutput` carry `(addr, model_name)`
   facts and honor a model-only binding in `callee_proto_model` /
   `ActionDefaultParams`.
2. **Swift models added to cspecs** (**done**: `__swiftcall` is the platform
   base plus `swiftself` (x20 / r13) as an input resource section of its own;
   `__swiftcall_throws` also kills `swifterror` (x21 / r12)). Bound per symbol
   as `(addr, model)` facts — the model-only binding item 1 calls for, now
   honored by the function's own drive and by `ActionDefaultParams` — from the
   Swift metadata and the demangled symbols (`throws` ⇒ the throwing model),
   and per call where the caller zeroes the error register before it. The
   swifterror out-value is still an anonymous local after the call;
   representing it as a second joined return (below) is open.
3. **The g register**: `<global><register name="R14"/>` in the golang cspec
   plus the synthesized `runtime.g` shell type makes goroutine-local accesses
   render as `g->stackguard0` instead of an unexplained raw register — and
   the survey confirms `decode_global` already supports register globals.

**Multiple returns.** `ProtoStore` has exactly one output and ≥3 output
trials currently hit a stub and render as never-assigned locals. Two stages:
- *Stage 1 (this plan)*: multi-register returns come back as one **join-space
  value** (the golang cspec's `piece1..piece9` entries already describe this)
  carrying a `MultiReturn` facet; call sites destructure through the existing
  partial-piece machinery, and the emitters render `(a, b) = f(...)` (Rust) /
  `struct f_ret r = f(...)` (C). Requires extending
  `build_output_from_trials` past the 2-trial special cases
  (`kuna_callretpair`, `kuna_rustabi::build_call_output_pair` are the
  templates) — this also retires the `rustabi` pair-join as a special case of
  the general mechanism.
- *Stage 2 (explicitly deferred)*: true tuple return types in `FuncProto`.
  Not needed for readable output and touches every prototype consumer.

**Strings.** Generic **length-carried literal** support:
- Analysis tier: `Termination::Length` mode in `StringLiteralPass` — for Go,
  walk the rodata string-header tables and the `(ptr,len)` constant pairs the
  pclntab/funcname tables anchor, segmenting the blob; emit unterminated
  `StringFact{addr,len}`.
- Engine tier: a P5 rule that recognizes a `(const ptr, const len)` pair
  feeding a facet-typed string/slice and routes it through the **existing**
  bounded path `Funcdata::get_internal_string` → `BUILTIN_STRINGDATA`
  CALLOTHER — no NUL required, already prints as a literal. This single rule
  is the money shot for both Go (`string` headers) and Rust (`&str` fat
  pointers materialized as two constants at use sites).
- Swift small-string register packing is harder and deferred to the Swift
  pack's second round.

### W5 — Language packs

Each pack = detection arm (W1) + idiom rows (W2) + facet provider (W3) +
model/strings wiring (W4) + fixtures. One **master option per language**
(`golang`, `rustlang`, `swiftlang`: `auto|on|off`, default `auto` = act when
detected — the precedent is `rustabi off|auto|always`), with each
output-changing idiom additionally behind its own fine-grained option so a
single misfire is flippable per run. All pack options are **inert
off-language by construction** (the `rtti`/`objc`/`cortexmpriv` pattern), so
they are blanket-safe for the `aggressive` preset and the
`aggressive_carries_every_default_off_option` invariant.

**Go pack** (first: biggest C-assumption violations, best metadata story,
toolchain in the devcontainer):
- `morestack` prologue elision (`EntryGuardLoop` shape, pre-SSA). Without it
  every argument register gets an entry phi merging the original and
  reloaded-after-growth values — this one rule un-breaks parameter recovery
  for essentially every non-leaf Go function. *Measured (F3, go1.22 stripped
  hello-world, 1,543 fns): folds 623 of 1,144 guard-carrying functions and
  makes the whole-binary run 3.8% faster; the 521 misses are dominated by one
  cause — the grow arm physically follows a `runtime.panic*` call kuna does
  not know is no-return (`panicshift`, `panicunsafestringlen`, …), so the
  bogus fallthrough edge breaks the tight-cycle shape. The fix is the
  no-return coverage item below, not a looser matcher.*
- Write-barrier diamond (`DiamondStore`, post-SSA) → the plain store (the
  golang cspec's `gcwrite_*` fixups cover the call; the rule removes the
  flag check), `panicIndex`/`panicSlice`/`panicdottype` bounds diamonds
  (`GuardedColdCall` — no-return list already exists), `deferproc`/
  `deferreturn` annotation, `duffzero/duffcopy` → `memset/memcpy` builtins.
- **Go no-return coverage** (promoted by the F3 measurement): the vendored
  `GolangFunctionsThatDoNotReturn` predates the modern runtime panic family —
  `runtime.panicshift`, `runtime.panicunsafestringlen`,
  `runtime.panicunsafestringnilptr` and siblings are absent, so their call
  sites keep a bogus fallthrough edge that both corrupts the panic path's
  rendering and breaks the morestack fold's shape on every function whose
  grow arm follows one. One kuna-owned additive name list (gated like the
  vendored ones), measured against the full `runtime.panic*`/`runtime.goPanic*`
  surface of a current Go runtime.
- pclntab already supplies names + extents; add Go-version → default-ABI
  pick, `abi0` per-symbol binding, g-register global, string recovery,
  `gotypes` harvester (stretch goal within the pack).
- Mode interaction: real Go binaries are almost always >2 MiB, so `auto`
  resolves to `fast` and the Listing is off — but **pclntab *is* Go's
  function discovery**, runs at load, and is cheap. The pack keeps
  `gopclntab` on in every mode (it already defaults on), making Go the rare
  case where `fast` loses nothing on inventory.

**Rust pack** (second: much already exists — detection, demangling, rustabi,
panic no-return lists, securitycheck, DWARF variants):
- Wire the unread `VariantLayout` table into P5/P9 (variant-tagged union
  reads print the variant name; `Option<&T>` null-niche renders as the
  variant, not a magic constant) — highest-leverage single item, the parse
  work is already merged.
- `&str`/`&[T]` facets + the `(ptr,len)` literal rule; bounds/overflow panic
  diamonds generalized into W2 rows (today's securitycheck covers only seven
  helper names); `rustabi` pair-join subsumed by the Stage-1 multi-return.
- Demangle-driven naming is done; add generic-instantiation folding in
  display names (`Vec<T>::push` dedup) at the naming layer only.

**Swift pack** (third: most new infrastructure, smallest immediate corpus):
- Prereqs the others don't need: a Swift demangler (**done**: the pure-Rust
  `analyzers/demangle/kuna_swift.rs`, option `swiftdemangle`, verified against
  `swift-demangle`; every crate alternative builds Apple's C++ demangler, which
  `kuna-wasm` cannot carry), `__swift5_*` metadata walker (**done**:
  `analyzers/swift`, option `swiftmetadata` names accessors, vtable slots,
  witnesses and descriptors; its `swifttypes` arm lays out every fixed-layout
  struct and class from the static type metadata, resolving the field
  descriptors' symbolic references, and pins getters' `self` to the
  `swiftself` register), Swift detection, cspec `__swiftcall` models
  (**done**: option `swiftcall` binds them to methods, initializers,
  accessors and witnesses with an unlocked typed `self`, and gives a call
  that zeroes the error register the throwing model; still open: the
  indirect-result register (x8 / rax), up-to-four-register returns, the
  async context register (x22 / r14), and `self` at indirect vtable and
  witness calls that do not throw). Mach-O loading itself is already solid
  (fat binaries, stubs, issue-#666 underscore handling).
- Idioms: `swift_retain/release/autorelease` elision
  (`CallStraightLine` → `DeleteCall`; **default-on within the pack** — ARC
  traffic is overwhelming noise, and the per-idiom option plus `swiftarc off`
  preserves the faithful view), `swift_beginAccess/endAccess` exclusivity
  diamonds, dynamic-cast guards, existential/String facets.

## 3. Delivery roadmap

Foundation round (each its own PR, each green on all four gates):
1. **F1** `SourceLang` fact + loadtime `sourcelang` option + Swift detection +
   the 7-call-site consolidation. No output change by itself.
2. **F2** `AnalysisOutput` image-fact + model-binding streams
   (`(addr, model_name)`), honored by `callee_proto_model`.
3. **F3** `infra/idiom.rs` + the pre-SSA/post-SSA `ActionIdiomReduce` hosts,
   shipping with **one** witness row: Go `morestack` (option `gomorestack`,
   two-pass stage test: off = entry-phi soup, on = clean params).
4. **F4** `LangFacet` table + speller/cast consultation + the `(ptr,len)`
   literal rule (witness: Rust `&str` constant).

Then the packs, Go → Rust → Swift, one idiom/provider per PR per the standing
requirements (two-pass stage test, speed delta, spec chapter update, catalog
count bumps). The golang-cspec auto-select and the Stage-1 multi-return are
the two Go-pack PRs most likely to need a `[PROPOSAL]` round of their own —
both can move emitted C on non-Go binaries if gated wrong, so their options
start `auto` (= detected-language only) with explicit `off`.

## 4. Testing and measurement

- **Fixtures**: Go 1.18 is in the devcontainer and `go`/`rustc` exist on the
  dev host; Swift has no toolchain anywhere, so Swift fixtures are built once
  (devcontainer addition or upstream toolchain) and vendored as binaries like
  the existing Mach-O fixtures. Every idiom lands with a two-pass
  `tests/stages/` case; CLI-visible behavior (language field, literal
  rendering) lands as `tests/cli/` probes beside the existing
  `rust-symbol`/`structdefs` probes.
- **The benchmark blind spot is real**: decbench is C-only, so GED cannot
  referee these packs. Each pack needs a small language corpus
  (10–20 binaries per language, stripped + unstripped, O0/O2) checked against
  function-count, parameter-count and literal-recovery oracles — pclntab and
  DWARF make ground truth cheap for Go/Rust. A decbench language lane is the
  long-term answer but not a blocker.
- **Speed budget** (+5% standing requirement): the idiom matchers are
  per-function and shape-gated (first-block/callee-name prefilters before any
  walk); the costly items are load-time metadata parses (`gotypes`,
  `__swift5_*`), which must be measured on large Go binaries specifically —
  a 50 MB Go binary with hundreds of thousands of typelink entries is the
  stress case, and the parse must stay lazy or bounded. Detection itself is
  already paid for today (7 times over; W1 makes it cheaper).

## 5. Explicit non-goals (this round)

- True tuple returns in `FuncProto` (Stage 2 above).
- Goroutine/async machinery reconstruction (channels, select state machines,
  Swift async) — revisit once the packs stabilize; `select` lowering is the
  first candidate for a `Region`-window idiom shape.
- Full Rust `match`-tree reconstruction from switch+variant info (the facet
  naming gets 80% of the readability for 5% of the risk).
- A general subgraph-isomorphism matcher (the shape vocabulary is the
  deliberate alternative).
- Objective-C (already has its own `objc` analyzer plane).
