# Analysis-tier port backlog — the complete "eventually do them all" list

> **Companion docs:** the *process* log of what is already done is
> [`analysis-port-log.md`](analysis-port-log.md); the gap inventory is
> [`missing-analyses.md`](missing-analyses.md); the dependency-ordered execution
> plan for the deferred work is [`analysis-port-plan.md`](analysis-port-plan.md).
> This file is the **exhaustive remaining-work backlog**: every Ghidra analyzer
> that is *not* yet ported, recorded as actionable future work — including the
> out-of-scope ones, because the standing intent is "we need to eventually do
> them all."

## How to read this

Ghidra ships **142** non-test `*Analyzer.java` classes. They split three ways:

| Bucket | Count | Where it lives |
|---|---:|---|
| **Ported** (kuna-analysis Increments 1–17, + the PLT/GOT loader markup done pre-effort) | **~14 analyzers** | [`analysis-port-log.md`](analysis-port-log.md) work-list (✅ rows) |
| **Engine-inherited** — the ported decompiler engine already does this; no analyzer-tier pass is needed | **~13** | this doc, **Tier B** (recorded, not scheduled) |
| **Backlog** — everything still to do, from real ELF gaps to never-relevant container formats and pure infra base-classes | **~115** | this doc, **Tiers A, C–G** |

Each backlog row carries six fields:

- **Purpose** — what the Ghidra analyzer does.
- **Why-not-yet** — why it is not ported (scope decision, missing prerequisite, or genuinely infeasible at kuna's tier).
- **Prereqs** — the concrete thing kuna would need first (a Listing model, a Mach-O loader, a SymbolicPropagator, a side-file seam, …).
- **Approach** — the faithful port shape, or the explicit "do not port" / "salvage only the facts" decision.
- **Difficulty** — `easy` / `med` / `hard` / `xhard` / `subsystem` (subsystem = a whole new front-end/loader/parser, not a pass).
- **ELF-relevance** — `high` / `medium` / `low` / `none` — payoff for kuna's actual mission (standalone ELF C decompilation).

**The one architectural fact behind almost every deferral:** kuna's analyzer tier
(`kuna-analysis`) is an `AnalysisPass` that emits **facts**
(`AnalysisOutput { symbols, entries, noreturn, readonly, strings, prototypes, call_fixups }`)
read-only over an `object::File` + `ObjectLoadImage`, committed once in
`commit_analysis_output`. **The decompiler runs AFTER analysis.** So there is *no
pre-decompile Listing, no ReferenceManager, no FunctionManager, no
SymbolicPropagator, no PseudoDisassembler, and no mutable Program* at this tier.
The recurring blockers below are all corollaries of that one design choice — and
the recurring *opportunities* are the facts a few of these analyzers produce that
*can* be salvaged into `AnalysisOutput` without a Listing (entries, symbols,
strings, readonly ranges, prototypes, context paints, tracked-register seeds).

## Grand-total accounting (must sum to 142)

```
PORTED  (Increments 1–17 + PLT/GOT loader markup)                    14
  NoReturnFunctionAnalyzer (base + Rust + Go arms)                    1
  GnuDemanglerAnalyzer (Itanium + Rust)                               1
  ApplyDataArchiveAnalyzer (built-in libc table)                     1
  StringsAnalyzer (+ printer string-literal change)                  1
  SourceLanguageAnalyzer (detection half)                            1
  DWARFAnalyzer (names + typed sigs + stack locals — .debug_info)    1
  EntryPointAnalyzer / FunctionStartAnalyzer (minimal subset)        1
  GccExceptionAnalyzer (FDE-start oracle only)                       1
  ArmSymbolAnalyzer (+ ARM_ElfExtension $t/$a TMode markers)         1
  MipsAddressAnalyzer (t9-tracking / $gp half)                       1
  CallFixupAnalyzer (install half)                                   1
  FormatStringAnalyzer (constant-format path, gated off)             1
  AddressTableAnalyzer (ported, disabled by default)                 1
  ElfDefaultGotPltMarkup (PLT/GOT names — loader code)               1

ENGINE-INHERITED (Tier B — engine already does it; recorded only)   13
  ConstantPropagationAnalyzer                                        1
  DecompilerSwitchAnalyzer                                           1
  DecompilerFunctionAnalyzer                                         1
  DecompilerCallConventionAnalyzer                                   1
  StackVariableAnalyzer                                              1
  FunctionAnalyzer ("Subroutine References", real analyzer)         1
  FunctionAnalyzer (util.state interface — not an analyzer)          1
  SharedReturnAnalyzer                                               1
  SharedReturnJumpAnalyzer                                           1
  SegmentedCallingConventionAnalyzer                                 1
  X86FunctionPurgeAnalyzer (extrapop inferred by engine)            1
  RustStringAnalyzer (detection inherited; split infeasible)         1
  MicrosoftDemanglerAnalyzer (slots into ported demangle pass)       1

BACKLOG (this doc, Tiers A + C–G)                                   115
  Tier A  ELF-relevant gaps                                          11
  Tier C  per-CPU processor analyzers                               31
  Tier D  non-ELF executable formats (Mach-O/PE/COFF/PEF/.NET)      ~20
  Tier E  non-ELF container/firmware/filesystem formats             ~28
  Tier F  OS/runtime/language subsystems (PDB/Go/Swift/ObjC/FID…)   ~14
  Tier G  framework / abstract / non-analyzer infra                 11
  ─────────────────────────────────────────────────────────────
  Tier A + C–G                                                     115

GRAND TOTAL  14 + 13 + 115 = 142  ✓
```

> **Reconciliation note.** The exact split of the ~115 backlog rows across Tiers
> D/E/F is approximate because Ghidra's class count includes a handful of
> same-named or sub-clustered classes (e.g. the two `FunctionAnalyzer.java`
> files; `SharedReturnAnalyzer` + `SharedReturnJumpAnalyzer` as one subgroup; the
> two `state/FunctionAnalyzer.java`/`MySwitchAnalyzer.java` util classes;
> `MachoFunctionStartsAnalyzer` appearing under both `Features/Base` and the
> Mach-O cluster). The 142 / 14 / 13 / 115 **column totals are exact**; only the
> internal Tier-D/E/F apportionment carries that ±-few rounding. Nothing is
> dropped — every analyzer below appears in exactly one tier.

> **Two analyzers appear here as *deeper gaps of a partially-ported analyzer*,
> not as new classes:** `DWARFAnalyzer` (line/source-map half) and
> `GccExceptionAnalyzer` (full CFI + LSDA). Their *classes* are counted in the
> ported 14 (their `.debug_info` / FDE-start halves shipped); their unported
> halves are listed in **Tier A** as scheduled follow-on work, not as additional
> entries against the 142. Likewise `FormatStringAnalyzer`'s non-constant path is
> a Tier-B refinement of an already-working pass.

---

## Tier A — ELF-relevant gaps (do these first)

These are the items that would visibly improve kuna's decompiled C output for
real ELF binaries. Ordered roughly by payoff-per-effort.

| Analyzer | Purpose (1-line) | Difficulty | ELF |
|---|---|---|---|
| `_INIT_/_FINI_` array-element naming (loader markup) | name ctor/dtor functions from `DT_INIT_ARRAY`/`DT_FINI_ARRAY`/`DT_PREINIT_ARRAY` instead of `sub_<addr>` | easy | medium |
| `RISCVAddressAnalyzer` | seed `gp` from `__global_pointer$` so `.sdata`/`.sbss` loads resolve | easy | high |
| `DWARFAnalyzer` (line/source-map half) | `.debug_line` → `// file:line` comments on decompiled statements | med | medium |
| `PowerPCAddressAnalyzer` (r2-TOC seed only) | seed r2 (TOC) at entries on PPC64 ELF (the MIPS-t9 recipe) | med | high |
| `GccExceptionAnalyzer` (Tier-1: `.eh_frame_hdr` + LSDA landing-pads) | recover catch-handler code as additional entries from the LSDA call-site table | hard | high |
| `FunctionStart*` byte-pattern family (full bytesearch engine + vendored XMLs) | generalize entry-discovery from the x86-64-only minimal subset to all arches via the real `DittedBitSequence` engine + vendored `patterns/*.xml` | hard | high |
| `MachoFunctionStartsAnalyzer` | `LC_FUNCTION_STARTS` entry oracle — trivial *if* a Mach-O bootstrap exists | med | none (gates Mach-O) |
| `AARCH64PltThunkAnalyzer` | pattern-matched (non-relocation) AArch64 PLT thunks + x17 tracking | med | high |
| `CreateThunkAnalyzer` + `ExternalEntryFunctionAnalyzer` (thunk/external object model) | model single-jump stubs as thunks forwarding callee name/proto | hard | medium |
| `OperandReferenceAnalyzer` + `DataOperandReferenceAnalyzer` | listing-time operand-ref markup (scalar→`char*`, switch tables, sub-discovery) | med | medium (⛔ infeasible-at-tier) |
| `FindNoReturnFunctionsAnalyzer` | flow-based *discovered* no-return (vs. the ported known-list) | hard | medium (engine feedback pass) |

### Tier A — detail

**`_INIT_/_FINI_` array-element naming** — *easy / medium / the cheapest genuine ELF win.*
- *Purpose:* the ELF loader names each `.init_array`/`.fini_array`/`.preinit_array` pointer-table element and the ctor/dtor it points at (static C++ ctors, `__attribute__((constructor))`), so those functions get meaningful names. (Ghidra: `ElfProgramBuilder.java`, `ElfDynamicType` DT_INIT_ARRAY=25/DT_FINI_ARRAY=26/DT_PREINIT_ARRAY=32.)
- *Why-not-yet:* Increment 5 ported the array read as an **entry oracle** (functions get discovered, so they decompile as `sub_<addr>`) but stopped at discovery — naming was out of the oracle's scope, and `DT_PREINIT_ARRAY` is not yet read.
- *Prereqs:* none kuna lacks — pure byte-read over `.dynamic` + the array sections, exactly the data `s1_entry` already parses.
- *Approach:* extend `s1_entry/mod.rs` oracle-2 to also emit a `SymFact{Function, name}` per array element (Ghidra's array-element label scheme), optionally a `Data` SymFact for the pointer slot, and add `DT_PREINIT_ARRAY` to the existing `DT_INIT_ARRAY`/`DT_FINI_ARRAY` read. Pure additive.

**`RISCVAddressAnalyzer`** — *easy / high / the cleanest of the gp-seed family.* (Ghidra: `Processors/RISCV/.../RISCVAddressAnalyzer.java`.)
- *Purpose:* derives `gp_assumption_value` from `__global_pointer$` and seeds `gp` at every function entry so gp-relative (`.sdata`/`.sbss`) loads resolve.
- *Why-not-yet:* simply not reached yet; **no technical blocker** — the `TrackedRegFact` seam and the symbol are both available.
- *Prereqs:* `TrackedRegFact` (EXISTS), `__global_pointer$` lookup (trivial).
- *Approach:* a small RISC-V pass modeled on `s1_loader/mips_markers.rs`: look up `__global_pointer$`, emit `TrackedRegFact gp=addr` at every function entry. RISC-V `gp` is unconditional, so the unconditional `COPY #gp→gp` form is **safe** here (none of the MIPS PIC-poisoning caveat applies). Strongly recommended easy pickup.

**`DWARFAnalyzer` (line/source-map half)** — *med / medium / largest still-open piece of a partially-ported analyzer.* (Ghidra: `DWARFAnalyzer.java` + `format/dwarf/line/*` + `DWARFLineInfo*Script`.)
- *Purpose:* decode `.debug_line` (a stateful bytecode VM) into an address→(file,line) map so each decompiled statement is annotated with its source file/line. kuna ported the `.debug_info` side (names/types/locals, Increments 6/14); this is the second half it dropped.
- *Why-not-yet:* listed among the Increment-6 "faithful losses (DOC)" because kuna's printer had no comment channel ("listing cosmetics, zero decompiler-output payoff").
- *Prereqs:* gimli (already a dep — exposes `IncompleteLineProgram`/`LineRows`, so no need to hand-port the opcode VM); a printer hook to attach per-address `// file:line` comments.
- *Approach:* add a `source_lines: Vec<(u64,String,u32)>` fact, populate it in `s1_dwarf/mod.rs` from gimli, and in `s9_emit/coreaction_render.rs` look up the nearest source line for each emitted statement's seqnum address and emit a trailing `/* foo.c:42 */`. Faithful minimal scope = `DWARFLineInfoCommentScript` behavior, not the full `SourceFileManager` SourceMap subsystem.

**`PowerPCAddressAnalyzer`** — *med / high (r2 seed); the rest is hard.* (Ghidra: `Processors/PowerPC/.../PowerPCAddressAnalyzer.java`.)
- *Purpose:* PPC TOC/PIC recovery + switch tables: propagates r2 (TOC pointer, default-on for 64-bit ELF) and r30 (PIC base), marks `lis/addi` dual-instruction address pairs, recovers switch tables (default-on for PPC).
- *Why-not-yet:* the r2 seed is directly portable on the existing tracked-reg seam but hasn't been written (PPC64 prioritized below x86/ARM/MIPS); the dual-instr/switch parts need the absent propagator.
- *Prereqs:* `TrackedRegFact` (EXISTS) for r2/r30; the analysis-tier SymbolicPropagator + Listing for dual-instr markup and switch tables.
- *Approach:* an `s1_loader/ppc_markers.rs` modeled on `mips_markers.rs` that seeds r2 from the `.got`/`.TOC` symbol (or the e_entry TOC convention) at each function entry on PPC64 ELF. Defer r30 PIC-base, `lis/addi` markup, and switch recovery to the propagator buildout.

**`GccExceptionAnalyzer` (full CFI + LSDA, beyond FDE-starts)** — *hard / high.* (Ghidra: `exceptionhandlers/gcc/GccExceptionAnalyzer.java` + `sections/*` + `structures/ehFrame/*` + `structures/gccexcepttable/*`.)
- *Purpose:* full GCC/Clang EH markup: parse the complete CIE/FDE CFI in `.eh_frame` (CFA program), build per-function `RegionDescriptor`s, and follow each FDE's LSDA pointer into `.gcc_except_table` to decode call-site/action/type tables — emitting try/catch landing-pad ranges and comments. kuna ported **only** the FDE `pcBegin` extraction as an entry oracle (Increment 5).
- *Why-not-yet:* Increment 5 deliberately scoped to "FDE-start extraction (NOT CFI/LSDA)"; CFI unwind tables don't affect decompiled C and the try/catch markup is listing-comment cosmetics with no rendering channel.
- *Prereqs:* **Tier-1** (feasible now) needs nothing kuna lacks — pure byte-reads. **Tier-2** needs a Listing + comment channel.
- *Approach:* **Tier-1 (do it):** extend the existing `s1_entry` CIE/FDE walk (~150 lines, DW_EH_PE decoder already hand-ported) to (a) parse `.eh_frame_hdr`'s `fde_count` + sorted FDE table as a cheaper entry oracle, and (b) read each FDE's augmentation-data LSDA pointer, decode the LSDA header + call-site table, and emit each non-zero **landing-pad address as an additional entry** (recovers catch-handler code that no symbol/FDE-start names). **Tier-2 (defer):** the CFA opcode VM (`DwarfCallFrameOpcodeParser`) + try/catch comments — document as a loss like the DWARF-line cosmetics.

**`FunctionStart*` byte-pattern family** (`FunctionStartAnalyzer`, `FunctionStartPostAnalyzer`, `FunctionStartDataPostAnalyzer`, `FunctionStartFuncAnalyzer`, `FunctionStartPreFuncAnalyzer`) — *hard / high / single biggest entry-discovery win.* (Ghidra: `Features/BytePatterns/.../FunctionStart*Analyzer.java` + `Processors/*/data/patterns/*.xml` (52 XML files / 17 procs) + `ghidra/util/bytesearch/*`.)
- *Purpose:* the full arch/compiler-specific function-start engine: a `BulkPatternSearcher` over ditted bit-sequence patterns whose `<funcstart>`/`<possiblefuncstart>`/`<codeboundary>`/`<setcontext>` actions create function starts, set context registers (ARM TMode), mark thunks (`section=\.plt`), mark noreturn, and apply post-rules. Five subclasses run the same engine at different phases keyed off constraint flags. kuna ported a **minimal hand-coded subset** (a tiny `DittedBitSequence` matcher + a few x86-64 gcc prologue patterns, oracle 5).
- *Why-not-yet:* Increment 5 substituted "a minimal bare-`<funcstart/>` set" for the full engine and dropped the `after="defined"`/`validcode` post-rules (no PseudoDisassembler); oracles 4-5 are x86-64-only in v1.
- *Prereqs:* (1) a real `Pattern.readPatterns`/`DittedBitSequence` XML reader + `BulkPatternSearcher` so the vendored XMLs are consumed verbatim; (2) **vendor** the `Processors/*/data/patterns/` XMLs (currently not in `specs/`); (3) for the constraint-gated subclasses, a Listing + PseudoDisassembler (`checkValidSubroutine`).
- *Approach:* port `ghidra.util.bytesearch` into a new `s1_patterns` module, vendor the pattern XMLs, select by detected arch+compiler (the `ProgramDecisionTree` analog). For **unconstrained** patterns (`.plt`-thunk prepatterns, plain prologue patterns) emit entries directly (subsumes/generalizes oracle 5 to all arches) + thunk facts + `setcontext` facts (ARM TMode → feeds `arm_markers`). For **constrained** (`after=defined`/`validcode=N`) patterns, document as a faithful loss (need a Listing+PseudoDisassembler). Strict superset of the current minimal port.

**`MachoFunctionStartsAnalyzer`** — *med / none (gates Mach-O) / lowest-effort non-ELF discovery pass.* (Ghidra: `Features/Base/.../MachoFunctionStartsAnalyzer.java`.)
- *Purpose:* discovers entries from the Mach-O `LC_FUNCTION_STARTS` load command (ULEB128-delta list), optionally PseudoDisassembler-vetted; also handles the DYLD shared-cache case.
- *Why-not-yet:* the pass is trivial but **kuna has no Mach-O bootstrap** (only `\x7fELF` triggers analysis).
- *Prereqs:* a Mach-O loader/bootstrap (the `object` crate parses Mach-O headers; `LC_FUNCTION_STARTS` may need direct load-command reading); the entry-commit recipe already exists.
- *Approach:* once a Mach-O bootstrap exists, an `s1_macho_funcstarts` pass: ULEB-decode the deltas relative to `__TEXT` base, emit entries (exactly like the ported eh_frame/entry passes). Drop the PseudoDisassembler vetting (let the decompiler validate).

**`AARCH64PltThunkAnalyzer`** — *med / high.* (Ghidra: `Processors/AARCH64/.../AARCH64PltThunkAnalyzer.java`.)
- *Purpose:* recognizes AArch64 PLT thunks by byte-pattern (`aarch64-pltThunks.xml`), creates thunk functions, tracks x17 (the PLT scratch reg) to point each thunk at its real import.
- *Why-not-yet:* the common case is likely already covered by the generic relocation-driven PLT/GOT pass (`elf_plt.rs`, commit `aca8b44`); only the pattern-matched (non-relocation) thunks remain.
- *Prereqs:* the `pltThunks.xml` resource (vendorable) + `elf_plt.rs` (EXISTS) + x17 tracking.
- *Approach:* first verify whether `.rela.plt` already names AArch64 PLT thunks via the generic path (likely redundant if so). The incremental piece is the byte-pattern + x17 fallback for stripped/hand-rolled thunks — frame as "extend `elf_plt.rs` with the AArch64 pattern/x17 fallback."

**`CreateThunkAnalyzer` + `ExternalEntryFunctionAnalyzer` (+ `ExternalLocation` model)** — *hard / medium.* (Ghidra: `Features/Base/.../function/CreateThunkAnalyzer.java`, `ExternalEntryFunctionAnalyzer.java`, `cmd/function/CreateThunkFunctionCmd`, `symbol/ExternalLocation`.)
- *Purpose:* `CreateThunkAnalyzer` turns one-instruction computed-jump stubs into THUNK functions forwarding their callee (resolving to an `ExternalLocation` for imports); `ExternalEntryFunctionAnalyzer` creates functions at ELF external-entry symbols where an instruction exists and nothing falls into it.
- *Why-not-yet:* documented ⛔ `thunk-model` — kuna's Database has no thunk relationship (functions are flat address→symbol) and no `ExternalLocation` type, and there is no Listing to spot single-jump stubs. The high-value ELF case (PLT imports) is already handled by `elf_plt.rs`.
- *Prereqs:* a Listing (to identify single-jump stubs and run `isThunk`); a real thunk/`ExternalLocation` model in the Database; for `ExternalEntryFunctionAnalyzer`, the "instruction already exists + not fallen-into" guard (needs a Listing).
- *Approach:* minimal interim already done for PLT (alias stub address → import name). Faithful port needs (a) a Listing for single-jump-stub detection and (b) a thunk/`ExternalLocation` Database model so a thunk forwards callee name+prototype. The `.plt`-thunk byte patterns from the FunctionStart item would mark the rest once the pattern engine lands. Low marginal value beyond PLT.

**`OperandReferenceAnalyzer` + `DataOperandReferenceAnalyzer`** — *med / medium / ⛔ infeasible-at-tier.* (Ghidra: `Features/Base/.../OperandReferenceAnalyzer.java`, `DataOperandReferenceAnalyzer.java`.)
- *Purpose:* walks every instruction/data operand reference in the disassembled Listing and creates strings (min len 5), pointers (relocation-guided), address/switch tables, and disassembles/creates subroutines at flow targets; overrides jumps-into-EXTERNAL as CALL_RETURN. `DataOperandReferenceAnalyzer` is the data-sourced restriction.
- *Why-not-yet:* Increment 4 made the explicit doc-only decision **not** to port it: no Listing/ReferenceManager at this tier, and every useful product is already delivered elsewhere — ASCII/Unicode strings == `s1-strings`; address/switch tables == inherited-S2 + `s1-addrtable`; subroutine creation == `s1-entry-disc`.
- *Prereqs:* a pre-decompile recursive-descent disassembler producing a real Listing+ReferenceManager+AddressTable — a whole new tier the project deliberately avoids.
- *Approach:* record as ⛔ infeasible-at-tier. The one decompiler-relevant idea (scalar→`.rodata`-string == `char*`) is blocked by the same printer/MapGlobals shadowing that disabled `s1-strings` (`ActionMapGlobals` is a stub, `s9_emit/coreaction_render.rs`).

**`FindNoReturnFunctionsAnalyzer` ("Non-Returning Functions - Discovered")** — *hard / medium / engine feedback pass.* (Ghidra: `Features/Base/.../FindNoReturnFunctionsAnalyzer.java`.)
- *Purpose:* flow-based discovery of non-returning functions (vs. the ported known-list `NoReturnFunctionAnalyzer`): scans call sites for indicators (function/data immediately after the call, INT3 padding, inconsistent fall-through refs, a subgraph reaching only noreturn callees), tallies per-target evidence, marks noreturn when ≥3 callers agree, then `CALL_RETURN`-overrides and runs `ClearFlowAndRepairCmd`.
- *Why-not-yet:* documented ⛔ `s1-noreturn-discovered` (Increment 1) — "needs pre-decompile listing/flow; not at this tier." The whole heuristic is cross-caller flow evidence over a populated Listing+ReferenceManager that doesn't exist until after decompilation.
- *Prereqs:* a Listing + ReferenceManager + `SimpleBlockModel` + `ClearFlowAndRepairCmd` — none at this tier.
- *Approach:* the kuna-native route is a **second-pass, decompiler-resident detector** (an engine feedback edge, not an analyzer): after decompiling all functions, find callees where every caller has unreachable/data fall-through after the call site, re-pin noreturn, re-decompile callers — the same decompile→inspect→override→re-decompile loop already built for format-string half-B (`s1_formatstring/apply.rs` + `IfcDecompile`). The flow-repair half is subsumed by the engine's dead-code elimination.

---

## Tier B — engine-inherited refinements (recorded, not scheduled)

The ported decompiler engine already performs the *core* of each of these; only a
narrow application-layer *refinement* (usually persistence/seeding, or a
multi-convention/segmented target that ELF never hits) is unported. **No
analyzer-tier pass is warranted** for any of these today; they are recorded so the
inventory is closed and so the one-or-two genuinely-missing engine refinements are
findable. (Logged 🟡 in `analysis-port-log.md`.)

| Analyzer | Inherited by | The unported refinement (if any) | Status |
|---|---|---|---|
| `ConstantPropagationAnalyzer` (+ per-CPU subclasses) | S3 SSA const/value-set | program-wide pointer/data markup from speculative const refs | ⛔ blocked (needs Listing+RefMgr; one idea blocked by MapGlobals shadowing) |
| `DecompilerSwitchAnalyzer` | S2 `recover_jump_tables` / `jumptable.rs` | **roadmap #9**: POST-TYPING multistage jump-table re-recovery | ⬜ deferred **engine** task |
| `DecompilerFunctionAnalyzer` ("Param ID") | S4 proto/param + S5 types + S7 locals | persistent commit-and-re-decompile over the whole program | ⛔ needs durable cross-function proto store; PE-default-only |
| `DecompilerCallConventionAnalyzer` ("Call Convention ID") | engine applies the default convention | convention *disambiguation* on multi-convention cspecs | ⛔ canAnalyze false on single-convention System V x86-64 |
| `StackVariableAnalyzer` ("Stack") | S7 stack-local recovery | inject *external* stack-local defs into the engine | = the DWARF subtask-3 write-back (an engine change) |
| `FunctionAnalyzer` ("Subroutine References") | partly by `s1-entry-disc`/`s1-eh-frame` | CALL-reference-driven entry discovery | ⛔ needs a pre-decompile ReferenceManager |
| `FunctionAnalyzer` (`util.state` interface) | S3 dataflow | — (it is the SPI for `ResultsState`, not an analyzer) | n/a |
| `SharedReturnAnalyzer` / `SharedReturnJumpAnalyzer` | S2 tail-call flow handling | `assumeContiguousFunctions` cross-jump function discovery (honor pspec keys) | ⬜ folds into entry-discovery track |
| `SegmentedCallingConventionAnalyzer` | cspec convention selection | RET-opcode→convention 4-way map | ⛔ inapplicable to ELF (needs a segmented 16-bit address space) |
| `X86FunctionPurgeAnalyzer` | engine infers `extrapop` during decompilation | forced-purge override (Win32 stdcall) | ⛔ Linux ELF x86 is cdecl (purge=0) |
| `RustStringAnalyzer` | detection inherited (`s1_sourcelang`) | the str-slice split itself | ⛔ infeasible-at-tier (needs interior-ref ReferenceManager) |
| `MicrosoftDemanglerAnalyzer` | the ported demangle pass (`s1_demangle`) | wire `msvc-demangler` crate behind a `?`-prefix gate | ⬜ **lowest-friction backlog item** (see Tier F) |

> **Two of these are genuinely actionable** despite being "inherited," and they
> recur from Tier A: (1) roadmap #9 multistage jump-table re-recovery is a
> deferred *engine* task (`Override::queryMultistageJumptable`, the W4 SEAM at
> `jumptable.rs:3878-3881`, + a second `recover_jump_tables` after
> `ActionStartTypes` in `universalaction.rs`); (2) the StackVariable write-back ==
> the still-deferred DWARF subtask-3 stack-local *injection* seam. The
> `MicrosoftDemanglerAnalyzer` wiring is tracked in Tier F because it needs a new
> crate dependency, not an inheritance.

---

## Tier C — per-CPU processor analyzers (`Processors/*`, 31)

All but four (`HCS12ConventionAnalyzer`, `eBPFSyscallAnalyzer`, the Hexagon
non-CP passes, the bespoke PIC/JVM families) subclass
`ConstantPropagationAnalyzer` and override `flowConstants(...)` with a per-arch
`ContextEvaluator` driven over a `SymbolicPropogator` across a **disassembled
Listing**.

**SHARED PREREQUISITE for the whole CP-subclass group:** a pre-decompile Listing +
`SymbolicPropogator`/`VarnodeContext` + `ContextEvaluator` + `ReferenceManager` at
kuna's analysis tier — **none of which exist**. Build this *once, generically*,
not per-arch. Two seams the ARM/MIPS work already added are reusable: **ContextPaint**
(context-register paint, used for TMode) and **TrackedRegFact** (tracked-register
value seed, used for MIPS t9). For most arches the high-value port is just the
per-arch register **SEED** (gp/r2/r12/o7/ra), not the full listing-time propagator —
the decompiler's own `ActionConstbase`/const-prop recovers the rest once the seed
is installed.

| Analyzer | Per-arch hook | Portable now? | Difficulty | ELF |
|---|---|---|---|---|
| `RISCVAddressAnalyzer` | gp seed from `__global_pointer$` (unconditional, safe) | **YES** — `TrackedRegFact` | easy | high |
| `NDS32Analyzer` | gp seed from `_SDA_BASE_` (unconditional, safe) | **YES** — `TrackedRegFact` | easy | low |
| `MipsSymbolAnalyzer` | MIPS16 LSB symbols → re-create at `addr&~1` + `ISA_MODE=1` paint | **YES** — ContextPaint + symbol seam | med | medium |
| `PowerPCAddressAnalyzer` | r2-TOC seed (PPC64) | **r2 seed: yes**; rest needs propagator | hard | high |
| `MipsAddressAnalyzer` | t9=entry seed (**DONE**); gp-direct/dual-instr/switch | t9 ported; rest unsafe/needs propagator | hard | high |
| `X86Analyzer` | LEA→DATA-ref markup; out-of-mem ref suppression | mostly subsumed by engine const-prop | hard | high |
| `ArmAnalyzer` | TMode paint (**approximated** by `arm_markers.rs`); BX/LR flow-override; switch recovery | paint done; flow-override+switch need propagator+FlowOverride | xhard | high |
| `SparcEarlyAddressAnalyzer` | o7-restore-in-delay-slot → CALL_RETURN | needs Listing/pcode + a new FlowOverride fact | med | high |
| `SparcAnalyzer` | the Early flow-override + full CP body | needs propagator | hard | high |
| `SH4AddressAnalyzer` / `SH4EarlyAddressAnalyzer` | r12 GOT-base propagation; bsrf/braf operand fix | r12 is a *propagated* const, not a static symbol → needs propagator | med | medium |
| `Motorola68KAnalyzer` | `pea`-address DATA-ref; switch (default-off) | pea markup subsumed by engine const-prop | hard | low |
| `LoongsonAnalyzer` | ra return-vs-branch → CALL_RETURN | needs propagator + FlowOverride fact | hard | low |
| `MipsPreAnalyzer` | `PAIR_INSTRUCTION_FLAG` paint for unaligned ld/st pairs | needs listing-time instruction scan | hard | medium |
| `ToyAnalyzer` | none (pass-through binding to the Toy test processor) | covered for free by the generic CP pass | easy | none |
| `HCS12ConventionAnalyzer` | terminal RET-mnemonic → convention | needs Listing + convention-selection sub-fact | med | low |
| `AARCH64PltThunkAnalyzer` | (in Tier A — PLT thunks + x17) | extend `elf_plt.rs` | med | high |
| `eBPFSyscallAnalyzer` | helper-id → `FunctionDefinition` from a built-in table | **YES** — parallels the ported libc-proto pass | med | low |
| `Pic12Analyzer` | bespoke W/file-reg + bank/page walker (Harvard) | needs Listing + bespoke tracker | hard | none |
| `Pic16Analyzer` | STATUS/PCLATH/BSR bank-select CP subclass | needs propagator + PIC context | hard | none |
| `Pic17c7xxAnalyzer` | bespoke ~1160-line W/file-reg + flow-graph walker | from-scratch subsystem | xhard | none |
| `Pic18Analyzer` | bespoke file-reg + banked-flow walker | needs Listing + bespoke tracker | hard | none |
| `Pic24DInitAnalyzer` | parse `.dinit` records → materialize init data | needs writable synthesized memory | med | low |
| `PicSwitchAnalyzer` | PIC skip-instr + PCL-modify switch recovery | needs Listing + PseudoDisassembler + switch fw | hard | none |
| `HexagonAnalyzer` | VLIW packet-aware CP | needs propagator + a packet-semantics model | xhard | medium |
| `HexagonPrologEpilogAnalyzer` | alloc/dealloc-frame helpers → inline / call-fixup | byte-matcher; needs an inline-function fact | med | medium |
| `HexagonThunkAnalyzer` | allocframe+R28+jumpr thunk pattern → thunk fn | byte-matcher; needs a thunk-relationship fact | med | medium |
| `HexagonUnsupportSemanticAnalyzer` | bookmark intra-packet predicate-use (diagnostic) | diagnostic-only; no decompiler payoff | med | low |
| `JavaAnalyzer` / `AbstractJavaAnalyzer` / `JvmSwitchAnalyzer` | `.class`-file loader+markup + JVM switch recovery | needs a whole `.class` front-end | subsystem | none |

### Tier C — notable detail

- **`RISCVAddressAnalyzer` / `NDS32Analyzer`** — the two *unblocked* easy pickups. Both are unconditional gp seeds (RISC-V `__global_pointer$`, NDS32 `_SDA_BASE_`) on the existing `TrackedRegFact` seam; the `COPY #gp→gp` form is safe (unlike MIPS PIC gp). Model on `s1_loader/mips_markers.rs`. RISC-V is high-ELF; NDS32 is rare (priority-deferred only).
- **`MipsSymbolAnalyzer`** — portable now: a pure symbol+context transform (executable symbols with LSB set → symbol at `addr&~1` + `ISA_MODE=1` paint). Good first MIPS pickup; the ARM TMode analog was done first because ARM/Thumb is far more common. (`ISA_MODE` is a *decode-mode* register exactly like TMode, despite the Increment-8 note scoping it out.)
- **`MipsAddressAnalyzer`** — the decompiler-critical t9 PIC seed is **already ported** (`mips_markers.rs`). Residual: (a) gp-direct seed for non-PIC objects — deliberately not done (an unconditional `COPY #_gp→gp` poisons PIC functions that recompute gp from t9; needs Ghidra's lazy-`unknownValue` propagator to do safely); (b) `lui/addiu` dual-instruction markup; (c) ISA_MODE; (d) switch recovery (default-off).
- **`SparcEarlyAddressAnalyzer`** — the most tractable SPARC item: for each call with a delay slot, lift the delay-slot pcode, check for a non-constant o7 write, emit a **new `FlowOverride(CALL_RETURN)` fact** at the call. Needs a minimal listing/pcode-at-analysis-tier + the new FlowOverride fact kind (shared with ARM/LoongArch).
- **`eBPFSyscallAnalyzer`** — self-contained and portable: a static helper-id→`FunctionDefinition` table (analogous to the ported `ApplyDataArchiveAnalyzer` libc table) applied to functions in the eBPF `syscall` address space; unknown ids → `bpf_undef_0xNN`. No deep blocker; needs the vendored helper table + syscall-space recognition.
- **The PIC family (Pic12/16/17c7xx/18/24DInit/PicSwitch)** — Harvard-architecture microcontroller listing walkers with bank/page context. Effectively zero overlap with kuna's ELF mission; defer indefinitely (inventory completeness). PIC firmware is almost never an ELF.
- **The JVM family (Java/AbstractJava/JvmSwitch)** — blocked on an entire `.class`-file loader (`object::File` is ELF/PE/Mach-O/COFF, not `.class`). The install-side seams exist; the missing 99% is the class-file front-end + the `constantPool` address space. Out of scope until/unless kuna grows a JVM loader.

---

## Tier D — non-ELF executable formats (Mach-O / PE / COFF / PEF / .NET)

These have real decompilable code, but every one is gated behind a **format
loader/bootstrap that does not exist** (kuna's only bootstrap is `\x7fELF` →
`bootstrap_from_elf`). The `object` crate parses most of these formats' headers;
the missing pieces are (a) a per-format bootstrap mapping sections→vaddrs into an
`ObjectLoadImage`, and (b) the bespoke struct-markup recipes (which mostly need a
Listing kuna lacks — only the *facts*, entries/symbols/strings, can be salvaged).

| Analyzer | Format | Decompiler-useful product | Difficulty | ELF |
|---|---|---|---|---|
| `PortableExecutableAnalyzer` | PE/COFF | import-name symbols + header data symbols | subsystem | none |
| `CoffAnalyzer` | COFF object/image | function/data symbols from the COFF symbol table | hard | low |
| `CoffArchiveAnalyzer` | `.lib`/ar archive | per-member COFF symbols (needs a container model) | hard | low |
| `MachoAnalyzer` (`MachoBinaryAnalysisCommand`) | Mach-O | symbols/entries from `LC_SYMTAB`/`LC_FUNCTION_STARTS`; rest is cosmetic | subsystem | none |
| `MachoFunctionStartsAnalyzer` | Mach-O | **entries** (`LC_FUNCTION_STARTS`) — the high-value slice | med | none (gates Mach-O) |
| `MachoConstructorDestructorAnalyzer` | Mach-O | `__constructor`/`__destructor` pointees as entries | med | none |
| `CFStringAnalyzer` | Mach-O | `__cfstring` literals as strings + `cf_` symbols | med | none |
| `TestAnalyzer` (`__unwind_info`, dev prototype) | Mach-O | none (scratch stub — do not port) | easy | none |
| `PefAnalyzer` ("PEF Indirect Addressing") | classic-Mac PEF | r2/TOC refs (listing/operand-level) | hard | none |
| `PefDebugAnalyzer` ("PEF Debug") | classic-Mac PEF | function names from trailing debug records | hard | none |
| `CliMetadataTokenAnalyzer` | .NET/CLI managed PE | symbolic method/type/field names in CIL | subsystem | none |
| `MingwRelocationAnalyzer` | MinGW PE | applies `_pei386_runtime_relocator` pseudo-relocs | hard | none |

### Tier D — notable detail

- **`MachoFunctionStartsAnalyzer`** — the simplest faithful port in the whole non-ELF set (pure ULEB-delta decoding feeding the existing entry path), blocked only by the missing Mach-O bootstrap. It is the natural first step of any Mach-O wave. (Appears in Tier A too as the gating item.)
- **`CFStringAnalyzer`** — reuses kuna's existing strings/symbols commit path; only the `__cfstring` struct-walk (16/32-byte stride, follow data-ptr+len) is new. Loader-blocked only.
- **`PortableExecutableAnalyzer` / `CoffAnalyzer`** — the structure-by-structure markup recipes are bespoke, but the genuinely useful product is import-name / COFF symbol-table symbols (mirroring how PLT/GOT names are recovered for ELF). COFF symbols can be MSVC-mangled → ties to `MicrosoftDemanglerAnalyzer` (Tier F).
- **`PefAnalyzer` / `PefDebugAnalyzer` / `CliMetadataTokenAnalyzer` / `MingwRelocationAnalyzer`** — all need pre-decompile listing/operand models, managed-code processor specs, or in-place memory rewriting that kuna's read-only fact-producing seam does not support. Obsolete/managed/MinGW-specific; lowest priority within Tier D.

---

## Tier E — non-ELF container / firmware / filesystem formats (~28)

**These produce no decompilable code of their own** — they are container,
firmware-wrapper, image, or filesystem annotators. Every one extends
`FileFormatAnalyzer` (the load-bearing base, below) and mutates a Program's
Listing/Memory/SymbolTable, which kuna does not have. For nearly all of them the
"port" is **⛔ do-not-port at this tier**: the contained payloads (kernels,
embedded DEX, etc.) are extracted by separate tools, and the struct/fragment/comment
markup has nowhere to go in a C-emitting decompiler. Recorded for completeness.

**`FileFormatAnalyzer` (abstract base for the whole cluster)** — *subsystem / none.* (Ghidra: `Features/FileFormats/.../file/analyzers/FileFormatAnalyzer.java`.) This is the structural reason none of Tier E is portable: every member is a Listing-mutating markup pass built on this base (`createData`/`createString`/`createFragment`/`setPlateComment`/`createFunction`). Porting it == adding a pre-decompile Program/Listing model — a subsystem-sized effort with **no decompiler payoff** for non-code container formats. Without it, only the rare *facts* (entries from function-starts, symbols/strings from CFString) salvage into `AnalysisOutput`.

| Analyzer | Format | Why ⛔ at this tier | Difficulty | ELF |
|---|---|---|---|---|
| `EmbeddedMediaAnalyzer` | GIF/PNG/JPEG/WAVE/… blobs in memory | typed media data-units; never reach C | easy | none |
| `AppleSingleDoubleAnalyzer` | AppleSingle/Double resource container | non-executable container | med | none |
| `DyldCacheAnalyzer` | iOS/macOS DYLD shared cache | heaviest format; needs Mach-O stack + cache parser | subsystem | none |
| `DmgAnalyzer` | Apple DMG (UDIF disk image) | disk-image container; header-only annotation | easy | none |
| `iBootImAnalyzer` | Apple iBootIm boot-logo image | graphics container; header-only | easy | none |
| `Img2Analyzer` / `Img3Analyzer` / `Apple8900Analyzer` | Apple IMG2/IMG3/8900 firmware | firmware-container annotation; nested code extracted elsewhere | easy | none |
| `iOS_Analyzer` | iBoot/LLB/iBSS/iBEC bring-up | brittle hard-coded offsets; **disabled in Ghidra** | hard | none |
| `iOS_FixupArmSymbolsAnalyzer` | Apple ARM boot ROMs | depends on Ghidra "pinned-symbol" model kuna lacks | med | none |
| `iOS_KextStubFixupAnalyzer` | iOS kext stubs | needs Mach-O + a **multi-program project**; disabled in Ghidra | subsystem | none |
| `BinaryPropertyListAnalyzer` | Apple bplist00 | serialized-data format; disabled in Ghidra | med | none |
| `LzssAnalyzer` | Apple LZSS firmware | compressed container; real value is decompression (elsewhere) | easy | none |
| `DexHeaderFormatAnalyzer` | Android DEX/CDEX | root of the Dalvik subsystem (loader + DexHeader) | subsystem | none |
| `DexMarkupInstructionsAnalyzer` | DEX | operates on a disassembled Dalvik Listing | hard | none |
| `DexMarkupDataAnalyzer` | DEX | post-processes Listing Data (defined-data iteration) | med | none |
| `DexMarkupSwitchTableAnalyzer` | DEX | Dalvik switch recovery over a Listing | hard | none |
| `DexExceptionHandlersAnalyzer` | DEX | triggers pre-decompile disassembly of handler regions | med | none |
| `DexCondenseFillerBytesAnalyzer` | DEX | cosmetic padding cleanup | easy | none |
| `OatHeaderAnalyzer` | Android OAT (rides in ELF) | large multi-version OAT struct library; markup needs Listing | subsystem | low |
| `OatExecAnalyzer` | OAT `oatexec` | quick-method headers → entries; **disabled+TODO in Ghidra** | hard | low |
| `OdexHeaderFormatAnalyzer` | legacy Android ODEX | container around embedded DEX (gated behind DEX) | med | none |
| `ArtAnalyzer` | Android ART image | serialized object graph, not code | subsystem | none |
| `VdexHeaderAnalyzer` | Android VDEX | container around embedded DEX | subsystem | none |
| `BootImageAnalyzer` | Android boot/recovery/vendor_boot | nested kernel extracted elsewhere; disabled by default | med | none |
| `AndroidBootLoaderAnalyzer` | Android bootloader image | partition directory; payloads elsewhere | easy | none |
| `FBPK_Analyzer` | Android Fastboot Pack | OEM firmware container | med | none |
| `Ext4Analyzer` / `NewExt4Analyzer` | ext4 filesystem | filesystem (files via FSRL layer); NewExt4 also needs multi-program | med | none |
| `CramFsAnalyzer` | CramFS filesystem | compressed-ROM filesystem | med | none |
| `DtbAnalyzer` / `FdtAnalyzer` | DTB/DTBO / Flattened Device Tree | hardware-description data, not code | easy/med | none |

### Tier E — the few salvageable facts

If a Mach-O / DEX / OAT loader is ever built, these specific rows yield real
`AnalysisOutput` facts and become worth doing:
- **Mach-O:** `MachoFunctionStartsAnalyzer` (entries — see Tier D), `MachoConstructorDestructorAnalyzer` (ctor/dtor pointees as entries), `CFStringAnalyzer` (strings + `cf_` symbols).
- **OAT:** `OatExecAnalyzer` (each `OatQuickMethodHeader` bounds a real function → entries) — but Ghidra itself leaves the function-creation TODO'd and the analyzer disabled.
Everything else in Tier E is cosmetic markup with no path into decompiled C; record-only.

---

## Tier F — OS / runtime / language subsystems (~14)

The high-value, high-effort frontier. These recover names/types/strings for whole
language runtimes and debug-info formats. Several are **the single most
decompilation-relevant non-ELF items** (PDB is "the Windows DWARF"; the Go pclntab
parser turns stripped Go from `sub_xxxx()` soup into named functions). They split
into "subsystem" (a whole parser + applicator) and a couple of low-friction wins.

| Analyzer | Subsystem | Decompiler-useful product | Difficulty | ELF |
|---|---|---|---|---|
| `MicrosoftDemanglerAnalyzer` | MSVC name demangling | demangled symbol names (slots into `s1_demangle`) | med | medium |
| `SwiftDemanglerAnalyzer` | Swift name demangling | demangled Swift names (Swift-on-Linux relevance) | hard | medium |
| `GolangSymbolAnalyzer` (+ ~90 support files) | Go pclntab / RTTI / register-ABI | **function names + entries** (Phase 1) then signatures | subsystem | high |
| `GolangStringAnalyzer` (+ GoString/GoSlice) | Go string/slice literals | string literals (data-segment-scan subset feasible) | hard | high |
| `RustStringAnalyzer` | Rust packed `&str` split | str-slice cuts (split infeasible-at-tier) | med | high |
| `PdbUniversalAnalyzer` | PDB (pure-Java/portable) | names + typed prototypes + locals (the Windows DWARF) | subsystem | low |
| `PdbAnalyzer` (MSDIA/XML) | PDB via MS DIA SDK | — (Windows-only native dep; non-target) | subsystem | none |
| `FidAnalyzer` (+ FidDB/hasher/extent) | Function-ID fingerprinting | names of statically-linked library functions | subsystem | medium |
| `SwiftTypeMetadataAnalyzer` (+ swift format) | Swift reflection metadata | Swift type/class names (mostly Mach-O) | subsystem | low |
| `ObjcMessageAnalyzer` (+ objc1/2 metadata) | ObjC dynamic dispatch | `[Class selector]` call resolution (Mach-O-only) | subsystem | low |
| `ObjcTypeMetadataAnalyzer` (+ objc1/2) | ObjC static metadata | ObjC class/method names (Mach-O-only) | subsystem | low |
| `PEExceptionAnalyzer` | MSVC C++ EH / SEH (PE) | handler functions as entries + EH data | hard | none |
| `RttiAnalyzer` | MSVC RTTI (PE) | C++ class names + vftable labels | hard | low |
| `TEBAnalyzer` | Windows TEB (PE) | named `fs:`/`gs:` TEB-field accesses | med | none |
| `WindowsResourceReferenceAnalyzer` | Windows .rsrc refs (PE) | resource-ID xrefs from WinAPI calls | hard | none |
| `PropagateExternalParametersAnalyzer` | Win32 push-arg naming (PE) | named/typed call args (subsumed by engine param prop) | hard | low |

### Tier F — detail (the items worth scheduling)

**`MicrosoftDemanglerAnalyzer`** — *med / medium / the lowest-friction item in the entire backlog.* (Ghidra: `Features/MicrosoftDemangler/.../MicrosoftDemanglerAnalyzer.java`.)
- *Purpose:* the MSVC counterpart of `GnuDemanglerAnalyzer` — demangles `?name@@...` MSVC names and applies the readable name/signature/convention/param types.
- *Why-not-yet:* kuna's demangle pass only wires Itanium + Rust; the MSVC demangler crate isn't integrated, and MSVC symbols are rare in the ELF corpus kuna validates against.
- *Prereqs:* the `msvc-demangler` crate (MIT, the natural analog to `cpp_demangle`/`rustc-demangle`). **No PE loader required** — it operates on symbol-name strings, so it runs on any input whose symbols carry MSVC mangling (incl. COFF objects the `object` crate already parses).
- *Approach:* extend `s1_demangle` to also try `msvc-demangler` on `?`-prefixed names, applying through the same `find_create_scope`/`register_symbol` rename seam. Purely additive; could land without a PE loader.

**`GolangSymbolAnalyzer`** — *subsystem / high / the flagship Go item.* (Ghidra: `Features/Base/.../GolangSymbolAnalyzer.java` + ~90 support files under `util/bin/format/golang/`.)
- *Purpose:* follows `runtime.firstmoduledata` to recover every Go function name (from pclntab `funcnametab`/`ftab`), signatures (RTTI methods + a bundled runtime API-snapshot JSON), Go RTTI types, no-return flags, source files, duff/gcWriteBarrier fixups, closure/RTTI propagation, and a synthetic G/M block. Turns stripped Go into named, typed functions.
- *Why-not-yet:* ⛔ in the port log (Go is niche; the single largest non-ELF-format subsystem); kuna scoped its Go work to no-return-list selection (Increment 14).
- *Prereqs:* **Phase 1** needs only a Go pclntab parser (GoPcHeader magic 0xfffffffb/fa/f0/f1 → GoModuledata → funcnametab + ftab → GoFunctabEntry → GoName) — a pure read-only blob parse, **no Listing**. **Phase 2** adds the Go register-ABI allocator + RTTI/api-snapshot. **Phase 3** (duff/gcWriteBarrier/closure/RTTI-propagation) needs a SymbolicPropagator + ReferenceManager (same wall as `FindNoReturnFunctionsAnalyzer`).
- *Approach:* **Phase 1 (high value, feasible NOW):** new pass gated on Go detection (add Golang to `s1_sourcelang` via `.note.go.buildid` / `.gopclntab` / `runtime.pclntab`), parse pclntab → emit function-name **symbols** + **entries** (each funcAddr is a guaranteed function start — a strong entry oracle complementing eh-frame). Same shape as the DWARF/eh-frame passes. Phase 2 via the prototype seam; Phase 3 deferred.

**`GolangStringAnalyzer`** — *hard / high.* (Ghidra: `Features/Base/.../GolangStringAnalyzer.java`.)
- *Purpose:* recovers Go string/slice literals (Go `char[]` has no NUL terminator; length comes from the `{ptr,len}` struct).
- *Why-not-yet:* whole Go subsystem ⛔; the higher-value of its two paths (in-function refs / register-pair inline strings) is structurally blocked at kuna's tier.
- *Prereqs:* depends on GolangSymbol infra (GoRttiMapper string-range helpers); the data-segment-scan path needs only the load image + ptr size.
- *Approach:* **feasible subset (data-segment scan, no Listing):** once the Go section model exists, scan `.rodata`/`.noptrdata` aligned to ptrSize for 16-byte string structs `{ptr,len}` / 24-byte slices `{ptr,len,cap}` whose pointer lands in the string-data range and whose `len` bytes are valid — emit `strings` facts (the `s1-strings` recipe). The Increment-12 printer change means these render as literals. Defer the in-function reference + register-pair inline-string paths.

**`SwiftDemanglerAnalyzer`** — *hard / medium / the only ELF-relevant Swift piece.* (Ghidra: `Features/Swift/.../SwiftDemanglerAnalyzer.java`.)
- *Purpose:* demangles Swift `$s.../_$s...` names. **Ghidra shells out** to a native `swift-demangle` binary — it has no pure demangler.
- *Why-not-yet:* Swift is mostly macOS/Mach-O; Ghidra's impl isn't self-contained (external Swift install), so there is no line-faithful pure port — a crate-dependency decision is required first.
- *Prereqs:* a pure-Rust Swift-demangle crate (preferred) **or** shelling out to a host `swift-demangle` (a non-portable runtime dep); Swift detection added to `s1_sourcelang`.
- *Approach:* mirror `s1-demangle`: detect Swift mangling (`$s`/`_$s`/`$S`) gated on Swift detection, bind a Rust Swift-demangle crate, feed through the existing rename seam. The shell-out route should be the fallback, not the default.

**`PdbUniversalAnalyzer`** — *subsystem / low / the most decompilation-relevant non-ELF item (the Windows DWARF).* (Ghidra: `Features/PDB/.../PdbUniversalAnalyzer.java`.)
- *Purpose:* the platform-independent PDB consumer — locates the matching `.pdb`, parses MSF/TPI/IPI/symbol streams, applies function names+signatures, globals/locals, UDTs, and source-line info. The Windows equivalent of the ported `DWARFAnalyzer`.
- *Why-not-yet:* massive subsystem — needs a PDB parser + type applicator + a **side-file (.pdb) discovery seam** + a PE loader. No part exists.
- *Prereqs:* a PE loader (PDB is applied onto PE sections/RVAs); a side-file mechanism (the analysis seam today sees only the input bytes); the `pdb` crate (MIT/Apache, covers MSF/TPI/symbol streams) + a PDB-type→`TypeFactory` mapping; section-contribution/RVA resolution.
- *Approach:* mirror the DWARF port (names + typed prototypes + locals) sourced from a PDB via the `pdb` crate; the commit recipes already exist. The hard parts are the side-file discovery seam and the type mapping.

**`PdbAnalyzer` (MSDIA/XML)** — *subsystem / none / deliberate non-target.* Port `PdbUniversalAnalyzer` instead; this one depends on the Windows-only MS DIA SDK (or a Ghidra-specific PDB-XML file) and is inherently non-portable. Record-only.

**`FidAnalyzer`** — *subsystem / medium.* (Ghidra: `Features/FunctionID/.../FidAnalyzer.java` + FidDB/hasher/extent.)
- *Purpose:* identifies known statically-linked library functions by a three-part hash quad (medium/full/specific), looked up in pre-built `.fidb` databases, applying the matched name.
- *Why-not-yet:* ⛔ — two heavy prereqs (a **byte-exact** hasher to reuse Ghidra's `.fidb` files, and an external DB-file format) plus a pre-decompile Listing of full function bodies. Its niche partly overlaps the shipped library-prototype seeding.
- *Prereqs:* (1) a byte-exact `FidHashQuadImpl` + `MessageDigestFidHasher` + `FunctionExtentGenerator` over a kuna code-unit model; (2) a `.fidb` reader (DB shipped separately, not vendored); (3) a Listing with full function bodies.
- *Approach:* faithfulness is critical — any deviation in code-unit ordering/operand masking makes existing `.fidb` files useless. A pragmatic **kuna-native alternative** is a simpler content-hash-of-bytes fingerprint over a built-in libc set (cf. `ApplyDataArchiveAnalyzer`), trading Ghidra-DB compatibility for tractability.

**`SwiftTypeMetadataAnalyzer` / `ObjcMessageAnalyzer` / `ObjcTypeMetadataAnalyzer`** — *subsystem / low.* All overwhelmingly Mach-O/Apple-platform; gated behind a Mach-O loader + a new struct-type-markup commit facility kuna does not have (it only installs `char[]` strings + prototypes). `ObjcMessageAnalyzer` additionally needs a ReferenceManager + CALL_OVERRIDE installation — though the *override-via-decompile* shape exists (format-string half-B). Tackle as one Objective-C / one Swift wave *after* Mach-O; near-zero ELF payoff.

**The MicrosoftCodeAnalyzer PE group** (`PEExceptionAnalyzer`, `RttiAnalyzer`, `TEBAnalyzer`, `WindowsResourceReferenceAnalyzer`, `PropagateExternalParametersAnalyzer`) — all gated behind a PE loader + Windows-specific models (MSVC EH structs, RTTI validators + an MSVC demangler, a synthetic-memory-block commit API for TEB, a .rsrc parser + ReferenceManager, x86 push-counting over a Listing). `PropagateExternalParametersAnalyzer`'s effect is largely subsumed by the engine's inherited param propagation. Pure Windows-runtime metadata; out of scope.

---

## Tier G — framework / abstract / non-analyzer infra (11)

Not program analyzers — interfaces, abstract bases, drivers, and util plumbing
that happen to match the `*Analyzer` class-name search. kuna re-expresses the few
that matter (the analyzer contract, the demangle/headless drivers) idiomatically
in Rust. **Nothing to "do" beyond recording them** so the 142 count is closed.

| Class | What it really is | Disposition |
|---|---|---|
| `Analyzer` (interface) | the ExtensionPoint SPI (`canAnalyze`/`added`/options/priority) | re-expressed as kuna's `AnalysisPass` trait; record-only |
| `AbstractAnalyzer` | base impl (bookkeeping + `runParallelAddressAnalysis`) | maps to per-pass metadata; parallel fan-out tied to Ghidra threadpool/Address model; record-only |
| `AbstractBinaryFormatAnalyzer` | adapter wrapping a `BinaryAnalysisCommand` | no-logic adapter over a command framework + mutable Program kuna lacks; record-only |
| `AbstractDemanglerAnalyzer` | shared GNU/MSVC demangle driver loop | concrete functionality already shipped in `s1_demangle`; the externals-last/skipSymbol heuristics are worth cross-checking; record-only |
| `AbstractJavaAnalyzer` | JVM-classfile markup base | wholly outside kuna's native-binary scope (needs a `.class` loader + constantPool space); record-only |
| `SkeletonAnalyzer` | copy-paste plugin template (`added()` returns false) | documentation stub; nothing to port |
| `HeadlessAnalyzer` | the `analyzeHeadless` driver/orchestrator (not an Analyzer) | kuna has its own driver (`kuna` binary + kuna-console); record-only |
| `FunctionAnalyzer` (`util.state` interface) | value-state callback SPI for `ResultsState` | inherited by S3 dataflow; record-only (also in Tier B) |
| `MySwitchAnalyzer` (`util.state.analysis`) | experimental switch-resolution demo on `ResultsState` | superseded by the ported-but-disabled `AddressTableAnalyzer`; record-only |
| `JitDataFlowBlockAnalyzer` | per-block dataflow inside the **p-code JIT emulator** (Framework/Emulation) | a subsystem kuna doesn't port (decompiler+SLEIGH only); "Analyzer" name is incidental; record-only |

---

## Recommended do-next ordering (across the whole remaining backlog)

Ordered for **payoff-per-effort on real ELF binaries**, front-loading the unblocked
easy wins and the one shared prerequisite that unlocks a whole tier.

**Phase 1 — unblocked ELF wins (no new prerequisite; days each):**
1. **`RISCVAddressAnalyzer`** (gp seed) — easy, high, zero blockers; the `mips_markers.rs` recipe verbatim. *The single best first pickup.*
2. **`_INIT_/_FINI_` array-element naming** — easy, medium; pure additive byte-reads on the existing `s1_entry` oracle-2 (+ `DT_PREINIT_ARRAY`).
3. **`MicrosoftDemanglerAnalyzer`** — med, medium; wire `msvc-demangler` into `s1_demangle` behind a `?`-prefix gate. No PE loader needed; lowest-friction item overall.
4. **`MipsSymbolAnalyzer`** (MIPS16 ISA_MODE) — med, medium; a pure symbol+ContextPaint transform.
5. **`PowerPCAddressAnalyzer`** (r2-TOC seed only) — med, high; new `ppc_markers.rs` on the tracked-reg seam.
6. **`NDS32Analyzer`** (gp seed) — easy but low-traffic; bundle with the gp-seed family if convenient.

**Phase 2 — moderate ELF wins (one focused module each):**
7. **`GccExceptionAnalyzer` Tier-1** (`.eh_frame_hdr` + LSDA landing-pads) — hard, high; extend the existing `s1_entry` CIE/FDE walk to recover catch-handler entries.
8. **`DWARFAnalyzer` line/source-map half** — med, medium; gimli line-rows → a `source_lines` fact + an `s9_emit` `// file:line` comment hook (also unblocks the printer comment channel for other items).
9. **`AARCH64PltThunkAnalyzer`** — med, high; first confirm the generic `elf_plt.rs` already covers relocation-driven AArch64 thunks, then add the pattern/x17 fallback.

**Phase 3 — the two big shared prerequisites (each unlocks a cluster):**
10. **The byte-pattern engine** (`ghidra.util.bytesearch` + vendored `patterns/*.xml`) → the full **`FunctionStart*`** family (Tier A), generalizing entry discovery to all arches; also feeds Hexagon thunk/prolog byte-matchers and `EmbeddedMediaAnalyzer`'s magic-scan. *Biggest entry-discovery win.*
11. **The Go pclntab parser** → **`GolangSymbolAnalyzer` Phase 1** (names + entries) + **`GolangStringAnalyzer`** data-segment subset. High ELF payoff (Go binaries are common ELF targets); pure blob-parse, no Listing.

**Phase 4 — engine feedback edges (touch the action pipeline; gate carefully):**
12. **Roadmap #9** multistage jump-table re-recovery (`Override::queryMultistageJumptable` + a post-`ActionStartTypes` `recover_jump_tables`) — the deferred engine half of `DecompilerSwitchAnalyzer`.
13. **DWARF subtask-3 stack-local *injection* seam** — unblocks `StackVariableAnalyzer`-style external-local seeding.
14. **`FindNoReturnFunctionsAnalyzer`** as a decompile→inspect→override→re-decompile pass (the format-string half-B pattern, applied to noreturn).
15. **`FormatStringAnalyzer` non-constant path** — extend the existing half-B loop with MULTIEQUAL/phi following + printf-wrapper handling (keep off by default).

**Phase 5 — the SymbolicPropagator buildout (one generic engine, then the per-CPU CP-subclasses):**
16. Build the analysis-tier **Listing + SymbolicPropagator + ContextEvaluator + a `FlowOverride` fact kind** once, generically. Then port the per-arch CP subclasses cheaply: `SparcEarlyAddressAnalyzer`/`SparcAnalyzer`, `LoongsonAnalyzer`, `X86Analyzer`, `ArmAnalyzer` (flow-override + switch halves), `Motorola68KAnalyzer`, `SH4*`, `MipsPreAnalyzer`, `eBPFSyscallAnalyzer` (independent of the propagator). `ToyAnalyzer` comes for free.

**Phase 6 — non-ELF format loaders (largest lifts; do per-format, salvage facts first):**
17. **Mach-O bootstrap** → `MachoFunctionStartsAnalyzer` (entries) → `MachoConstructorDestructorAnalyzer` + `CFStringAnalyzer` → (later) the ObjC/Swift waves.
18. **PE/COFF bootstrap** → import/COFF symbols → `RttiAnalyzer`/`PEExceptionAnalyzer`/`TEBAnalyzer` (with the MSVC demangler from step 3 in hand).
19. **PDB side-file seam + `pdb` crate** → `PdbUniversalAnalyzer` (the Windows DWARF) — highest decompilation value of the Windows set, gated behind the PE loader.

**Defer indefinitely (inventory-only; revisit only if a target demands it):**
- The entire Tier-E container/firmware/filesystem set (no decompilable code; needs the `FileFormatAnalyzer` Listing subsystem with zero C-output payoff).
- The bespoke PIC family, the JVM family, `.NET`/`CliMetadataTokenAnalyzer`, `PefAnalyzer`/`PefDebugAnalyzer`, `PdbAnalyzer` (MSDIA), `FidAnalyzer` (unless the kuna-native byte-hash alternative is chosen), and the DYLD shared-cache / multi-program-project analyzers.
- All of **Tier G** (framework/abstract/non-analyzer infra) — already re-expressed idiomatically or structurally irrelevant; record-only, no work.