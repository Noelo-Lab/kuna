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
   installed *in the symbol table*: the discovered-entry arm's overlap check
   resolves **across scopes**
   (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
   (Database::find_function_across_scopes)`, the port of C++ `Scope::queryFunction`,
   which spans the scope tree), so a function already known under a *namespaced*
   name is recognized as present and no placeholder is installed over it. Scoping
   that check to the global scope alone was the DIV-59 defect: a demangled C++
   funcsym lives in its namespace scope (`std::terminate` is base `terminate` in
   scope `std`), the synthetic `sub_<addr>` the arm would name a rediscovered entry
   carries no `::` and therefore resolves to GLOBAL, so the probe missed the real
   symbol and installed a duplicate beside it — and since the cross-scope resolver
   searches global first, that duplicate then shadowed the real name for
   `FlowInfo::queryCall`, rendering `sub_<addr>` at every C++ call site on any
   surface that enables a discovery pass. Idempotence does **not** extend to the
   flat name→address stream
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

The commit is also not transactional. Its arms mutate the architecture in place
and in order, so an arm that fails leaves the earlier ones applied and abandons
every later one — library and DWARF prototypes, processor-context paints,
tracked register values, call-fixups, stack locals, source-line comments — and
the drained stash makes it unretryable. A partial commit is therefore a *failed*
load, not a degraded one, and every surface says so and stops: the in-process
drivers propagate the error as `read symbols (analysis commit) failed: <reason>`,
and the subprocess driver recovers the same reason from the console transcript
and reports it identically (§0.2). Nothing about a partially-committed program is
visible in the C it would render, which is what makes reporting it — rather than
printing that C — the contract.

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
  single-function path. Its two selection forms resolve the printed function name
  the same way: `load function <name>` carries the requested name through, and
  `--addr <vma>` (`decompiler/crates/kuna-console/src/ifacedecomp.rs
  (IfcAddrrangeLoad)`) first asks the symbol table what is installed at that
  address — across scopes, so a demangled C++ entry reports its qualified
  `Class::method` form — and only falls back to the generic
  `Architecture::name_function` (`sub_<addr>`) for a genuinely unknown address. An
  explicit `load addr <vma> <name>` still wins over both. Before DIV-59 the address
  form skipped the lookup entirely, so an addressed function on an **unstripped**
  binary printed a `sub_<addr>` header that the by-name form printed correctly.
  Because the engine runs in another process, this surface holds no error object:
  it recovers what failed from the transcript
  (`decompiler/crates/kuna-cli/src/decompile.rs (check_errors)`), and does so to
  the same wording the in-process surfaces produce, so one failure reads the same
  from all four commands (DIV-90). A failed `load file` prints the escaped error
  and then `Could not create architecture`, so the reason is the line before the
  trigger; the generic `(unsupported/!recognized binary)` wording is only the
  fallback for a transcript that carried no reason at all. A failed analysis
  commit is an `Execution error:` the console prints while **keeping the session
  alive**, so `print C` still renders C and the command's exit code is the only
  thing left to distinguish a program whose debug facts were dropped from a
  binary that never had any: each console diagnostic is attributed to the command
  echo above it, and one belonging to `read symbols` is reported with the
  in-process surfaces' message and exit code rather than the C.
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
  only, but it enumerates what `decompile-all` would decompile: both surfaces
  take the same driver discovery defaults, so on a stripped non-x86-64 binary
  the inventory is built with `funcstart_patterns`, `aif`, and the Listing that
  gates them, exactly as a whole-binary run is (DIV-68). What the inventory
  surface does not take is the Listing on an architecture where the Listing
  discovers nothing: on x86-64 it is the decompiling surfaces' default alone,
  because there it changes no-return facts and therefore emitted C, never the
  entry set. An explicit `--option`, and any preset that names one of these
  options, still wins on either surface.

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
  Explicit selection of an entry with **no mapped bytes** — an import slot, or a
  relocatable object's undefined symbol bound to a synthetic extern-area address
  so that calls to it render by name — answers with the entry's nature rather
  than with the lifter's byte-load failure: the shared decompile step probes
  `decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::entry_bytes_mapped)`
  first and emits a one-line external-symbol body. The probe is a one-byte read
  rather than a section-flag test, so an address that is mapped but outside any
  `CODE` section (packed code in `.data`, a hand-picked `--addr`) decompiles
  exactly as before. The browser inventory sorts the same entries into its
  imports-and-thunks group off that predicate
  (`decompiler/crates/kuna-console/src/classify.rs`), which a name test cannot
  do: loader names are demangled (`CellClass::Cell_Coord`) and symbol-table names
  are not. That classifier sits beside the shared decompile-project core rather
  than in either front-end because two surfaces answer "what kind of function is
  this" — the browser inventory and the `kuna decompile-graph` document (§9.7) —
  and they must not answer it differently.

  (kuna) **The listing surfaces read the same section flags to choose a view.**
  `kuna disassemble` and its `kuna read` spelling
  (`decompiler/crates/kuna-cli/src/disassemble.rs (render)`) are one command with
  two renderings of one walk: decoded instructions, or the bytes as a hexdump
  with an ASCII gutter and, under `--json`, the span as one contiguous hex
  string. `--as code|data|auto` selects, and `auto` — the default for
  `disassemble`, where `read` defaults to `data` — decides from the loader's own
  classification of the section holding the start address
  (`decompiler/crates/kuna-cli/src/disassemble.rs (decide_view)`): a section
  carrying `DATA` without `CODE`
  (`decompiler/crates/kuna-sleigh/src/loadimage.rs (section_flags)`) holds bytes,
  so it is shown as bytes. Two exceptions keep the inference honest. A target that
  resolved to a discovered function entry is code wherever it was linked, so it is
  never reclassified; and an address in no section the loader published — the XML
  `<binaryimage>` corpus, a raw blob — is silence rather than evidence and keeps
  the instruction listing. Only an inferred flip is explained, on stderr and in
  the JSON `notes`, so `--json` stdout stays one document; an explicit `--as` is
  the caller's decision and is not narrated back at them. This exists because the
  instruction view alone is a wrong answer that reads like a right one: `.rdata`
  and `__TEXT,__const` decode perfectly well into `ADD`/`OR` rows that describe
  nothing in the program, which is what sent two RE-loop testers to `xxd` and
  `objdump -s` (`docs/re-needs/cli-mode-read-raw.md`).
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
  GUI never desyncs.

  (kuna) **The Phase-3 lazy providers** make ghidra-mode consume the program
  facts the host already has. The wire has no enumerate-the-program query, so
  eager pre-population (what the console's analysis commit does) is impossible;
  kuna instead ports upstream's lazy `ScopeGhidra` model to the seams its own
  pipeline reads. `RemoteScope`
  (`decompiler/crates/kuna-decomp/src/infra/remote_provider.rs`) is installed on
  the `Architecture` at registerProgram (after the cspec `<global>` ranges and
  pspec property paints are in — the `lockDefaultProperties` point) and rides
  every per-function `ArchContext`; when present, every global-scope read
  (`decompiler/crates/kuna-decomp/src/substrate/context.rs
  (ArchContext::effective_global_query)`: properties, global names/types,
  containers, callee prototypes, deindirect resolution) and the flow
  environment's callee name / no-return queries
  (`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (ArchFlowEnv)`)
  resolve through it. A miss inside a cspec `<global>`-ranged space fires ONE
  getMappedSymbols query; the `<doc><mapsym>` answer decodes through the
  symbol-family decoder (`decode_mapped_answer`: `<symbol>`, `<function>` with
  its `<prototype>` and `<localdb>` category-0 parameters, `<functionshell>`,
  `<labelsym>`, `<externrefsymbol>`, `<equatesymbol>`, `<facetsymbol>`) into
  `GlobalEntry` records merged over the Database snapshot; a `<hole>` answer
  lands in a negative-cache range list (never re-queried) and its
  readonly/volatile bits paint a local property map that `clear()` rolls back
  to the locked default. Namespace ids resolve once through getNamespacePath.
  A decoded LOCKED signature (typelocked params or a locked-void input) parks a
  `TypeCode` prototype for `query_callee_proto` plus `PrototypePieces` for
  `ActionDefaultParams` and — for the current function — seeds the fresh
  `Funcdata`'s prototype at `decompileAt`, whose display name now comes from
  the mapsym answer (the Java `Function.getName()` echo), getCodeLabel demoted
  to fallback. The decoded `<function noreturn>` fact truncates flow at
  no-return call sites, exactly as the console's analysis facts do.
  Types: registerProgram decodes the wire `<coretypes>` (so kuna's core-type
  ids equal the host's), and a `find_by_id` miss fetches the definition with
  getDataType (`decompiler/crates/kuna-decomp/src/substrate/dtype.rs
  (decode_type, decode_core_types, find_by_id_or_remote)`); composites intern
  an incomplete stub before their fields decode so a self-referential struct
  cannot re-query forever. Comments fill once per flush cycle from getComments,
  filtered by the printer's comment settings (an empty filter issues no query).
  Registers resolve through a query-backed lookup installed on the ghidra-mode
  space manager (`decompiler/crates/kuna-ghidra/src/translate.rs
  (GhidraRegisterLookup)`) — the mirror of the Sleigh's own installed lookup,
  without which the naming pass misclassifies every register-storage high as
  global data and the output leaks raw `EAX`-style tokens. The pspec
  `<tracked_set>` (e.g. x86-64's `DF = 0`) decodes into the engine trackbase in
  ghidra mode too (`decompiler/crates/kuna-decomp/src/infra/architecture.rs
  (decode_ghidra_tracked_sets)`), resolving register names through the
  query-backed translator, so `ActionConstbase` plants the direction seed.
  Because that lookup is a host query, an undefined name is not a local miss:
  Ghidra's callback throws `No Register Defined`, which the host logs as an
  `Unexpected Exception` with a stack trace before the exception frame ever
  reaches kuna — recoverable on the wire, but visible to the user and
  unsuppressible from this side. So a *speculative* by-name lookup — a pass
  asking "does this language happen to have register X?" rather than resolving
  a name the host itself supplied — must go through the probe seam
  (`decompiler/crates/kuna-base/src/space.rs (RegisterLookup)`'s
  `probe_register` and `decompiler/crates/kuna-decomp/src/infra/engine_translate.rs
  (EngineTranslate)`'s `probe_register_varnode`) instead of the exact lookup.
  Both default to the exact lookup's `Ok`-to-`Some`, so the standalone Sleigh
  path is unchanged; the ghidra translator overrides them to answer from the
  `nm2addr` cache alone and issue no query. A `None` therefore means "not
  resolvable here", never "this language has no such register" — which is why
  only speculative tests may consult it. The x86 direction-flag assertion
  (chapter 04) is the case this shapes: its `DF` probe still resolves in ghidra
  mode because the pspec `<tracked_set>` sweep above runs first and caches `DF`,
  and every stock x86 pspec carries that set.
  The same absence of a local `.sla` leaves every p-code INJECTION payload
  without a compiled template. Registration still happens — the cspec
  `<callfixup>`/`<callotherfixup>` names, their `incidentalcopy`/`paramshift`
  flags and their parameter lists all decode as usual, and the passes that read
  that metadata are unaffected — but the snippet bodies are never compiled, so
  the two template consumers
  (`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (emit_inject)`)
  fall through to a second translator seam
  (`decompiler/crates/kuna-decomp/src/infra/engine_translate.rs
  (EngineTranslate)`'s `fetch_inject_pcode`). The ghidra translator answers it
  with a getPcodeInject query carrying the live injection context — base
  address, call address, and the sized input/output operand lists, with the
  follow-on address deliberately omitted because the host re-derives it — and
  streams the response straight into the emitter. What comes back is p-code the
  host has ALREADY LIFTED against that context: the ops are stamped with that
  call site's address and bound to that site's storage, so it is not a reusable
  template and must never be cached — two queries for the same payload name
  legitimately differ. A host exception on the query becomes a low-level error
  naming the payload rather than a passed Java exception, so a payload the host
  cannot supply costs that one function and not the whole command. On the
  standalone path the template always exists and the seam is unreachable. The
  reach is wide: `ARM.cspec` and all nine vendored MIPS cspecs declare a
  `setISAMode` `<callotherfixup>` that every interworking branch raises, so
  without the fetch most functions of both architectures fail outright in
  ghidra mode.
  External references resolve through the upstream two-step
  (`ScopeGhidra::resolveExternalRefFunction`): the `<externrefsymbol>` answer
  keeps its resolve address, getExternalRef fires at the POINTER address, the
  returned function materializes at its own entry, and the pointer symbol
  types as pointer-to-code.  A function answer's RAW name and its `label` stay
  SPLIT (the upstream `Funcdata` name/displayName pair): the raw name is the
  Funcdata identity `HighFunction.decode`'s name echo compares against, the
  label only ever prints (`Funcdata::set_display_name`).  The host's
  per-address tracked registers arrive for real: decompileAt issues
  getTrackedRegisters at the entry (`RemoteScope::tracked_at`, cached until
  flush) and merges the answer OVER the pspec `<tracked_set>` defaults —
  wire values win per register.  A wire/decoder failure inside a lazy query
  negative-caches the address as a one-byte hole for the flush epoch and
  surfaces ONE "Warning:"-prefixed 16/17 line (`RemoteScope::drain_warnings`)
  instead of re-querying unboundedly.  setOptions follows the upstream
  reset-then-apply contract (`Architecture::reset_wire_defaults` + the DIV-77
  preset layer before every decode) because Java delta-encodes the list.
  ghidra-mode also prints a `Kuna v…` plate comment (the release
  `KUNA_VERSION` bake, `kuna_banner_text`) at the top of every function —
  cache-only, HEADER-typed, rendered by the printer's plate arm
  (`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs
  (emit_comment_func_header)`), which also renders the host's PLATE comments;
  the standalone pipeline never inserts HEADER comments, so that arm is inert
  there.  `flushNative` clears it all in the upstream order
  (`Architecture::flush_remote_caches`): symbol cache + property rollback +
  the tracked cache, non-core types (`TypeFactoryImpl::clear_noncore`),
  comments, decoded strings.
  `setOptions` decodes the `<optionslist>` for real through
  `decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs (decode_lenient)`
  — every known option applies, unknown elements are skipped whole with a
  "Warning:"-prefixed 16/17 line (DIV-76), and the command always answers `t`.
  registerProgram also applies the CLI `aggressive` ENGINE-TIER preset (the
  GUI has no `--mode` surface) and flips address-derived fallback naming to the
  Ghidra GUI convention `FUN_`/`DAT_`/`LAB_` (DIV-77) via
  `Architecture::kuna_name_style` — kuna's angr-style local naming stays on.
  None of this touches the standalone path: no provider installed means every
  seam takes its frozen-snapshot branch, byte-identically.

  (kuna) **The Phase-4 full response encode** makes the `decompileAt` answer
  carry everything the native GUI features consume, in the upstream child
  order (`Funcdata::encode`,
  `decompiler/crates/kuna-decomp/src/substrate/funcdata_encode.rs`): the base
  `<addr>`, the `<localdb>` symbol scope, the `<ast>` (savetree), the
  `<highlist>` (savetree + high-level on), the `<jumptablelist>`, and the
  `<prototype>`.  `<localdb>` is `ScopeLocal::encode`
  (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs`) over the
  function's private symbol database
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
  (Database::encode_scope, Symbol::encode_header, SymbolEntry::encode)`):
  every `<symbol>` carries its NONZERO id (internal `SYMBOL_ID_BASE`-range for
  kuna-invented symbols; Java's `HighSymbol.decodeHeader` throws on 0), every
  `<mapsym>` at least one storage entry (`<addr>`/`<hash>` + its uselimit
  `<rangelist>`), parameters their `cat=0` + slot `index` + exact storage (the
  Java rename path re-commits the whole signature when these disagree with the
  database), and the `<scope>` opens positionally with `<parent>` +
  `<rangelist>` because `LocalSymbolMap.decodeScope` skips both blind.
  Because kuna's naming pass binds plain strings (`kuna_name`) instead of the
  C++ `ActionNameVars::linkSymbols` Symbol objects, two mechanisms supply the
  ids the wire needs. First, the naming pass RECORDS the bind it actually made
  (`HighVariable::kuna_link_symbol`,
  `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs`) when
  a high resolves to a covering localmap entry — and, for a `&symbol`
  REFERENCE, the identity of the Symbol referred to
  (`HighVariable::kuna_ref_symbol`, set by `Funcdata::link_symbol_reference`,
  the port of C++ `Varnode::setSymbolReference` →
  `HighVariable::setSymbolReference`). That second record is what a stack
  aggregate reached ONLY through `&sym` — a `char v [16]` passed to `memcmp`,
  whose entire HighVariable is the constant `PTRSUB` offset operand and which
  therefore owns no storage to re-derive a Symbol from — is declared off; it is
  read for the declaration only, because such a high encodes
  `class="constant"`, where Java's `HighConstant.decode` does nothing with a
  mapped local symref. Second, an encode-time link
  pass (`Funcdata::kuna_link_high_symbols`) gives every REMAINING named high a
  **wire-only symbol**
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs (WireSymbol)`):
  a mapped one at its storage when nothing covers it, or — when a Symbol does
  cover the storage yet the naming pass declined the bind as a CONFLICT (the
  narrower addr-tied return over a wider scalar parameter; the float8 lane over
  a float4 param) — a data-flow-HASHED one, upstream's `buildDynamicSymbol`
  answer. The encode never re-derives a container binding itself: doing so
  would hand a conflict-separated high the parameter's id, and a rename from
  that variable's token would rename the parameter. Wire symbols are encoded
  into `<localdb>` and referenced by `<high symref>`, but never enter the
  analysis scope — which is what lets the pass run BEFORE the markup is
  printed (so `<vardecl symref>` carries the same ids) without changing a byte
  of the emitted C. The `<highlist>` (`Funcdata::encode_high`, the
  `HighVariable::encode` port: `repref` = name-representative create-index,
  the five-way `class` rule, `symref` + partial `offset`, the type reference,
  one instance `<addr ref>` per member) therefore points only at ids the
  just-encoded `<localdb>` resolves — a symbol the encode skipped defensively
  is withheld from `symref` too (`Database::encodable_symbol_ids`, and
  `WireSymbol::is_encodable` for the wire ones: a 0-sized data-type at MAPPED
  storage is the `MappedEntry.decode` throw, so such a high takes the hashed
  shape instead), because an orphan reference is the Java hard-throw the skip
  exists to avoid.  The markup's `<vardecl symref>` passes the SAME filter and
  falls back to the create index when it fails — an unresolvable declaration
  reference is not a throw (`ClangVariableDecl.decode` logs and returns) but it
  is a dead rename on that line, which is the whole point of the attribute.
  Globals echo the REAL host database id delivered by
  getMappedSymbols (`GlobalEntry::symbol_id`,
  `decompiler/crates/kuna-decomp/src/substrate/context.rs`) and NEVER a
  fabricated one — an unknown id omits `symref` (Java warns and falls back to
  address-keyed rename) rather than silently renaming the wrong symbol.  Type
  references marshal through the `Datatype::encode_ref` port (chapter
  [05](05-types.md)); the prototype through `FuncProto::encode` (chapter
  [04](04-calls-and-prototypes.md)).  The `<jumptablelist>` re-uses the ported
  `JumpTable::encode` and is emitted INDEPENDENTLY of savetree — the switch
  analyzer asks `noc`+`notree`+`jumpload` and consumes only this list; the
  session's jumpload toggle reaches recovery as the upstream
  `FlowInfo::record_jumploads` flowoptions bit, applied per-decompile in
  `decompiler/crates/kuna-ghidra/src/process.rs` so the setOptions baseline
  reset can never strand it.  Under action `paramid` with parammeasures on,
  the doc contains ONLY `<parammeasures>`
  (`decompiler/crates/kuna-decomp/src/infra/paramid.rs
  (ParamIDAnalysis::encode)`, the `<rank>` child always on — Java throws
  without it); otherwise an optional `<parammeasures>` precedes the function
  pair.  The markup `<function>` is rendered BEFORE `fd.encode` runs (the
  link pass must not perturb the printed C) and spliced after the syntax tree,
  keeping the upstream document order.
  The rename/retype PERSISTENCE loop closes the circle: a GUI edit is a DB
  write (`HighFunctionDBUtil.updateDBVariable`) followed by an event-driven
  re-decompile whose getMappedSymbols answer now carries the edited local in
  the function's `<localdb>` (Java `LocalSymbolMap.grabFromFunction`).  kuna
  decodes those non-parameter locals
  (`decompiler/crates/kuna-decomp/src/infra/remote_provider.rs
  (RemoteLocalVar)`) and seeds them into the fresh `Funcdata` along FOUR
  channels, chosen by the two bits Java sets — the storage class and the
  typelock:
  a mapped, TYPELOCKED local (a retype — Java sends `typelock=false` only for
  `Undefined` types) seeds as a real mapped/usepoint symbol
  (`Funcdata::seed_mapped_symbols` / `Funcdata::seed_usepoint_symbols`,
  surviving restructure's typelock-keep rule); a mapped, namelocked-only local
  (a plain rename) — which C++ itself never keeps as a Symbol — stages as a
  NAME RECOMMENDATION (`Architecture::kuna_pending_name_recs` →
  `Funcdata::seed_name_recommendations`), the `ScopeLocal::nameRecommend`
  mechanism of chapter [06](06-variables-and-merge.md) §6.4; and the same two
  cases in DYNAMIC (`<hash>`) storage — the class Java writes for every
  variable that `requiresDynamicStorage`, i.e. unique-space representatives and
  `splitOutMergeGroup` products — seed as a dynamic Symbol
  (`Funcdata::seed_dynamic_symbols`) or a DYNAMIC name recommendation
  (`Architecture::kuna_pending_dyn_recs` →
  `Funcdata::seed_dynamic_recommendations`, applied through
  `DynamicHash::find_varnode` by
  `Funcdata::kuna_apply_dynamic_recommendations`).  Dropping the hash-storage
  half would silently revert renames of exactly the register/temporary
  variables users rename most.  The host's declared prototype MODEL and its
  EXACT committed parameter storage ride along too
  (`Architecture::kuna_pending_proto_model`,
  `Funcdata::apply_locked_prototype_with_model`, and the decoded cat-0
  storage threaded into `Funcdata::apply_mapped_params`, whose slots are
  counted in the SAME compacted basis `RemoteProto::to_pieces` builds).  The
  storage echo is the load-bearing half: Java's `checkFullCommit` compares the
  parameter COUNT, each `categoryIndex`, and each storage — never the model
  name — so a kuna-rederived storage or a slot skew force-rewrites the user's
  signature on the next rename.  The model rides along because the storage kuna
  would otherwise derive comes from it, not because Java inspects it.  Those
  echoed pieces carry the `ParameterPieces` lock bits (`TYPELOCK|NAMELOCK`),
  never the same-named `varnode_flags` ones — the two namespaces share no bit,
  and `apply_mapped_params` re-`setParam`s each slot WHOLESALE after the
  prototype channel has already locked it, so unlocked pieces here silently
  unlock the whole signature (`FuncProto::isInputLocked` reads slot 0's
  typelock) and the host's declared types and names get re-derived instead of
  applied.

(kuna) **Surfacing a failed function.** A per-function pipeline abort is
*recoverable*: the drive catches the unwind and returns the reason as an error
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (panic_message)`
recovers the panic payload's text — it must take the `catch_unwind` payload by
value, since a `&Box<dyn Any + Send>` downcasts as the box and loses the
message). Each front-end then decides how to report it, and every one of them
must make the failure observable:

- `decompile-all` / `decompile-project` / wasm record it as the function's
  `error` field and continue the batch (above).
- The console keeps the session alive — it prints `Skipping <fn>: <reason>` and
  returns success, so a datatest's `<stringmatch>` rules still evaluate rather
  than the whole file erroring
  (`decompiler/crates/kuna-console/src/ifacedecomp.rs`, the `IfcDecompile`
  error arm). Because the *previous*, un-decompiled `Funcdata` survives, a
  following `print C` renders a shell with no structured blocks; the arm
  therefore stamps the reason onto that `Funcdata`
  (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
  (Funcdata::set_kuna_pipeline_failure)`) so the emitted comment names the
  abort instead of blaming structuring (chapter
  [09](09-emission.md) §9.2).
- `kuna decompile` recognizes that notice in the subprocess transcript
  (`decompiler/crates/kuna-cli/src/decompile.rs (find_pipeline_failure)`) and
  reports it: the reason plus the forwarded `decomp_dbg` stderr on its own
  stderr, and **exit 1** — the shell still goes to stdout. Without this the
  command exits 0 with a plausible-looking empty function, because the rendered
  shell is not empty (DIV-45; the contract is `docs/cli.md`).

(kuna) **The stdout boundary.** Every CLI command renders its output and hands it
to `decompiler/crates/kuna-cli/src/output.rs (emit)` rather than calling
`println!`, which panics — exit 101, with a Rust panic trace on stderr — as soon
as a downstream reader closes the pipe, because Rust `SIG_IGN`s SIGPIPE and the
print macros unwrap the resulting `EPIPE`. A closed pipe is a normal terminal
condition, so the boundary is fallible and the failure is folded into the status
the command already reached (`output.rs (status_after)`): `BrokenPipe` keeps that
status and says nothing, any other write error is reported and forces 1.

Keeping the status is the load-bearing half. The write failed because nobody was
reading, which is orthogonal to whether the work succeeded, and stderr is still
open — so collapsing it to 0 would convert a false red into a false green:
`kuna test | head` would report a REGRESSED parity run as passing, and the DIV-45
contract above would hold only while someone was listening. `kuna specs` is the
one command whose stdout is a child's (`slacomp`): it streams that pipe through
the same boundary and leaves stderr inherited, because slacomp's diagnostics are
on stderr while the `Compiling <spec>:` line that attributes them is on stdout,
and capturing both would print every warning of a run ahead of every progress
line (DIV-89).

(kuna) **The console's filename grammar.** `kuna decompile` is the one front-end
that reaches the engine through a console *script* rather than an in-process
call: it writes `load file <path>` / `openfile write <path>` into `decomp_dbg`'s
stdin (`decompiler/crates/kuna-cli/src/decompile.rs (build_script)`), where the
other three read the image with `bootstrap_from_object` and never tokenize the
path at all. Upstream reads every path with `s >> filename`, a pure whitespace
scan, so a path containing a space arrived as two arguments: `load file` took the
head as a BFD target and loaded the tail, and `openfile write` truncated the
redirect at the split, writing the C to a file named after the first component.
The four commands that take a path — `load file`, `openfile write`, `openfile
append`, `parse file` — now read it with
`decompiler/crates/kuna-console/src/interface.rs (CommandStream::read_filename)`,
which accepts an optional double-quoted argument (`\"` and `\\` are escapes
inside quotes; any other backslash is literal, so a Windows path survives either
spelling) and is byte-identical to `read_token` for unquoted input, so the
vendored corpus and every script written before quoting existed parse exactly as
before. The two producers — `decompile.rs (console_path)` and its mirror in
`scripts/decompile.py` — quote only a path that needs it, which keeps the emitted
script byte-identical for every path that works today, including for an older
`decomp_dbg` reached through `--decomp-dbg` (DIV-100).

The redirect's own write is fallible for the same reason. `decomp_dbg` re-syncs
the open redirect after every command
(`decompiler/crates/kuna-console/src/bin/decomp_dbg.rs (sync_redirect_file)`),
and the open both creates and TRUNCATES its target, so discarding the error is
how a mis-parsed path became silent data loss. A target that cannot be opened or
written is now reported on stderr, once per target — the CLI forwards that into
its failure report, so a write that did not happen is never mistaken for a
decompiler that produced nothing.

(kuna) **One decompile step, two surfaces.** Every front-end turns a function
into a `Funcdata` through the same driver-tier step
(`decompiler/crates/kuna-console/src/decompile_step.rs (decompile_one)`), which
wraps the engine drive
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs
(decompile_func_full_with_override_dyn)`) in the parts of the per-function
contract that are policy rather than pipeline. Today that is the format-string
varargs-typing loop: when the run enables `formatstring`, the step decompiles
once, reads the constant format strings at the printf/scanf-family call sites it
finds, installs the derived per-call-site prototype overrides, and decompiles a
second time so the variadic arguments render typed (chapter
[01](01-program-prep.md) §1.4); reading those constants also needs read-only
propagation, so the step enables it for the duration and restores the prior
value. The caller supplies only the facts it has — the console its `map addr` /
`parse line` / `override` state, the whole-binary loop the function's DWARF
locals and the no-return flow prunes — through one seed struct, and both the
console `decompile` command
(`decompiler/crates/kuna-console/src/ifacedecomp.rs`, `IfcDecompile`) and the
whole-binary loop
(`decompiler/crates/kuna-console/src/project.rs (decompile_targets)`, behind
`decompile-all` / `decompile-project` / wasm) go through it. Duplicating the
step is a defect, not a variation: while the loop lived only in the console
command, `--option formatstring on` was a **silent no-op** on every whole-binary
surface even though `--mode aggressive` — and therefore `auto` under 500 KiB —
named it, so every benchmark number was measured on the weaker of the two. Making
both surfaces honour it then exposed what it costs: a caller whose call sites
yield an override is decompiled **twice**, which is +43% to +75% on a
printf-heavy whole binary. `formatstring` is therefore no longer in the
`aggressive` preset — it is a per-run opt-in on every surface, the speed gate's
prescribed outcome — so the shipped default runs no second decompile and both
surfaces deliver the same C when the option is given (DIV-66).

(kuna) **Surface defaults.** The drivers inject their defaults before the option
pass, from one shared table
(`decompiler/crates/kuna-cli/src/decompile_all.rs (driver_default_options)`), and
which bundle a surface takes is named at its one call site rather than inferred: `option listing on` (DIV-15 — without the Listing the
default-on no-return propagation is a structural no-op, and a stripped binary's
unnamed exit wrapper swallows every following function into its caller), and
`option funcstart_patterns on` plus `option aif on` for non-x86-64 objects only
(DIV-20 — the prologue-pattern pass and the aggressive gap-walk are the primary
discovery sources where the x86-64 scan oracle does not apply).

The discovery bundle belongs to every surface, `kuna functions` included
(DIV-68). Discovery passes exist to add entries, so an inventory command that
declined them reported an entry set the whole-binary command contradicted —
`decompile-all` decompiled functions `functions` did not list. Because both
non-x86-64 passes read the Listing's code units and are inert without it, the
Listing is part of that bundle and not separable from it. The Listing-only
default remains the decompiling surfaces': on x86-64 it is measured entry-neutral,
so building it for enumeration would buy nothing and cost a whole-program decode.
Every injection yields to an explicit caller option — the driver skips it whenever
the caller (or the resolved preset) names that option at all — and none of them
touches the engine default or the console/datatest surfaces.

Single-function `kuna decompile` reads the same table, and that is why the table
is shared rather than duplicated: it builds a `decomp_dbg` script instead of
loading in-process, so it applies the pairs as `option` lines ahead of
`read symbols` (`decompiler/crates/kuna-cli/src/decompile.rs (build_script)`).
What it does differently is *when*. It injects the Listing up front and holds the
discovery half back for a **second attempt**, made only when the console answers
a by-name selection with `no function matches`.

The gap that forced this: on a non-x86-64 image `kuna functions` listed, and
`kuna decompile-all` decompiled, entries that exist only because
`funcstart_patterns` found them, while `kuna decompile <that generated name>`
answered that no such function exists. kuna printed a name it would not then
accept, which is worse than not finding the function at all — an agent cannot
tell a name it mistyped from a name the tool minted. It hid behind the mode
policy, since `auto` resolves to `aggressive` under 500 KiB and that preset names
all three options itself, so it surfaced only above the size threshold or under
an explicit `--mode reliable`.

The retry rather than plain alignment, because the bundle is not free. It changes
the ENTRY SET, and not every entry it adds is real: on i386 and PPC64 the
prologue matcher seeds a start a few bytes inside a function it already knew
(PPC64 ELFv2's local entry point sits 8 bytes past the global one), and
`funcboundflow` truncates the outer function's flow at that seed, so a
`__do_global_ctors_aux` that decompiles to a loop becomes an empty husk. A
whole-binary surface takes that trade knowingly — its inventory has to contain
everything it will decompile, and the husk is a discovery defect to fix at the
analyzer tier, not a reason to under-enumerate. A caller who has already named
one function gains nothing from the wider inventory unless the name is not there,
which is exactly the condition the retry tests. So the first attempt is the
script this surface has always emitted, and only a MISS — not an ambiguity, not a
load failure, not a pipeline abort, and never an `--addr` selector — buys the
second one.

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

(kuna) **Declared function boundaries.** Every function boundary the engine knows
is derived: discovery supplies the entries, and the extent is the
address-contiguous clip `[entry, next_entry)` over an unbounded flow follow
(`decompiler/crates/kuna-console/src/funcextent.rs`). That is the wrong answer on
exactly the images where reverse engineering is hard — obfuscated, packed or
hand-written code, where a missed entry merges two functions and an invented one
splits a body — so a caller can override both halves.

The primitive is a **declared extent**, entry VMA → byte size, held per program in
`decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::declared_extents)`
and written by
`decompiler/crates/kuna-console/src/engine.rs (ConsoleProgram::declare_function)`.
Declaring also installs the `FunctionSymbol` and the name→address registration
`map function` installs, so the entry enumerates, resolves by name and names its
call sites; an address that already carries a function symbol is renamed rather
than given a second one, and only when the caller supplied a name. The store is
consulted by every later load of that entry — `load function`, `load addr`
(`decompiler/crates/kuna-console/src/ifacedecomp.rs`) and the whole-binary loop
(`decompiler/crates/kuna-console/src/project.rs (decompile_targets)`) — which pass
it as the `Funcdata` size that bounds flow following (chapter
[02 §2.1](02-lift-and-flow.md)), and by `funcextent` when the inventory reports an
extent. A declaration therefore outlives the one command that made it, which is
what separates an interface from a one-shot flag.

Two surfaces reach it. The console command is `function bounds <start> [<end>]
[as <name>]`
(`decompiler/crates/kuna-console/src/kuna_console.rs (IfcKunaFunctionBounds)`),
which takes plain integers rather than the `parse_machaddr` address grammar
precisely because that grammar's `[space,offset,size]` size is indistinguishable
from the address width for a small size, and keys the name with `as` so a
declaration that gives a name but no extent cannot have its name read as the end
address. The CLI flag is `--define-function <start[-end][=name] | @file>`
(`decompiler/crates/kuna-cli/src/funcdecl.rs`), repeatable, honored by
`decompile`, `decompile-all`, `functions`, `decompile-project` and `disassemble`.
`end` is exclusive. The script surface emits the console command AFTER `read
symbols` and BEFORE the load, and the in-process surfaces apply the declarations
after `commit_pending_analysis`
(`decompiler/crates/kuna-cli/src/decompile_all.rs (load_program)`): in both, a
declaration is applied after discovery has had its say, because it is an assertion
that outranks it. Durability is caller-carried — the `@file` form is the artifact,
and kuna does not write boundaries back into the image.

(kuna) **Caller assertions (`--assert`).** Declared boundaries are one fact an
agent can state; the assertion plane is the rest of them. Everything the engine
knows about a program it derived, and the console has long carried the commands
that correct each derivation — `rename`, `retype`, `map param`, `map return`,
`map address`, `comment instruction`, `parse line` — while none of them was
reachable from the `kuna` binary, whose generated script emitted a fixed
vocabulary (`option`, `read symbols`, `load`, `kassert`, `function bounds`,
`decompile`).

A **directive** is one line of an intent-keyed vocabulary — an agent does not have
to know that renaming is P9 to rename something — parsed by
`decompiler/crates/kuna-cli/src/assertdecl.rs` and applied by
`decompiler/crates/kuna-console/src/assertions.rs`:

| directive | lowers to | writes at |
|---|---|---|
| `function <start>[-<end>][=<name>]` | `function bounds` | P1, the `--define-function` spelling |
| `typedef <C declaration>` | `parse line` | P5 type-propagation |
| `prototype <func> <C declaration>` | `parse line extern` | P4 prototype-source |
| `data <addr> <C typedeclaration>` | `map address` | P5 const-pointer |
| `param [<func>::]<i> <storage> <C typedeclaration>` | `map param` | P4 prototype-source |
| `return [<func>::]<storage> <C typedeclaration>` | `map return` | P4 prototype-source |
| `comment [<func>::]<addr> <text>` | `comment instruction` | P9 external-refinement |
| `flow [<func>::]<addr> branch\|call\|callreturn\|return` | `override flow` | P2 flow-classification |
| `name [<func>::]<symbol> <newname>` | `rename` | P9 naming-policy |
| `type [<func>::]<symbol> <C type>` | `retype` | P5 type-propagation |
| `readonly <addr>+<size>` | `readonly` | P1 code-data-partition |
| `volatile <addr>+<size>` | `volatile` | P1 code-data-partition |

Four application points, and the ordering between them is forced rather than
stylistic. **Image-scoped** directives (`readonly`, `volatile`) OR one boolean
Varnode property over a memory range, and must be stated before the image's
symbols are mapped: `Scope::addMap` folds the range property into each
`SymbolEntry` as it maps it (`database.cc:1156-1158`) and never consults the range
again, so a property painted afterwards is silently inert over every address the
loader named. The generated console script therefore emits them ahead of `read
symbols`; the in-process surface, where `bootstrap_from_object` has already read
the loader's symbols before a caller can say anything, re-applies the property to
the symbols the range covers (`assertions::paint_property`). Both surfaces then
render the same C. **Program-scoped** directives (`function`, `typedef`, `prototype`,
`data`) are applied right after the analysis commit
(`ConsoleProgram::set_assertions` + `assertions::apply_program_scoped`, called
from `decompiler/crates/kuna-cli/src/decompile_all.rs (load_program)`), for the
same reason a declared boundary is: an assertion outranks discovery.
**Function-scoped** directives (`param`, `return`, `comment`, `flow`) become
decompile SEEDS (`assertions::function_seed`), because a prototype fact is
consumed at flow time and cannot be applied afterwards. **Symbol-scoped** directives (`name`,
`type`) can only be applied to an already-decompiled function — the local they
name does not exist until a decompile has produced it — so
`decompiler/crates/kuna-console/src/project.rs (decompile_targets)` decompiles,
applies them to the first pass's `Funcdata` (`assertions::apply_symbol_scoped`),
and decompiles again with the mutated local scope carried across as
`mapped_symbols`. That second pass is emitted only when such a directive bound to
the function, so every run without one costs exactly what it did before. The
script surface (`decompiler/crates/kuna-cli/src/decompile.rs (build_script)`)
emits the same facts at the same three slots, with the same conditional second
`decompile`.

(kuna) **The C the assertion plane accepts is the C it prints.** Six directives
carry a C declaration, and every one of them goes through the console's
C-declaration grammar (`decompiler/crates/kuna-console/src/grammar.rs (CParse)`),
a port of upstream's `grammar.y`. Upstream has no scalar keywords at all: a base
type is whatever `TypeFactory::findByName` answers, so only Ghidra's own `int4` /
`uint8` / `float8` core-type names parse. kuna's printer, though, spells those
types the way the target's own compiler would (§9,
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_ctypes.rs`), which left the two
halves speaking different languages: a declaration kuna had just emitted could
not be pasted back at it, and the manual's own example
(`prototype authenticate int authenticate(char *user,char *pass)`) was rejected
as a syntax error.

The grammar therefore also accepts the standard C scalar specifiers
(`decompiler/crates/kuna-console/src/grammar.rs (CParse::scalar_specifier)`):
`void`, `char`, `short`, `int`, `long`, `float`, `double`, `signed`, `unsigned`,
`_Bool` and `wchar_t`, in any legal combination and in every position a type may
appear — a return type, a parameter, a `type` / `param` / `return` / `data`
operand, a struct field. A run of these keywords names ONE base type, and its
width is read from the compiler spec's `<data_organization>` rather than from a
fixed table, so `long` is eight bytes on LP64 and four on LLP64 — the same
source, read the same way, that §9's speller prints them back out from. A
combination that is not a C type (`short long`, `float int`, three `long`s) is
rejected by name rather than as a bare syntax error, and a keyword whose width
the compiler spec never declared names nothing on that target and is rejected
too, rather than resolving to a zero-sized type.

The Ghidra vocabulary is untouched and still wins: a run of exactly one keyword
is resolved by `findByName` first, so `void`, `char` and any host-supplied type
that happens to be spelled with a keyword resolve to exactly the interned type
they always did. Only combinations, and the keywords the type factory does not
name, take the width-driven path.

A directive that names no function binds to the function being decompiled, which
is unambiguous only when the run selected exactly one; on a whole-binary run it
would silently mean *every* function that happens to have a `v2`, so it is
rejected there with a detail naming the `<func>::<operand>` form. Rejecting is the
design: every directive produces exactly one row in the run's report
(`ConsoleProgram::assertion_outcomes`, serialized as the `assertions` array of
every `--json` document and spoken on stderr on the human surface), because a
directive that is accepted and does nothing is worse for an agent than one that
errors. `--assert-strict` turns any rejection into a non-zero exit; without it a
rejection is reported and the run continues, so a batch of forty renames against a
re-decompiled binary does not lose the other thirty-nine to one stale name.
Durability is caller-carried, as it is for boundaries: `--assert @FILE` is the
artifact.

A `flow` directive is the sharpest of the four function-scoped ones, and the only
one that changes which bytes are in the function at all. P2 classifies the flow
out of each instruction — branch, call, call-that-does-not-return, return — and
`FlowInfo::process` consults the per-function `Override` store
(`has_flow_override`/`get_flow_override`, then `Funcdata::overrideFlow`) before it
decides. The directive seeds that store: `assertions::seed_one` resolves the
address in the default code space, maps the caller's word through
`Override::string_to_type` (rejecting anything outside `branch`, `call`,
`callreturn`, `return` with a reason rather than dropping it), and parks the pair
in `FunctionSeed::flow_overrides`, which
`decompiler/crates/kuna-console/src/project.rs (decompile_targets)` appends to the
derived overrides it already carries — the analysis's `call error(nonzero,…)`
no-return prunes — so a caller-stated fact wins the map insert at an address both
name. The script surface reaches the same store through the ported console
command (`kuna-console/src/ifacedecomp.rs (IfcFlowOverride)`), whose facts the
console re-seeds on every IR rebuild; the two surfaces render the same C. Because
the override is read at flow time, a type the engine cannot honour at that
instruction — `call` at an indirect call, which has no destination to make direct —
raises `Could not apply flowoverride` and the run reports that as the function's
error, rather than decompiling as though nothing had been asserted.

A `readonly` range is the one directive whose effect depends on a second switch:
folding a read-only load into the value behind it is
`ActionVarnodeProps`/`Funcdata::fillin_read_only`, gated on the program-wide
`readonly` option, which is default-off. Asserting a range therefore turns that
option on for the run — a directive that paints a property and then declines to
act on it would be the accepted-and-inert failure this plane exists to end — and
it is applied ahead of the caller's own `--option`s, so an explicit `--option
readonly off` still wins. The reverse composition is not equivalent: the option
alone folds only what the loader already marked (section flags), which is why
`.data` that nothing writes needs the range and not the switch.

There is deliberately no `global` directive. `global add`/`global remove` are the
console commands `phases.toml` names as the `code-data-partition` exposure and
they are wired here onto `Database::add_range`/`remove_range`, but every stock
cspec's `<global>` already claims the whole default data space (`<range
space="ram"/>`), so on any ordinary image an added range was global before the
caller spoke; only the removal direction moves the C. Exposing an assertion that
is measurably a no-op would be the same failure the plane is built to avoid.

(kuna) **Load-time env bridges.** Seven loader gates are consumed *inside* the
bootstrap — before any console `option` line can possibly run — so the option
surface alone cannot deliver them; each is bridged through a process environment
variable exported first (`decompiler/crates/kuna-cli/src/decompile_all.rs
(apply_loadtime_env)` in-process; the equivalent `Command::env` calls in
`decompiler/crates/kuna-cli/src/decompile.rs` for the subprocess):

| env var | option | read at |
|---|---|---|
| `KUNA_RELOC_OBJECTS` | `relocobjects` | relocatable-object (`ET_REL` `.o`, COFF `.obj`) layout + relocation resolution in the loader, `decompiler/crates/kuna-analysis/src/loadimage_object.rs (RELOC_OBJECTS_ENV)` |
| `KUNA_I386_PIE_PLT` | `i386_pie_plt` | i386 PIE PLT-stub decode, `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_i386_pie_plt.rs (I386_PIE_PLT_ENV)` |
| `KUNA_DYNRELOCS` | `dynrelocs` | linked-image dynamic-relocation application + the `PT_GNU_RELRO` constant slots, `decompiler/crates/kuna-analysis/src/loader/kuna_dynrelocs.rs (resolve)` |
| `KUNA_RELOCREBASE` | `relocrebase` | relocatable-object analysis-fact rebase, `decompiler/crates/kuna-analysis/src/loader/kuna_relocrebase.rs (rebased_view)` |
| `KUNA_IFUNCFPRET` | `ifuncfpret` | x86-64 IFUNC (`R_X86_64_IRELATIVE`) stub naming, `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_ifuncfpret.rs (IFUNCFPRET_ENV)` |
| `KUNA_TYPEDEPTH` | `typedepth` | DWARF full-depth type resolution, `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_typedepth.rs (TYPEDEPTH_ENV)` |
| `KUNA_DWARFSTRUCTS` | `dwarfstructs` | DWARF aggregate-layout import (`DW_AT_byte_size` + `DW_TAG_member` walk), `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_dwarfstructs.rs (DWARFSTRUCTS_ENV)` |
| `KUNA_DWARFVARIANTS` | `dwarfvariants` | DWARF variant-part (discriminated-union) import (`DW_AT_discr` + `DW_TAG_variant` walk), `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_dwarfvariants.rs (DWARFVARIANTS_ENV)` |
| `KUNA_PDATACHAINED` | `pdatachained` | PE `.pdata` chained-`UNWIND_INFO` entry suppression, `decompiler/crates/kuna-analysis/src/analyzers/entry/pe_entry.rs (pdata_begins)` |
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
survive op deletion, §0.6, and which counts its own insertions and removals in an
*epoch* so a holder of cached successor ids can tell in O(1) whether the tree
still orders the way it did — `decompiler/crates/kuna-decomp/src/substrate/op.rs
(optree_epoch, ops_after_seq)`), and **two** block graphs
(`decompiler/crates/kuna-decomp/src/substrate/block.rs (BlockGraph)`): `bblocks`,
the CFG, and `sblocks`, the structuring tree — physically distinct, seeded as a
`BlockCopy` mirror of the CFG when structuring begins
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs (seed_sblocks_copy)`).

The varnode bank's two sorted trees are the container the decompiler touches most
— a large function creates and destroys well over a million Varnodes, each one
inserted into and removed from both — so their keys
(`decompiler/crates/kuna-decomp/src/substrate/varnode.rs (LocKey, DefKey)`) do not
store an `Address` or a `SeqNum` directly. They store the ordering triple those
compare by, flattened into plain integers: the sentinel rank, the space index and
the offset (`decompiler/crates/kuna-base/src/address.rs (Address::sort_key)`).
Lexicographic comparison of the triple reproduces `Address::cmp` exactly — two
Addresses sharing a space pointer share a rank and an index and fall through to
the offset, which is what the pointer-equality fast path does — so the tree order
is the C++ comparator's order, while the key itself becomes `Copy`: no reference
counting on clone or drop, no pointer chase into an `AddrSpace` to compare, and a
smaller node. Insertion also takes a single descent
(`decompiler/crates/kuna-decomp/src/substrate/varnode.rs (VarnodeBank::xref)`):
the "is an equivalent varnode already present" lookup and the insertion that
follows it are the same search, because the `insert` flag set afterwards is
outside the `(input|written)` mask the key is built from and so cannot move the
entry.

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
  A qualified symbol name is nested by splitting it on every `::`
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
  (find_create_scope_from_symbol_name)`), one Scope per component, and an **empty**
  component — `a::::b`, `::b` — cannot name a Scope: `attach_scope` rejects it.
  That rejection is raised while the loader symbols are being installed, i.e.
  inside the architecture build, so it does not cost one symbol — it escapes the
  build, and every command answers `could not build an architecture for <binary>:
  Non-global scope has empty name` and emits nothing. (Answering with the reason
  attached is what DIV-90 gave the subprocess surface, which until then replaced
  it with a fixed string — so this symptom, which `docs/options.md` publishes as
  the trigger for flipping `symbolnamerepair`, was unmatchable from the surface an
  agent is most likely driving.) Symbol-name bytes are attacker-controlled data
  that no header check
  validates, which makes that a denial-of-analysis primitive a hostile binary can
  buy for a few `.strtab` bytes. `symbolnamerepair` (on|off, default on;
  `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_symbolnamerepair.rs`) skips
  the degenerate component instead, so the symbol keeps the rest of its scope path
  and the load survives; only the empty component is treated as degenerate, since
  every other string names a Scope perfectly well however strange it looks. Off
  restores the hard error, which is what someone investigating a binary's symbol
  table itself wants to see. Like the other gates consumed inside `load file`
  (`relocrebase`, `i386_pie_plt`, `typedepth`) it is read through a process
  environment variable rather than an `Architecture` flag, because `option` is
  applied downstream of the load it would have to govern.
  The scope path is not the only thing a name's bytes reach: the same string is
  printed into emitted C, and nothing on the way validates it.
  `symbolnamechars` (off|safe|ident, default safe;
  `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_symbolnamechars.rs`) is the
  character half of the same problem, and it sanitizes at the mint rather than at
  the printer for a reason that is about the control surface: `kuna decompile
  <name>`, the console's `load function` and the DB scope path all key on the
  string the symbol table holds, so a printer-side rewrite would put a name in
  the `.c` that cannot be handed back. Chapter 01 §1.1 states the byte-level
  behavior; here it only matters that the name in `prog.symbols`, in `kuna
  functions`, in the `.c`/`.h`/`.asm` export and in the Scope chain is ONE
  string, and that it is decided before the symbol reaches this database.
  The same split is also a **resource** seam, and a second gate bounds it.
  Nothing limited how many components a name could have, and a `Scope` is not
  cheap — a range list, three ordered maps, two strings and a per-address-space
  map table, about 1.5 KB resident — so one name bought one `Scope` per `::`
  without limit, and the interning key includes the parent, so even a repeated
  component name allocated a fresh `Scope` at every level. That made a symbol
  name a roughly 498-fold input-to-RSS amplifier: 600 KB of `.strtab` in a single
  name cost 292 MB, and the whole-binary path is quadratic in depth on top of
  that, so tens of kilobytes already bought a stall of tens of seconds.
  `symbolnamebound` (`<n>|off`, default 256;
  `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_symbolnamebound.rs`) caps
  the scope-component count, and with it each component's length and the whole
  scope path's length. A name over a limit is **folded**, not truncated: the
  dropped run of components collapses into one synthetic component carrying a
  hash of the exact bytes it replaced, so two names that differed only inside the
  folded region still differ, and two symbols that share a scope path but differ
  in their base name still share the folded scope. The base name is never
  rewritten — it nests no `Scope`, so it was never the amplifier. The hash is
  written out in the module rather than taken from the standard library's default
  hasher, whose per-process seed would make the folded spelling differ between
  runs and turn every golden comparison into noise. The fold is applied
  identically on the **read** path
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
  (resolve_scope_from_symbol_name)`) and at the loader's own name list, and it is
  idempotent, so a symbol installed under a folded path is addressable by the name
  the binary spells *and* by the name the listing renders, and one spelling
  reaches every surface. The defaults are set from measurement, not from taste:
  over 1,683,515 demangled names — the repo fixtures, fourteen large C++ objects,
  nine rustc-built binaries, and the sixty largest system objects — the deepest
  `::` nesting is 21 (a Rust name; C++ never exceeds 6), and over the DWARF names
  of a rustc binary it is 79, because the DWARF path does not strip template
  arguments and every `::` inside `<…>` counts. The longest scope component is
  exactly 256 bytes, which is where rustc's own mangler truncates, and the longest
  name of all, 1,780 bytes, carries no `::` at all and so is not a scope path in
  the first place. The ceilings sit at 256, 1024 and 4096, three to four times
  above each, and the fold is therefore unreachable in practice; `off` restores
  the unbounded behavior exactly, for reproducing a report.
  The bound caps what **one name** costs, not what a symbol **table** costs.
  The amplifier is per-`Scope`, so the same `.strtab` bytes spent on many
  moderately deep names buy the same memory — 3,000 distinct 64-component names,
  1.9 MB of ELF, cost 343 MB with the gate on or off, since none of them reaches
  the ceiling and none of their scopes can be shared. Closing that needs a cap on
  the total `Scope` population, or a cheaper `Scope`; what this gate closes is
  the reported primitive, one name turning 600 KB of `.strtab` into 292 MB, and
  the quadratic whole-binary blowup that rode on it. Same loader-tier env
  bridge as `symbolnamerepair`, and deliberately a
  separate gate from it: turning the repair off is a debugging affordance for
  someone inspecting a symbol table, and that must not also remove a resource
  bound.
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
  `v850indirectbranch`, which would mis-decode register-indirect calls off-V850,
  `dwarf_lines`, which annotates rather than recovers and would bury a `-g`
  binary's body in `/* src.c:NNN */` comments, and `formatstring`, whose
  re-decompile loop misses the speed budget by an order of magnitude on a whole
  binary — DIV-66; the exclusion list is enforced by an invariant test in
  `decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs`, so a default-off
  option is either in the preset or listed there with its reason),
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
use-point selection. Within each space, an offset interval index restricts
container candidates to entries whose first offset is at or below the query
start and whose last offset reaches the query end. The final reduction still
uses stable encounter order for equal-size entries, preserves the effect of the
exact-size early break, and applies the original use-point validity test.
Property, naming, container, and callee lookups first isolate the requested
space, so register, stack, and other non-global varnodes do not scan mappings
from unrelated spaces.

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
  `decompile` command, *except* when `decompile` adopts the IR `load function`
  already followed (§0.8), and that adoption is refused the moment any command at
  all — an `option` among them — has run since the load.

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
effects mid-op, and the reset order is part of the observable output. The C++
cursor is a map iterator whose `++` is O(1); kuna models it as the last consumed
`SeqNum` (so it survives the op's own deletion) and reads a short *run* of
successors per tree descent rather than one search per op, discarding the run
whenever the optree epoch above moves — any op created or destroyed by anything
other than the pool's own consumption of the op it just left. The visit order is
the search's, one buffered value at a time. The
materialized `decompile` tree's listing is byte-equal to the C++ oracle dump
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(UNPORTED_ALLOWLIST)` — empty).

Flow-follow itself runs *before* the tree (the upstream `followFlow` →
`startProcessing` order), bounded by the P0 flow options — decode-error policy
`error_toomanyinstructions` and a 100000-instruction ceiling by default
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`), applied at
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (follow_flow_on_fd)`.

**Where a run's time went** (kuna). The schedule can be asked to account for
itself: with `KUNA_ACTION_PROF` set to a path, every `apply` call is timed and
the engine writes an exclusive-time table there
(`decompiler/crates/kuna-decomp/src/infra/actionprof.rs`), rewritten each time
the schedule unwinds so the file holds the running total for the whole process.
Time is exclusive — a group is charged only what it spends outside its children,
so the rows sum to the schedule's wall time and a container cannot hide a leaf —
and each row is keyed by the root variant it ran under, which is what separates a
function's own `decompile` pass from the reduced `jumptable` pipeline running on
a partial clone beside it. The root label is set where the variant is selected
(`decompiler/crates/kuna-decomp/src/infra/action.rs
(ActionDatabase::set_current)`). This is a measuring instrument, not a decision
point: it changes nothing the engine emits, and with the variable unset it costs
one cached read per `apply`.

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
| P0 → everything, the outer loop | knowledge-store re-run | an operator/agent writes an assertion (`option`, `kassert`, override) and re-decompiles | the entire P0 store | `decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_assert.rs (Dispatch)`; the console rebuilds the IR per `decompile`, re-seeding stashed facts — `decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (decompile_func_full_with_override_dyn)` (§0.8 is when the rebuild is skipped) |

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

## 0.8 One flow follow per decompile

The console has two commands that build IR for a function, and a `kuna decompile
<bin> <fn>` runs both: `load function <fn>` (or `load addr`), then `decompile`.
Upstream follows the flow once — C++ `IfcFuncload` follows it, and `IfcDecompile`
re-runs the action pipeline on *that* `Funcdata` after
`Architecture::clearAnalysis`. kuna's `decompile` instead builds a fresh
`Funcdata` and follows the flow again, because a decompile is seeded with facts
that `load function` never applied — and some of them are consumed AT FLOW TIME,
so re-seeding them onto an already-followed IR would be too late:

- `override prototype` call-site overrides, which `FlowInfo::build_call_specs`
  consumes as it builds the call specs, and every `parse line` prototype re-parked
  on its global `FunctionSymbol` before the drive (a callee prototype the follow
  resolves against). These two are the genuinely flow-time seeds.
- `override flow` facts, likewise consumed at flow time — but `load function`
  seeds these too, from the same store, so the two follows agree on them.
- `map address` symbols and DWARF stack locals, `type varnode %REG(pc)` usepoint
  symbols, `map hash` dynamic symbols, a `parse line extern` prototype for the
  function itself, and `map param` storage locks. The drive re-seeds all of these
  onto the `Funcdata` *after* the follow, so they do not require a re-follow —
  they are nonetheless required absent below, because "no facts at all" is the
  condition that is cheap to prove and impossible to get subtly wrong.

So the rebuild is *required* when a flow-time fact exists, and pure waste when no
fact exists at all — which is every plain `kuna decompile`. The waste is not
small: the second follow repeats the whole lift, the block build, and the
jump-table sub-decompilation (§0.7), which on a large switch-heavy function is the
single most expensive thing the run does.

`decompile` therefore **adopts** the loaded IR when it can prove the rebuild would
repeat the same follow
(`decompiler/crates/kuna-console/src/ifacedecomp.rs (PristineFlow)`, consumed
through `decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs
(decompile_func_full_with_override_dyn_prefollowed)`). Two independent guards must
both hold:

- **Every seed above is empty**, flow-time or re-seeded alike. A flow-time seed
  present means the loaded IR was followed without it, so adopting would silently
  drop it; the re-seeded ones are held to the same bar deliberately, so the guard
  is one question ("did the console learn anything about this function?") rather
  than a per-seed judgement that a later seed could be forgotten from.
- **The architecture is configured as it was at the load.** A `Funcdata`
  snapshots the per-function flags into its ArchSeam handle when it is *built*
  (§0.5), so a flag flipped afterwards is invisible to it. Three things move
  between the load and the drive and therefore refuse adoption: `formatstring`,
  which turns read-only propagation on around the drive so the printf format
  constant can be read (adopting there leaves `printf((char *)(dat_… + …), …)`,
  the format string unresolved); the watchdog's per-function budget, armed inside
  the drive; and ghidra mode's staged name/dynamic/prototype-model
  recommendations.
- **The `decompile` is the immediately next command.** `load function` records the
  console's command counter
  (`decompiler/crates/kuna-console/src/interface.rs (IfaceStatus::command_seq)`)
  and `decompile` requires it to have advanced by exactly one, along with the same
  name, entry, declared extent and flow overrides. The counter is the whole
  invalidation story on purpose: an `option` that changes a flow-time decision, a
  `kassert`, a `map`, a second `load` — anything at all — advances it, so no
  command needs its own invalidation hook and none can be forgotten.

Adoption is a pure-performance seam: the adopted `Funcdata` is the one the rebuild
would have produced, so the emitted C is byte-identical either way, and
`decompiler/crates/kuna-console/tests/verify_flowreuse.rs` asserts exactly that
(plus that the fast path is really taken, via `IfaceDecompData::adopted_flows`).
The one place the two paths differ is the failure arm: a drive that aborts
consumes the adopted IR, where the rebuild path left the loaded `Funcdata`
untouched for a following `print C`, so the error arm re-follows the recorded
name/entry/size/overrides to put it back.

## 0.9 Reading order

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
