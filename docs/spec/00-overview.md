# 00 — Overview & machinery

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/substrate
  - decompiler/crates/kuna-decomp/src/p0_knowledge
  - decompiler/crates/kuna-decomp/src/infra
  - decompiler/crates/kuna-cli/src
  - decompiler/crates/kuna-console/src
  - decompiler/crates/kuna-ghidra/src
```

This chapter is the machinery every other chapter assumes: the two tiers and their
hand-off, the front-ends, the IR containers, the knowledge plane, the two
`Architecture` types, the pass scheduler, and the feedback edges that make the
pipeline non-linear. The algorithms themselves live in chapters 01–09; this is how
they are hosted, ordered, configured, and restarted.

## 0.1 The two tiers

kuna is two engines with one boundary. The **program-preparation tier**
(`kuna-analysis`, chapter 01) looks at the whole binary once — loader parse, symbol
and relocation markup, strings, DWARF, entry discovery, the Listing, the no-return
family — and produces *facts*. The **decompiler tier** (`kuna-decomp`, chapters
02–09) analyzes one function at a time and never scans the program; everything it
knows about the outside world it reads from the knowledge plane (§0.4) that the
first tier populated. The analysis tier, symmetrically, never touches the
per-function IR.

(kuna) The hand-off is a three-step *stash → flags → gated commit* protocol, and
the order is load-bearing:

1. **Stash at load.** `decompiler/crates/kuna-console/src/engine.rs
   (bootstrap_from_object)` opens the object, builds the engine, then runs every
   analysis pass read-only over the parsed image
   (`decompiler/crates/kuna-analysis/src/passes.rs (run_default_analyses_per_pass)`).
   Nothing is committed: the per-pass facts — function/data symbols, discovered
   entries, no-return marks, no-fall-through call sites
   (`decompiler/crates/kuna-analysis/src/pass.rs (AnalysisOutput)`) — are parked on
   the program keyed by pass id (`decompiler/crates/kuna-console/src/engine.rs
   (ConsoleProgram::pending_analysis)`).
2. **Flags.** The caller applies its `option` lines. Each analysis pass has an
   enable flag on the engine `Architecture` (`decompiler/crates/kuna-decomp/src/infra/architecture.rs
   (reset_defaults_internal)`, the `analysis_*` block), flippable per run.
3. **Gated commit at the read-symbols boundary.**
   `decompiler/crates/kuna-console/src/ifacedecomp.rs (IfcReadSymbols)` calls
   `decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::commit_pending_analysis)`,
   which drains the stash, drops the facts of any disabled pass
   (`decompiler/crates/kuna-console/src/engine.rs (analysis_pass_enabled)` — an
   unregistered id fails *open*, so a new pass runs by default unless it is
   output-changing and registers a default-off gate), merges the survivors in pass
   order, and installs them through
   `decompiler/crates/kuna-console/src/engine.rs (commit_analysis_output)`. Every
   commit arm is additive and idempotent against the loader symbols already
   installed *in the symbol table* (the `find_function` overlap check no-ops a
   duplicate). Idempotence does **not** extend to the flat name→address stream
   `decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::register_symbol)`
   maintains: that retains by NAME, so an entry the loader already named
   accumulates a second record whenever a pass supplies a different name for it —
   a debug-info name (DWARF/PDB/pclntab/objc), a FID rename, or the generated
   `sub_<addr>` placeholder for a rediscovered entry. Several names for one entry
   is therefore the normal state, and §0.2 defines how the whole-binary surfaces
   collapse it.

Two pass families cannot run at load at all and are deferred *into* the commit:
the Listing walk and its consumers (the call-graph no-return fixpoint, §1.6–§1.7;
`decompiler/crates/kuna-analysis/src/passes.rs (run_listing_consumers)`) and the
scalar-operand markup (`decompiler/crates/kuna-analysis/src/passes.rs
(run_operand_refs)`). Both decode through the engine's SLEIGH translator, whose
program load-image is only attached after the load-time pass list runs — so
`commit_pending_analysis` re-parses the stashed image bytes and runs them at the
boundary, when their gates are finally known.

The failure mode of this protocol is silent: an analysis option applied *after*
the read-symbols boundary is a no-op — the facts were already committed or
dropped, and the drained stash means a second `read symbols` re-commits nothing.
Every driver therefore emits option lines strictly between `load file` and
`read symbols` (`decompiler/crates/kuna-cli/src/decompile_all.rs (load_program)`,
`decompiler/crates/kuna-cli/src/decompile.rs (build_script)`).

**Parity isolation.** The XML `<binaryimage>` bootstrap the datatests use
(`decompiler/crates/kuna-console/src/engine.rs (bootstrap_program)`) never runs
the analysis tier: nothing is stashed, so the gated commit is structurally a
no-op and the datatest parity oracle (`docs/baseline.json`) cannot be perturbed
by any analysis change. Only the real-object path pays for — or benefits from —
tier one.

## 0.2 Front-ends and the decompile-all walk

Four front-ends drive one engine assembly:

- **The console** — `decomp_dbg`
  (`decompiler/crates/kuna-console/src/bin/decomp_dbg.rs`), the interactive
  command interpreter (`load file` / `read symbols` / `decompile` / `print C`),
  and the datatest harness `decomp_test_dbg`
  (`decompiler/crates/kuna-harness/src/bin/decomp_test_dbg.rs`), which drives the
  same bootstrap over the XML corpus. This is the parity surface: it never arms
  the watchdog and (on the XML path) never runs tier one.
- (kuna) **`kuna decompile`** (`decompiler/crates/kuna-cli/src/decompile.rs
  (build_script)`) — subprocess-per-function: it scripts a fresh `decomp_dbg` for
  each request, so every invocation re-parses the SLEIGH spec and re-runs the
  whole-binary analysis. It injects `option listing on` by default (unless the
  caller names `listing`), so the no-return analyses fire even on the
  single-function path.
- (kuna) **`kuna decompile-all` / `kuna functions`**
  (`decompiler/crates/kuna-cli/src/decompile_all.rs (run, decompile_all)`) — the
  whole-binary, machine-readable surface: load and analyze **once** in-process
  (`bootstrap_from_object` → options → `commit_pending_analysis`, i.e. the
  `load file` + `read symbols` seam inlined,
  `decompiler/crates/kuna-cli/src/decompile_all.rs (load_program)`), then loop
  `decompile_func_full_with_override_dyn` + `print_c` over every selected
  function. A failed function degrades to a per-function `error` record — the
  pipeline drive catches un-ported-seam panics, and the render/variable
  extraction is wrapped in its own `catch_unwind` so a printer invariant cannot
  discard the functions already decompiled. `kuna functions` is enumeration
  only; under concrete `reliable`/`fast` it keeps the Listing off, while an
  auto-selected `aggressive` preset intentionally enables the broader analysis
  even for inventory.

  The full callable-symbol inventory these surfaces share is
  `decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::function_entries_canonical)`,
  which yields **exactly one record per function entry address**, address-ordered.
  It exists because the raw symbol stream
  (`decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::function_entries)`)
  holds one record per NAME (§0.1), so without it a whole-binary run reports — and
  decompiles — the same function once per name it carries. Each record keeps the
  most informative name and carries the rest as aliases, ranked by
  `decompiler/crates/kuna-console/src/engine.rs (entry_name_rank)`: a real symbol
  outranks a synthesized dynamic-table name (`_INIT_<i>` / `_FINI_<i>` /
  `_DT_INIT` / `_DT_FINI`,
  `decompiler/crates/kuna-console/src/engine.rs (is_structural_entry_name)`), which
  outranks a generated placeholder
  (`decompiler/crates/kuna-console/src/engine.rs (is_generic_placeholder_name)`);
  ties prefer the unprefixed spelling over the underscore-prefixed one, then the
  shorter name, then lexicographic order, so the choice is total and independent of
  symbol-stream order. Name-keyed selection resolves aliases too
  (`decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::find_entry_by_name)`),
  so collapsing the records never makes a name stop selecting its function. On an
  ARM-family spec the grouping key folds away the Thumb mode bit (`vma & !1`, the
  same normalization
  `decompiler/crates/kuna-console/src/project.rs (build_asm)` applies to its
  labels), so an `entry` and its `entry|1` twin are one entry; address-keyed
  selection folds it too
  (`decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::find_entry_at)`),
  so `--addr` on an odd ARM address reaches the function rather than decoding
  mid-instruction. Both folds are gated to ARM, where an odd symbol address is
  never an instruction boundary; elsewhere an odd address is genuine and is left
  alone. `kuna functions` and wasm `list` report this complete canonical
  inventory, including callable import pointer slots. Unfiltered
  `decompile-all`, `decompile-project`, and wasm whole-binary runs derive their
  default target set through
  `decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::function_entries_executable)`:
  only entries inside a loader section carrying `CODE` are treated as function
  bodies. Import slots remain installed for call naming, prototypes, and
  unrestricted explicit address selection; name selection keeps its normal
  first matching canonical-entry behavior when a stub and slot share a name. A
  loader that publishes no section metadata retains the complete canonical set.
- **`kuna_ghidra`** (`decompiler/crates/kuna-ghidra/src/bin/kuna_ghidra.rs`) —
  the ghidra-mode process front-end: the stock Ghidra GUI spawns it as its
  decompiler core and talks the burst-framed stdin/stdout protocol
  (`decompiler/crates/kuna-ghidra/src/protocol.rs`). No `.sla` is loaded in this
  mode; every instruction's p-code, every byte, symbol, and type arrives by
  callback query (`decompiler/crates/kuna-ghidra/src/client.rs`).
  `registerProgram` builds a live engine `Architecture` over the query-backed
  translator (`decompiler/crates/kuna-ghidra/src/process.rs`,
  `decompiler/crates/kuna-ghidra/src/translate.rs (GhidraTranslate)`), and
  `decompileAt` drives the real `decompile_func`, its providers issuing nested
  queries on the still-open command response
  (`decompiler/crates/kuna-ghidra/src/provider.rs (SharedClient, GhidraLoadImage)`).
  A decompile failure degrades to the incomplete-function response shape so the
  GUI never desyncs; the lazy scope/type providers are the in-progress remainder
  (`decompiler/crates/kuna-ghidra/src/provider.rs`).

(kuna) **Surface defaults.** `decompile-all` injects two driver-level defaults
before the option pass (`decompiler/crates/kuna-cli/src/decompile_all.rs
(load_program)`): `option listing on` (DIV-15 — without the Listing the
default-on no-return propagation is a structural no-op, and a stripped binary's
unnamed exit wrapper swallows every following function into its caller), and
`option funcstart_patterns on` for non-x86-64 objects only (DIV-20 — the
prologue-pattern pass is the primary discovery source where the x86-64 scan
oracle does not apply). Both yield to an explicit caller option; neither touches
the engine default or the console/datatest surfaces.

(kuna) **The watchdog.** `decompile-all --max-fn-seconds N` (`0` disables) is
driver policy, not a phase-model option. An unfiltered whole-binary run in the
resolved `fast` preset defaults to 10 seconds per function. Native selected
runs and other presets retain 120 seconds, and an explicit value always wins.
The WASM front-end arms the same 10-second budget only for fast whole-binary
`decompile` and `project` commands; its other commands remain unbudgeted. The
driver sets the budget on the
architecture (`decompiler/crates/kuna-cli/src/decompile_all.rs
(decompile_all)`), which the drive arms as a wall-clock deadline covering
flow-follow, the jump-table sub-pipeline, and the action pipeline
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs
(decompile_func_full_with_override_dyn)`). The deadline is probed cooperatively —
at every group/sub-action boundary and repeat gate
(`decompiler/crates/kuna-decomp/src/infra/action.rs (ActionGroup::apply,
Action::perform, ActionRestartGroup::apply)`), every 1024 op-visits inside the
rule-pool loop (`decompiler/crates/kuna-decomp/src/infra/action.rs
(POOL_DEADLINE_STRIDE)`), and at the heritage loop
(`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs`). On expiry the
containers stop scheduling work and unwind; the driver converts that into the
function's `error` record and the batch continues. A function whose drive
completes before expiry is byte-identical with or without a budget, and the
console/parity paths never set one. It is not a hard wall around discovery,
unprobed SLEIGH work, C rendering and variable extraction, assembly/JSON
construction, total project time, or memory.

(kuna) **Load-time env bridges.** Four loader gates are consumed *inside* the
bootstrap — before any console `option` line can possibly run — so the option
surface alone cannot deliver them; each is bridged through a process environment
variable exported first (`decompiler/crates/kuna-cli/src/decompile_all.rs
(apply_loadtime_env)` in-process; the equivalent `Command::env` calls in
`decompiler/crates/kuna-cli/src/decompile.rs` for the subprocess):

| env var | option | read at |
|---|---|---|
| `KUNA_RELOC_OBJECTS` | `relocobjects` | ET_REL `.o` relocation resolution in the loader, `decompiler/crates/kuna-analysis/src/loadimage_object.rs (RELOC_OBJECTS_ENV)` |
| `KUNA_I386_PIE_PLT` | `i386_pie_plt` | i386 PIE PLT-stub decode, `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_i386_pie_plt.rs (I386_PIE_PLT_ENV)` |
| `KUNA_MACHO_SLICE` | `--slice` | Mach-O fat-binary slice peel, `decompiler/crates/kuna-console/src/engine.rs (select_macho_slice)` |
| `KUNA_MACHO_ARM64E` | `macho-arm64e` | arm64e spec selection, `decompiler/crates/kuna-analysis/src/loader/format/macho.rs (MACHO_ARM64E_ENV)` |

The matching `option` is still applied afterwards so the run's configuration
record is honest.

## 0.3 The IR substrate

The per-function IR is one container, `Funcdata`
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs (Funcdata)`), owning
slotmap arenas keyed by three generational id newtypes — `VarnodeId`, `OpId`,
`BlockId` (`decompiler/crates/kuna-decomp/src/substrate/context.rs`). Where the
C++ original links objects with raw pointers, kuna links them with arena keys: a
stale handle is a caught lookup failure, not a use-after-free. The arenas are the
varnode bank (`decompiler/crates/kuna-decomp/src/substrate/varnode.rs
(VarnodeBank)` — storage-sorted def/free/input trees), the op bank
(`decompiler/crates/kuna-decomp/src/substrate/op.rs (PcodeOpBank)` — a
`SeqNum`-keyed optree, whose stable key order is what lets a rule-pool cursor
survive op deletion, §0.6), and **two** block graphs
(`decompiler/crates/kuna-decomp/src/substrate/block.rs (BlockGraph)`): `bblocks`,
the CFG, and `sblocks`, the structuring tree — physically distinct, seeded as a
`BlockCopy` mirror of the CFG when structuring begins
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs (seed_sblocks_copy)`).

Every cross-arena mutation routes through `Funcdata` — Rust cannot hold two
`&mut` arenas through a method on one of them, so the op-in-block primitives the
C++ splits between `Funcdata` and `BlockBasic` are all `Funcdata` methods here
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs (bb_insert_op,
bb_remove_op)`).

**The impl map.** `Funcdata` is one struct whose `impl` blocks are split by the
phase that owns the mutation — the split is itself the documentation of which
phase mutates what (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs`,
module docs):

| impl block | owns |
|---|---|
| `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs` | construction, arenas, flags, `clear` |
| `decompiler/crates/kuna-decomp/src/substrate/funcdata_op.rs` | op creation/mutation primitives |
| `decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs` | varnode creation/lookup primitives |
| `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs` | CFG surgery + the jump-table drivers |
| `decompiler/crates/kuna-decomp/src/substrate/funcdata_encode.rs`, `decompiler/crates/kuna-decomp/src/substrate/funcdata_printraw.rs` | marshaling, raw printing |
| `decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs` | flow resolution (P2) |
| `decompiler/crates/kuna-decomp/src/p5_types/funcdata_union.rs` | union facet resolution (P5) |
| `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_facing.rs`, `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_merge.rs`, `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_spacebase.rs` | variable/merge/stack tiers (P6) |
| `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_casts.rs` | cast insertion hooks (P9) |

**Data types are shared IR, not per-function state.** The type factory
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (TypeFactoryImpl)`) is one
`Rc` owned by the engine and shared into every per-function handle
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs (build_arch_handle)`),
so a type interned while decompiling one function — or committed by a prototype
lock — is the same object every later function resolves. Chapter 05 owns the
lattice; here it only matters that `Datatype` handles cross function boundaries
and IR arenas do not.

## 0.4 The knowledge plane (P0)

P0 is everything that outlives a function's IR — the plane a restart re-reads
and an agent writes:

- **The symbol database** (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
  (Database, Scope)`): symbols in a namespace-scoped hierarchy, mapped to storage
  by range-tree `SymbolEntry`s, plus the boolean property map (read-only /
  volatile paint). Populated by the loader-symbol read and the analysis commit
  (§0.1); queried by name, address, containment, or property, walking the scope
  chain exactly as the upstream `stack*` helpers do.
- **The Override store** (`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
  (Override)`): per-function commands that override pipeline decisions — flow
  reclassification, direct-call redirects, prototype replacement, multistage
  jump-table requests, dead-code delays, forced gotos. Its defining property is
  that it **survives `Funcdata::clear`**
  (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs (clear)` resets the
  arenas and analysis state but not the override store): a mid-pipeline pass that
  discovers a decision too late writes the correction here and requests a
  restart, and the restarted run reads it back (§0.7).
- **The typed assertion facade** (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_assert.rs
  (validate_assertion, Dispatch)`) (kuna): `kassert <phase> <subphase> …`
  validates a request against the phase registry, computes the *reported* minimal
  rewind scope, logs it, and routes it to whichever battle-tested store already
  implements it (Override, proto locks, retype/rename, an option). It adds a
  model over the stores, not a new mechanism.
- **The option surface** (`decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs
  (OptionDatabase, KUNA_OPTION_NAMES)`): upstream options dispatch by registered
  element id through `OptionDatabase::set`; the kuna-added options are an
  allowlisted name set routed to
  `decompiler/crates/kuna-decomp/src/infra/architecture.rs (set_kuna_option)`,
  which writes the live flag the consuming pass reads. The machine-readable
  catalog rows — values, defaults, tier, symptoms, flip guidance — are generated
  into `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases.rs
  (SETTABLE_TABLE, emit_catalog_json)` from `decompiler/crates/kuna-decomp/phases.toml`
  by `decompiler/crates/kuna-decomp/build.rs`; the rendered catalog is
  [docs/options.md](../options.md) and this spec never duplicates its metadata.
- **Modes (option presets)** (kuna)
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs (MODE_TABLE, mode_overrides)`,
  applied by `decompiler/crates/kuna-decomp/src/infra/architecture.rs (apply_mode)`):
  a *mode* is a named, ordered list of `(option, value)` overrides layered over the
  shipped defaults — a P0 pipeline-variant preset over the option surface, **not** a
  `[[settable]]` row (it references existing option names, so it never touches the
  catalog or its count/tier gates). Three concrete presets ship:
  **`reliable`** (the shipped defaults, an empty-override alias),
  **`aggressive`** (every off-by-default recovery/analysis pass on, except
  `v850indirectbranch` which would mis-decode register-indirect calls off-V850),
  and **`fast`** (`listing`, `funcstart_patterns`, and `aif` off to avoid
  program-wide decode and speculative discovery). A fourth frontend policy,
  **`auto`**, resolves from the raw input length before the Architecture is
  built: `<500 KiB` selects `aggressive`, `500 KiB–<2 MiB` selects `reliable`,
  and `>=2 MiB` selects `fast`. File-based CLI commands use `auto` when
  `--mode` is omitted; the WASI/browser frontend uses the same Rust classifier.
  The interactive console accepts concrete `mode <name>` presets but cannot
  apply unresolved `auto`, because an Architecture has no input-file metadata.
  Overrides are applied *before* the user's `--option` (last-write, so an
  explicit `--option` still wins). Discover with `kuna modes`; full membership
  and exact byte boundaries are in [docs/modes.md](../modes.md).
- **The restart log** (kuna)
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs (RestartLog)`):
  owned by the engine `Architecture` so it survives function clears; every
  restart trigger records *why* (§0.7). Observability only.
- The phase registry itself
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases.rs (KunaPhase)`):
  P0–P9 with Band-B membership (`KunaPhase::in_band_b` — P3..P6), queryable at
  the console. The model behind it is [docs/phases.md](../phases.md).

**Effective defaults — the single narrative.** A knob's effective value is
layered, in order: (1) the engine default —
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (reset_defaults_internal)`
is the *single source*, and the `default` column of
`decompiler/crates/kuna-decomp/phases.toml` mirrors it (a hard-coded live-default assertion, `decompiler/crates/kuna-decomp/src/infra/architecture/tests.rs (kuna_anchor_flags_default_to_div_values)`, pins the engine defaults to the DIV values; the toml column mirrors them by convention); (2)
the file frontend's mode policy (`auto` when omitted) resolves to a concrete
preset, and load-time members plus explicit load-time options are exported
before bootstrap with last-write precedence; (3) per-program loader adjustments
made at bootstrap (e.g. `readonlypropagate` forced on for
MIPS so GOT-slot loads fold to import names,
`decompiler/crates/kuna-console/src/engine.rs (bootstrap_from_object)`);
(4) driver surface injections (`listing`, non-x86-64
`funcstart_patterns`/`aif`, §0.2) for options the concrete preset did not name;
(5) the concrete mode's runtime overrides followed by the user's
`--option`/`kassert` lines (which override the mode); and finally
(6) the per-function snapshot copy (§0.5), after which the value is frozen for that
function's drive.
Which defaults deliberately diverge from upstream, and the measurements behind
each flip, live in `docs/history.md`, not here.

## 0.5 The two Architecture types

There are two types named "architecture", and confusing them is the classic way
to ship a dead option.

The **engine god object**
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs (Architecture)`) owns
everything program-wide: the SLEIGH translator, the symbol database, the option
and action databases, the user-op and injection libraries, the type factory, the
printer, the restart log, and the whole bag of tuning values.

The **per-function snapshot**
(`decompiler/crates/kuna-decomp/src/substrate/context.rs (ArchContext)`) is the
`glb` every `Funcdata` carries (`ArchHandle`, an `Rc<ArchContext>`): the
IR-boundary slice of the god object that passes and rules may reach while the
pipeline holds `&mut Funcdata`. It shares the engine's single address-space
manager, type factory, string manager, and loader by `Rc`, and *copies* the
scalar configuration — every tuning value and (kuna) every rule gate — plus
read-only snapshots of the global symbol scope, callee prototypes, and tracked
registers.

The global-symbol snapshot
(`decompiler/crates/kuna-decomp/src/substrate/context.rs (GlobalQuery)`) groups
mapped entries by address-space index once when it is built. Grouping is stable:
the encounter order of entries within one space is unchanged, preserving
`findContainer`'s first-match behavior for equal-size overlaps and its
use-point selection. Property, naming, container, and callee lookups first
isolate the requested space, so register, stack, and other non-global varnodes
do not scan mappings from unrelated spaces.

The copy happens in exactly one place:
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (build_arch_handle)`,
called from `(Architecture::new_funcdata)` when a function's `Funcdata` is
built. Two consequences:

- **The flag-copy hazard** (kuna). A gate a rule reads through the per-function
  handle (`data.get_arch().<flag>`) exists twice — on the god object (where
  `option`/`kassert` writes it) and on `ArchContext` (where the rule reads it).
  If `build_arch_handle` does not copy it, the rule silently reads the
  `ArchContext` constructor default (`decompiler/crates/kuna-decomp/src/substrate/context.rs
  (ArchContext::new_shared)`) — deliberately `false` for the kuna rule gates, so
  hand-built fixtures keep gated rules inert — regardless of what the option
  surface wrote. The symptom is an option that parses, is confirmed, appears in
  the catalog — and changes nothing. Every new per-function-consumed flag must be threaded
  through `build_arch_handle`.
- **Snapshot timing.** The handle is built once per `Funcdata` and kept for that
  function's whole drive, including restarts (the restart re-flow clears and
  reuses the same `Funcdata`, `decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs
  (refollow_flow)`). Options must therefore be in effect before the function is
  built — which the console guarantees by rebuilding a fresh `Funcdata` on every
  `decompile` command.

## 0.6 The schedule

The pipeline's execution order is not the folder order. Every per-function run
executes a single declarative pass tree, `universal_sched`
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs (universal_sched)`,
a transcription of upstream `ActionDatabase::universalAction`). The tree is built
once per engine as `SchedNode` values (Action leaf / Pool of rules / Group /
RestartGroup), *filtered* by the root variant's enabled group list
(`decompiler/crates/kuna-decomp/src/infra/action.rs (build_default_groups,
ActionDatabase::set_current)`), and *materialized* into engine objects. Six root
variants exist — `decompile` (34 groups: everything), `jumptable` (12 groups:
only what a reduced flow analysis needs — the switch-recovery sub-decompilation
of §2.3 runs under it), `normalize`, `paramid`, `register`, `firstpass`. A
variant is a filter over the same tree, not a separate pipeline, which is what
makes reduced sub-queries cheap.

The shape, outermost-in: a RestartGroup wraps setup passes (constant-base,
default params, extrapop, prototype seeding, function linking), then
**fullloop**, a repeat-group that iterates until no member reports change. Inside
it, **mainloop** repeats the core sequence: unreachable-block and
varnode-property maintenance, (angr) lowered-switch installation, **heritage**
(SSA construction, §3.1), the prototype phalanx (param-double, direct-write,
active-param, return recovery, local restriction — §4), **dead-code
elimination**, spacebase and non-zero-mask analysis, **type inference** (§5),
varnode restructuring, and then **stackstall**, itself a repeat-group whose heart
is the `oppool1` rule pool — the opcode-indexed worklist of simplification rules
(141 registered in the default tree, plus per-architecture extras) that fires to
a local fixpoint — followed by lane division, CSE, shadow-var elimination, deindirection, and
stack-pointer flow. Mainloop's tail runs redundant-branch removal, block
structuring, constant-pointer recovery, the 5-rule `oppool2` (pointer-arithmetic
forms), determined-branch pruning, node joining, and conditional-execution/
conditional-constant analysis. Phases 3–6 therefore do not run as a sequence:
they co-evolve inside mainloop until mutual quiescence — the Band-B fixpoint
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases.rs
(KunaPhase::in_band_b)`). Fullloop's own tail (likely-trash, switch
normalization, (angr) lowered-switch detection and stack-guard stripping, return
splitting and the (angr) return-duplication family, unjustified params, active
return) runs between mainloop convergences.

Only after fullloop exits do the one-shot tails run: the 22-rule cleanup pool,
the merge phalanx (§6), prototype fixation, naming and casts (§9), final
structuring, and (angr) the goto-quality passes (§8.3). A pass that discovers it
has invalidated earlier work does not edit backwards; it requests a restart by
setting the restart-pending flag, having first persisted its lesson into the
knowledge plane (§0.7).

**The restart machinery, as actually implemented** (kuna): the in-tree
RestartGroup (`decompiler/crates/kuna-decomp/src/infra/action.rs
(ActionRestartGroup::apply)`, budget `max = 1`) cannot re-follow flow — the
action loop carries only the IR-boundary handle, not the SLEIGH translator — so
it hands every restart up (`ActionContext::reflow_requested`) to the outer drive,
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (run_pipeline)`,
which owns `&mut Architecture`: it clears the function (`Funcdata::clear` — the
Override store survives), re-follows flow, and re-performs the root, bounded at 8
cross-flow restarts (`MAX_REFLOW`); past the budget it keeps the last analyzed IR
rather than failing. Restarts are refused outright during jump-table recovery
(`is_jumptable_recovery_on`, same `apply`). The relocation is behavioral
plumbing, not semantics: trigger, clear, re-read-P0, re-run are the upstream
restart contract.

Two engine details are output-affecting and deliberately preserved
(`decompiler/crates/kuna-decomp/src/infra/action.rs`): `Action::perform` is a
resumable status machine (an action with `rule_repeatapply` loops until its
change count stops rising; `rule_onceperfunc` latches done), and
`ActionPool::process_op` walks each op's per-opcode rule list *resetting the walk
to index 0 whenever a rule changes the op's opcode* — rules observe each other's
effects mid-op, and the reset order is part of the observable output. The
materialized `decompile` tree's listing is byte-equal to the C++ oracle dump
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(UNPORTED_ALLOWLIST)` — empty).

Flow-follow itself runs *before* the tree (the upstream `followFlow` →
`startProcessing` order), bounded by the P0 flow options — decode-error policy
`error_toomanyinstructions` and a 100000-instruction ceiling by default
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`), applied at
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (follow_flow_on_fd)`.

## 0.7 Feedback edges

The pipeline is a fixpoint machine wearing a pipeline's clothes. Beyond the
in-tree repeat groups (§0.6), these are the edges where a *later* phase dirties
an *earlier* phase's artifact, what each persists, and where each lives in kuna.
(The mechanism taxonomy — local fixpoint, staged re-entry, restart-with-hints,
reduced sub-query, knowledge-store re-run — derives from the 2026-06 stage-model
study summarized in `docs/history.md`; every row below is re-verified against the Rust.)

| Edge | Mechanism | Trigger | Survives / persisted where | kuna anchor |
|---|---|---|---|---|
| rule pools → themselves | local fixpoint | any rule fires; opcode change rewinds the per-op rule walk | — | `decompiler/crates/kuna-decomp/src/infra/action.rs (ActionPool::process_op)` |
| P2 → P2, jump-table recovery | reduced sub-query | `BRANCHIND` with unrecovered targets mid flow-follow | recovered table → `jumpvec`; the cloned partial is discarded | `decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (run_jumptable_pipeline)`, driven from `decompiler/crates/kuna-decomp/src/p2_lift/flow.rs (generate_ops_with_jumptables)` |
| Band B → P3/P2, dead-code delay | restart + persisted hint | a free varnode reappears at an already-heritaged address after dead code was removed | `Override::insert_deadcode_delay` (+1) in P0 | `decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs (bump_deadcode_delay)`; suppressed during jump-table recovery (the `is_jumptable_recovery_on` guards at its call sites) |
| P4 → Band B, late prototype | restart + persisted hint | a resolved indirect call's prototype cannot be merged in place (`late_restriction` fails) | `Override::insert_indirect_override` — the re-flow rebuilds the CALLIND as a direct CALL | `decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (FuncCallSpecs::deindirect, FuncCallSpecs::force_set)` |
| (angr) P2 → P2, lowered switch | detect-then-restart, two halves | a comparison cascade recognized as a compiler-lowered switch after simplification | the recovered cascade record, in a store shared by both halves | detect in fullloop writes + requests restart, install in mainloop (before heritage) reads on the restarted run — `decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs (ActionLowerSwitchDetect, ActionLowerSwitchInstall)` |
| P5 → P2, determined branch | in-loop re-entry | constant folding decides a conditional branch, removing a CFG edge | the simplified ops themselves | `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs (ActionDeterminedBranch)`, inside mainloop |
| (kuna/angr) P7/P8 structuring fallback | degraded re-run | the region structurer cannot collapse the graph to a single root | nothing; `sblocks` is re-seeded clean | `decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs (ActionBlockStructure)` falls back to `CollapseStructure` after `decompiler/crates/kuna-decomp/src/p8_structure/region_structurer.rs (run_region_structurer)` declines |
| P0 → everything, the outer loop | knowledge-store re-run | an operator/agent writes an assertion (`option`, `kassert`, override) and re-decompiles | the entire P0 store | `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_assert.rs (Dispatch)`; the console rebuilds the IR per `decompile`, re-seeding stashed facts — `decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (decompile_func_full_with_override_dyn)` |

**Not implemented in kuna** (theory-only, kept for the record): the upstream
jump-table *size-mismatch* restart — `matchModel` finding the recovered model's
size differs from the flow-recovered address table would persist
`Override::insertMultistageJump` and restart. In kuna the mismatch keeps the
flow-recovered addresses and does not restart
(`decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs (JumpTable::match_model)`,
a documented stub); the Override store already carries the hint surface
(`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
(Override::insert_multistage_jump)`) with no live producer.

Mechanisms are mutually disabling by design: no restart, and no dead-code-delay
bump, fires inside the jump-table sub-decompilation — the sub-query must answer
its one question and be discarded, never mutate P0.

(kuna) Every restart trigger and suppressed trigger records its reason in the
engine-owned restart log
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs (RestartLog)`),
because a function that silently decompiles twice is otherwise invisible.

## 0.8 Reading order

The folder taxonomy is the *artifact* order, not the execution order. Source
under `decompiler/crates/kuna-decomp/src` is arranged as `substrate` (the IR
containers, §0.3), `infra` (scheduler, god object, drive — this chapter),
`p0_knowledge` (§0.4), and `p1_partition` … `p9_emit`, which map 1:1 onto
chapters 01–09 of this spec; the program-preparation tier is
`decompiler/crates/kuna-analysis/src` (chapter 01). Execution order is §0.6's
tree — when you need to know *when* a pass runs, read
`decompiler/crates/kuna-decomp/src/infra/universalaction.rs (universal_sched)`
and search for the pass's constructor, never the folder.

Conventions worth knowing before reading anything:

- **Tests ride in sibling directories**: a module `foo.rs` ends with
  `#[cfg(test)] mod tests;` and its tests live at `foo/tests.rs` (e.g.
  `decompiler/crates/kuna-decomp/src/infra/universalaction.rs` +
  `decompiler/crates/kuna-decomp/src/infra/universalaction/tests.rs`).
- **C++ citations in code comments** (`decompiler/cpp/<file>.cc`) are upstream
  Ghidra anchors at the pinned `GHIDRA_REV` (`docs/history.md`) — the tree kuna
  was ported from — not paths in this repository.
- **`Funcdata` methods are phase-owned**: find the owning phase through the impl
  map (§0.3) rather than grepping one giant file.
- Option metadata lives in the generated catalog
  ([docs/options.md](../options.md)); the phase model at a glance in
  [docs/phases.md](../phases.md); intentional default divergences, their
  measurements, and the original derivation study in `docs/history.md`.

Suggested order for a first full read: this chapter, then 01 → 02 → 03 (the
world up to SSA), then 04/05/06 as one unit (they converge together, §0.6), then
07 → 08 → 09.
