# Build-plan: the infeasible-at-tier / non-ELF / huge-subsystem analyzers

A concrete, sequenced **build-plan** (not an implementation) for the Ghidra
analyzers that the analysis-port effort declared **infeasible at kuna's current
analysis tier**, **non-ELF / out-of-scope**, or a **huge subsystem**. The
feasible ELF analyzer set is *done* — Increments 1–17 in
[`analysis-port-log.md`](analysis-port-log.md), the completeness sweep being
Increment 15. This document is the user's "Feasible + Docker-unblock" scope-B
deliverable: *for the harder analyzers, produce a build-plan, not code.*

Each item gives **(a)** what the Ghidra analyzer does; **(b)** the precise reason
it is not feasible on kuna's tier today; **(c)** a concrete plan — what kuna would
build first, the engine seams it would hook (real kuna files/types), rough effort
(S/M/L/XL), and ordering; **(d)** a recommended verdict
(*build-now-if-scope-expands* / *defer* / *never-for-an-ELF-decompiler*).

Cross-refs: the gap inventory is [`missing-analyses.md`](missing-analyses.md);
the per-increment process log is [`analysis-port-log.md`](analysis-port-log.md);
the pass interface is
[`kuna-analysis/src/pass.rs`](../decompiler/crates/kuna-analysis/src/pass.rs).
Ghidra source paths are cited at the `GHIDRA_REV` in [`UPSTREAM.md`](UPSTREAM.md)
(`cef869af…`); class *locations* are stable across nearby revisions.

---

## 0. What the analysis tier actually provides today (the baseline)

Every "infeasible" verdict below is relative to a single, precise contract. An
analysis is a [`AnalysisPass`](../decompiler/crates/kuna-analysis/src/pass.rs):

```rust
pub struct AnalysisCtx<'a> {
    pub file:  &'a object::File<'a>,   // parsed object (read-only `object` crate view)
    pub image: &'a ObjectLoadImage,    // vma -> raw bytes (loadimage_object.rs)
    pub arch:  &'a Architecture,       // engine arch: types, spaces, options
}
pub trait AnalysisPass {
    fn stage(&self) -> Stage;
    fn id(&self) -> &'static str;
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput;   // pure, additive, never fails
}
```

A pass sees **the parsed object container + raw image bytes + the architecture**.
It runs **before any decompilation** — pre-disassembly, on the real-ELF bootstrap
path only (`bootstrap_from_elf`; the XML datatest path never constructs an
`ObjectLoadImage`, which is what keeps the 675/675 parity oracle structurally
untouched). It returns one flat, deduplicated `AnalysisOutput` whose fact kinds
are the *entire* vocabulary the commit seam
([`engine.rs::commit_analysis_output`](../decompiler/crates/kuna-console/src/engine.rs))
knows how to apply:

| Fact kind | What it seeds |
|---|---|
| `symbols: Vec<SymFact>` | `(addr, name, Function|Data)` -> `add_function` / `add_symbol_mapped` |
| `entries: Vec<u64>` | discovered function entry VMAs -> `name_function` + `add_function` |
| `noreturn: Vec<NoReturnFact>` | by-address no-return marking |
| `readonly: Vec<(u64,u64)>` | read-only ranges (string rendering) |
| `strings: Vec<StringFact>` | typed `char[N]` data objects |
| `prototypes: Vec<PrototypePieces>` | callee signatures (libproto/DWARF) |
| `context_paints: Vec<ContextPaint>` | SLEIGH decode-mode context bits (ARM `TMode`) |
| `tracked_regs: Vec<TrackedRegFact>` | entry register-VALUE seeds (MIPS `t9`) |
| `call_fixups: Vec<CallFixupFact>` | cspec call-fixup inject ids |
| `locals: Vec<LocalFact>` | DWARF named/typed stack locals |

**The defining absence.** There is *no* fact kind for an **instruction**, a
**cross-reference**, or a **discovered-by-flow function body**, and there is no
`Listing` / `ReferenceManager` / `FunctionManager` object in `AnalysisCtx` —
because none of those exist at this tier. kuna's tier is "object bytes + symbols,
before decompile." Ghidra's analyzer tier, by contrast, runs *after* a
recursive-descent disassembly pass has populated a `Listing` (every decoded
instruction), a `ReferenceManager` (every operand->target xref), and a
`FunctionManager` (every discovered function body). **The whole "infeasible-at-tier"
group is exactly the analyzers that read those three subsystems.** Group 1 below
designs the shared prerequisite that would create them in kuna.

---

## Group 1 — Infeasible-at-tier: needs a post-disassembly Listing / Reference subsystem

`AggressiveInstructionFinderAnalyzer` (+ ARM variant), `FindNoReturnFunctionsAnalyzer`
(flow heuristic), and the operand/reference markup family
(`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`, `ScalarOperandAnalyzer`,
`ElfScalarOperandAnalyzer`) all share **one** missing dependency: a populated
disassembly Listing + reference graph. So the most valuable thing this group can do
is **design that shared prerequisite once**. The per-analyzer plans then become
small consumers of it.

### 1.0 The shared prerequisite — a kuna "post-disassembly Listing tier" (the keystone)

**What it is.** A new, optional, *post-load / pre-pass* subsystem that performs a
recursive-descent disassembly over the loadimage bytes — seeded from the already
known function entries (`AnalysisOutput.entries` + funcsyms + PLT) — and produces
an in-memory **instruction/xref/function model**: for each reached address, the
decoded instruction length + flow type (fall-through / branch / call / return /
indirect) and its computed control-flow + data targets, accumulated into (i) an
`Instruction` map (vma -> length, flow, targets), (ii) a `Reference` multimap
(from-vma -> to-vma, ref kind: code/call/data), and (iii) a `DiscoveredFunction`
set (entries reached by call edges / prologue, with rough body extents).

This is the kuna analog of Ghidra's `Disassembler` +
`AutoAnalysisManager`-populated `Listing` / `ReferenceManager`
(`Ghidra/Framework/SoftwareModeling/src/main/java/ghidra/program/model/symbol/ReferenceManager.java`)
/ `FunctionManager` — but *scoped to what the analysis tier and the three consumers
actually need* (linear+recursive decode, flow classification, xref accumulation),
**not** a full mutable program database. The Ghidra primitive it most resembles is
`PseudoDisassembler`
(`Ghidra/Framework/SoftwareModeling/src/main/java/ghidra/app/util/PseudoDisassembler.java`:
`disassemble()`, `followSubFlows()`, `checkValidSubroutine()`), which decodes
speculatively without persisting to a program.

**Where it would live.** A new module
`decompiler/crates/kuna-analysis/src/listing/` (e.g. `listing/mod.rs`,
`listing/disasm.rs`, `listing/xref.rs`). It produces a `Listing` value that is
threaded into `AnalysisCtx` as a new **optional** field
(`pub listing: Option<&'a Listing>`), populated by a driver step in
`passes::run_default_analyses` *after* the entry-discovery pass
([`s1_entry`](../decompiler/crates/kuna-analysis/src/s1_entry/)) and *before* the
listing-consuming passes. `None` when the tier is disabled (default), so every
existing pass is untouched.

**What it reuses (this is the cheap part).** kuna already owns a complete,
production SLEIGH decoder — it does not need a new one:

- **Decode.** [`Sleigh::one_instruction(emit, baseaddr)`](../decompiler/crates/kuna-sleigh/src/sleigh.rs)
  decodes one instruction's p-code at a vma; `Sleigh::instruction_length(addr)`
  gives the byte length; `Sleigh::print_assembly(...)` the mnemonic. The Listing
  tier walks bytes calling these — *the exact primitive a `PseudoDisassembler`
  wraps in Ghidra*.
- **Context.** The active language + its `ContextDatabase`
  ([`kuna-sleigh/src/globalcontext.rs`](../decompiler/crates/kuna-sleigh/src/globalcontext.rs))
  feed `one_instruction`; the ARM `TMode` / MIPS `ISA_MODE` context bits the
  analysis tier already paints (`context_paints`) are honored, so Thumb decodes as
  Thumb.
- **Bytes & read-only.** `ObjectLoadImage::load_fill` + `get_readonly`
  ([`loadimage_object.rs`](../decompiler/crates/kuna-analysis/src/loadimage_object.rs))
  are the byte source and the executable/RO section partition.
- **Flow classification (reference, not copy).** The S2 engine
  ([`kuna-decomp/src/s2_lift/flow.rs`](../decompiler/crates/kuna-decomp/src/s2_lift/flow.rs):
  `FlowInfo`, `generate_ops`, `generate_blocks`, fall-through/branch handling) is
  the *authoritative* model of how a raw instruction's p-code becomes flow edges.
  The Listing tier does **not** reuse `Funcdata`/`FlowInfo` directly (that is
  per-function decompile state, too heavy for a whole-image scan) — it mirrors
  *only* the flow-type classification logic (CALL / BRANCH / CBRANCH / RETURN /
  BRANCHIND from the decoded p-code opcodes), a small, well-understood subset. This
  keeps the Listing tier a lightweight whole-image scanner, not a second
  decompiler.

**New fact kinds it would add** (to `pass.rs::AnalysisOutput`, all additive,
default-empty):

- `InstructionFact { addr, len, flow }` — optional, only if a consumer needs to
  *emit* instruction markup (most consumers read the `Listing` directly and emit
  ordinary `symbols`/`entries`).
- `RefFact { from, to, kind }` — a recovered xref. **Open design question (decide
  before building):** kuna's decompiler reads loadimage bytes + the symbol/type
  tables, **not** a ReferenceManager (verified in Increment 4 — no fact kind or
  commit API carries a "reference"). So `RefFact` is only worth materializing if a
  consumer's *decompiler-relevant* output (a name, a type, an entry) derives from
  it. References-as-listing-cosmetics stay un-emitted (the same call that disabled
  the operand-reference family).

**How it avoids perturbing the XML datatest parity path.** Three structural
guards, each already proven by the existing pass binding:

1. The Listing tier is built **only** inside `bootstrap_from_elf` /
   `run_default_analyses`, reached only on `\x7fELF`. The XML path
   (`bootstrap_program`) constructs no `ObjectLoadImage` and calls no analyses, so
   it never builds a Listing (identical to how no current pass runs there).
2. It ships **default-off** behind a settable option
   (`--option listing on`, registered in `stages.toml` + `KUNA_OPTION_NAMES`),
   exactly like `addrtable`. Off => `AnalysisCtx.listing == None` => every consumer
   pass no-ops => byte-identical output.
3. It is **read-only over `ctx`** and feeds the *same* additive commit seam (it
   discovers new `entries` / `symbols`, it does not mutate the engine). A
   downstream Ghidra front-end (`ArchitectureGhidra`) is unaffected for the same
   reason every analyzer fact is — the front-end protocol never reaches
   `bootstrap_from_elf` (see [Downstream compatibility](analysis-port-log.md#downstream-compatibility)).

**Effort: XL.** This is a genuine new subsystem (recursive-descent driver + flow
classification + xref accumulation + the `Listing` data model + option gating +
tests). It is, however, the *force-multiplier* of scope B: it unlocks all of 1.1,
1.2, and 1.3 at once and is a prerequisite for the sound version of AIF.

**Verdict (1.0): build-first-if-scope-expands.** It is the single highest-leverage
subsystem in this document — it converts three "infeasible" analyzers into "small
consumer passes" and is reusable by future flow-based heuristics. Build it before
any consumer.

---

### 1.1 `FindNoReturnFunctionsAnalyzer` (the flow heuristic)

- **(a) What it does.** The *Discovered* half of no-return detection (the *Known*
  name-list half is already ported — Increment 1/7/15). It finds functions that
  never return by **flow analysis**: a callee is no-return if the call sites to it
  show enough "no fall-through" evidence (a function/data definition or junk after
  the call, no valid fall-through instruction), past an evidence threshold
  (`OPTION_DEFAULT_EVIDENCE_THRESHOLD = 3`), iterated to a fixpoint; it then repairs
  the damaged control flow.
  Source: `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/analysis/FindNoReturnFunctionsAnalyzer.java`.

- **(b) Why infeasible today.** It is built entirely on the three missing
  subsystems. Verified in the source: it calls
  `program.getReferenceManager().getReferencesTo(entry)` (callers),
  `program.getListing().getInstructionAt(fromAddr)` /
  `getInstructionAt(fallthruAddr)` (the call + its fall-through), and
  `cp.getFunctionManager().getFunctionAt(entry)`, deciding on
  `inst.getFlowType().isCall()` / `hasFallthrough()`, plus a `SimpleBlockModel` and
  `ClearFlowAndRepairCmd` for the repair half. **It needs a post-disassembly
  Listing + ReferenceManager + FunctionManager, which kuna's analysis tier
  (pre-decompile, object-bytes-only) does not have.**

- **(c) Plan.** *Depends on 1.0.* With a `Listing`, this is a **small** consumer
  pass `s1_loader/noreturn_discovered.rs` (sibling of the Known `noreturn.rs`): for
  each candidate callee, walk the `RefFact` callers; for each call site, ask the
  `Listing` whether a valid instruction exists at the call's fall-through vma and
  whether the bytes after look like a no-return pattern; tally evidence; iterate to
  fixpoint; emit `NoReturnFact { addr, name }` — the **existing** commit arm applies
  it (no engine change). The "repair damaged flow" half is *inherited* — kuna's
  engine already does dead-code elimination after a no-return call. Effort **S**
  *given 1.0* (the no-return commit path is already proven).

- **(d) Verdict: defer (build only after 1.0).** Lower marginal value than the
  reference-markup family: kuna already gets the high-value no-return cases from the
  three vendored `noReturnFunctionConstraints.xml` lists (base + rustc + golang) and
  the engine's own dead-code elimination already prunes most post-no-return junk.
  Worth doing once 1.0 exists, but not a reason to build 1.0 on its own.

---

### 1.2 The operand / reference markup family

`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`, `ScalarOperandAnalyzer`,
`ElfScalarOperandAnalyzer`.
Sources: `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/analysis/{OperandReferenceAnalyzer,DataOperandReferenceAnalyzer,ScalarOperandAnalyzer,ElfScalarOperandAnalyzer}.java`.

- **(a) What they do.** Walk every disassembled instruction operand and create
  *listing references*: `OperandReferenceAnalyzer` turns operands into string /
  pointer / address-table / subroutine references (and creates the targets — it even
  uses a `PseudoDisassembler` for subroutine detection and the `RelocationTable` to
  guide acceptance); `DataOperandReferenceAnalyzer` is the data-to-data subset that
  never creates functions; `ScalarOperandAnalyzer` turns scalar immediates that look
  like addresses into references; `ElfScalarOperandAnalyzer` is a *corrective* ELF
  subclass that **removes** the bad `.got`/`.plt` scalar references its parent
  creates.

- **(b) Why infeasible today.** All four operate on the disassembled Listing +
  `ReferenceManager` (they iterate instruction operands and call `addReference`).
  Neither exists at this tier. Two further facts (verified in source) make a naive
  port *unfaithful even if you had a Listing*: `ScalarOperandAnalyzer.getDefaultEnablement`
  returns `!ElfLoader.isElf(program)` — i.e. it is **default-OFF for ELF**
  upstream; and `ElfScalarOperandAnalyzer` only ever *deletes* bad `.got`/`.plt`
  references, which kuna never creates wrongly (it already gets correct `.plt`/`.got`
  naming from [`elf_plt.rs`](../decompiler/crates/kuna-sleigh/src/elf_plt.rs)).
  Crucially, **references never reach kuna's decompiler** — it consumes loadimage
  bytes + the symbol/type tables, not a ReferenceManager.

- **(c) Plan.** *Depends on 1.0.* Even with a `Listing`, the only output of this
  family with any decompiler relevance is "a scalar that points at a `.rodata`
  string should be typed `char *`." That single idea would be a tiny consumer pass
  that, for each `RefFact` whose target lands in a readonly char-printable run,
  emits the appropriate typed fact. **But it is blocked by a second wall** that 1.0
  does *not* remove: the printer/MapGlobals shadowing documented in Increment 3 — a
  constant mapping to a named global renders as the *name*, shadowing the literal —
  and the relevant typing is already delivered by `s1-libproto` + S5 usage
  inference. Effort **S** *given 1.0*, but with near-zero marginal payoff.

- **(d) Verdict: never-for-an-ELF-decompiler (as producing passes).** Three of the
  four are listing-cosmetic / non-ELF-default / corrective-of-a-bug-kuna-lacks; the
  fourth's one useful idea is already covered and printer-shadowed. Build 1.0 for
  AIF / FindNoReturn / Go, **not** for these. (This re-confirms the Increment-4
  call with the added 1.0 context.)

---

### 1.3 `AggressiveInstructionFinderAnalyzer` (+ ARM variant)

Sources:
`Ghidra/Features/Base/src/main/java/ghidra/app/plugin/prototype/analysis/AggressiveInstructionFinderAnalyzer.java`,
`…/prototype/analysis/ArmAggressiveInstructionFinderAnalyzer.java`.

- **(a) What it does.** A speculative, whole-program, *iterative* gap-filler that
  guesses code in undefined gaps. Builds a function-start byte-**fingerprint**
  histogram via `SleighDebugLogger` (`SleighDebugMode.MASKS_ONLY`,
  `getInstructionMask()`), then speculatively disassembles a gap candidate with a
  `PseudoDisassembler` only if its masked prefix matches a fingerprint seen enough
  times with matching context — its own DESCRIPTION warns "IT MAY CREATE A LOT OF
  BAD CODE!" The ARM variant adds `TMode`-state handling for ARM/Thumb mixed code.

- **(b) Why infeasible today.** Verified constants/deps in source: it is
  default-OFF (`setDefaultEnablement(false)`); it bails unless
  `funcCount >= MINIMUM_FUNCTION_COUNT (= 20)` **and**
  `program.getListing().getNumInstructions() > 0`; it constructs
  `new PseudoDisassembler(program)`, `PseudoDisassemblerContext`, and
  `SleighDebugLogger(..., MASKS_ONLY)`. So it needs (1) a populated `FunctionManager`
  with >=20 found functions, (2) a `Listing`, (3) a `PseudoDisassembler`
  (recursive-descent + `followSubFlows` + `checkValidSubroutine`), and (4) an
  instruction-mask machinery (`getInstructionMask`). kuna's analysis tier has none
  of these.

- **(c) Plan.** *Depends on 1.0 — and on more than 1.0.* The recursive-descent
  decode + >=20-function gate come from the Listing tier. The **distinctive**
  ingredient is the instruction-mask fingerprinting, which needs a new SLEIGH
  capability: exposing the per-constructor decode *mask* (the bits that are opcode
  vs operand) — kuna's [`sleigh.rs`](../decompiler/crates/kuna-sleigh/src/sleigh.rs)
  decodes but does not currently surface a `SleighDebugLogger`/`getInstructionMask`
  equivalent. Plan: (i) build 1.0; (ii) add a mask accessor to the SLEIGH decoder
  (M–L, touches `kuna-sleigh`); (iii) write the histogram + gap-candidate
  acceptance loop as a consumer pass emitting `entries`. Effort **L** on top of 1.0.

- **(d) Verdict: never (build the sound substitute, not AIF).** Even with all
  machinery, AIF's *unique* contribution — guessing code in gaps with no
  symbol/FDE/xref — is precisely the high-false-positive case Ghidra ships disabled.
  For a decompiler that is *given* entries, its sound output (new entries) is
  already delivered by `s1-entry-disc` + `s1-eh-frame`. Porting AIF faithfully is
  large *and* the result is off-by-default; porting it naively (linear gap decode)
  silently drops its correctness model. Recommend the Listing tier (1.0) for its
  sound consumers and **decline AIF itself**.

---

## Group 2 — Non-ELF loaders (PE/COFF, Mach-O)

kuna is a *standalone ELF decompiler*; its `loadimage_object.rs` + `elf_plt.rs`
read ELF containers via the `object` crate. A non-ELF target needs a whole new
loader: container parse -> segment/section mapping into `ObjectLoadImage` ->
symbol/import/entry extraction -> relocation handling. Only then do *any* analyzers
(even the ported ones) have anything to run on.

### 2.1 PE / COFF

- **What it'd unlock & cost.** Loader sources:
  `Ghidra/Features/Base/src/main/java/ghidra/app/util/opinion/{PeLoader,CoffLoader,MSCoffLoader}.java`;
  the analyzer wrappers `…/app/analyzers/{PortableExecutableAnalyzer,CoffAnalyzer,CoffArchiveAnalyzer}.java`
  (thin delegators to format-analysis commands). The *loader* work is **L** but
  largely a mechanical container port (the `object` crate already parses PE/COFF
  headers, sections, the export/import directories and the COFF symbol table — so
  much of the parse is free; the work is mapping it into `ObjectLoadImage` + import
  thunk naming, the PE analog of `elf_plt.rs`). Once loaded, PE unlocks the ported
  no-return / demangle / libproto / strings / entry-disc passes — most run unchanged
  because they read `object::File` + bytes generically. The exception is demangling:
  MS-mangled names need a *different* demangler (`msvc-demangler`, an **M**
  dep-substitution alongside the existing `cpp_demangle`/`rustc-demangle`).
  PE-specific *analyzers* (RTTI, SEH/TEB, the `PEFunctionsThatDoNotReturn` list,
  `X86FunctionPurgeAnalyzer` for stdcall purge) are separate follow-ons (**M–L**
  each).

- **Fixtures / container.** Needs Windows PE fixtures — the build container can add
  **mingw-w64** (`x86_64-w64-mingw32-gcc`) to cross-compile small PE binaries
  in-env, the PE analog of the existing ELF/MIPS fixtures (no off-host step).

- **Verdict: defer (build-if-scope-expands to Windows).** Self-contained and
  unlocks a real second platform, but out of the current ELF mandate. The loader is
  the gate; build it first if PE scope is taken.

### 2.2 Mach-O

- **What it'd unlock & cost.** Loader sources:
  `…/app/util/opinion/{MachoLoader,MachoProgramBuilder}.java`; analyzer wrapper
  `…/app/analyzers/MachoAnalyzer.java`; the Mach-O-specific
  `…/app/plugin/core/analysis/MachoFunctionStartsAnalyzer.java` (creates functions
  from the `LC_FUNCTION_STARTS` load command — a clean entry oracle, the Mach-O
  analog of kuna's `.eh_frame`/init-array oracles). Same loader shape as PE (the
  `object` crate parses Mach-O load commands / segments / `nlist` symbols / classic
  + chained-fixup import tables), so loader effort **L**; once loaded, the generic
  passes apply (Itanium C++ demangle is already in kuna; Swift/ObjC metadata are
  separate huge subsystems, out of scope). `MachOFunctionsThatDoNotReturn` is the
  Mach-O no-return list (trivial once loaded).

- **Fixtures / container.** Needs Mach-O fixtures — the container can add
  **osxcross** (an `o64-clang` cross-toolchain) to build small Mach-O binaries
  in-env.

- **Verdict: defer.** Same reasoning as PE; lower priority than PE for a typical
  RE workload. Build the loader first if Mach-O scope is taken.

---

## Group 3 — Huge ELF-relevant subsystems

These *are* ELF and *are* decompiler-relevant, but each is a large self-contained
subsystem, not a small pass.

### 3.1 Go pclntab name recovery (`GolangStringAnalyzer`, `GolangSymbolAnalyzer`)

- **(a) What they do.** Recover Go function names (and string slices) from the Go
  runtime metadata embedded in the binary — the `pclntab` (program-counter line
  table) + `moduledata`, which map every code address to a function name even in a
  *stripped* Go binary. Sources:
  `…/app/plugin/core/analysis/{GolangSymbolAnalyzer,GolangStringAnalyzer}.java`,
  backed by `…/app/util/bin/format/golang/rtti/{GoPcHeader,GoModuledata,GoPcDataTable,GoPcValueEvaluator}.java`.

- **(b) Why it's not just a pass.** It needs a faithful **pclntab/moduledata
  parser**: locate `runtime.pclntab` (the `GoPcHeader` magic — version-specific
  across the Go 1.2 / 1.16 / 1.18 / 1.20 layouts), parse the function-name table and
  the func-offset table, and walk `moduledata` to resolve string-slice headers. That
  parser is a few hundred lines of version-aware binary structure walking — a
  subsystem, not a fact emitter. (Note the completeness-sweep observation: stripped
  Go binaries are ~750 KB and drop `.symtab`, so this is *the* way to get names.)

- **(c) Plan.** Build a `s1_golang/` module: a `pclntab.rs` parser (no Listing tier
  needed — it is pure container/byte parsing, so it is **independent of Group 1**)
  that emits `SymFact { addr, name, Function }` per recovered Go function and
  `StringFact` per recovered Go string slice, through the **existing** commit arms.
  Gated on the existing `detect_compiler == Go`
  ([`s1_sourcelang`](../decompiler/crates/kuna-analysis/src/s1_sourcelang/)). Effort
  **L** (the parser dominates; the fact emission is proven). Fixtures: build a tiny
  Go binary at test runtime (the Increment-15 pattern — `go` is on the host),
  stripped and unstripped, to exercise the pclntab path.

- **(d) Verdict: build-if-scope-expands to Go (highest value in Group 3).** It is
  pure-byte-parsing (no XL prerequisite), Go-only-gated (zero risk to non-Go), and
  delivers the single biggest naming win available for stripped Go binaries. If any
  scope-B subsystem after the Listing tier is worth it, this is it.

### 3.2 FID function-ID fingerprinting (`FidAnalyzer`)

- **(a) What it does.** Identifies statically-linked **library** functions by
  hashing each function body and matching against a prebuilt database of known
  library-function fingerprints, naming `sub_…` as e.g. `memcpy` /
  `_ZSt…`. Source: `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/analyzer/FidAnalyzer.java`,
  backed by the hashing subsystem
  `…/feature/fid/hash/{FidHasher,MessageDigestFidHasher,FidHashQuad,FunctionExtentGenerator}.java`,
  the service `…/feature/fid/service/FidService.java` (FNV-1a64 hash;
  `MEDIUM_HASH_CODE_UNIT_LENGTH = 24`; `SCORE_THRESHOLD ~= 14.6`), and the
  database/query layer `…/feature/fid/db/FidQueryService.java`.

- **(b) Why it's not just a pass.** It needs **three** things kuna lacks: (1)
  per-function bodies to hash — i.e. the **Group-1 Listing tier** (the hash is over
  the *function extent's instructions*, masking out operands, via
  `FunctionExtentGenerator` + the instruction mask — the same mask machinery AIF
  needs); (2) a faithful port of the FID hash (the FNV-1a64 `FidHashQuad` 4-part
  hash); (3) a **populated FID database** — Ghidra ships `.fidb` databases built
  offline from real libraries; kuna has none, and building them is its own
  pipeline.

- **(c) Plan.** *Depends on 1.0 + the AIF mask accessor (1.3 step ii).* Then port
  the hasher (`s1_fid/hash.rs`, **M**), define a database format + loader
  (`s1_fid/db.rs`, **M**), and build at least one `.fidb`-equivalent from a known
  libc (a new offline tool, **L**). Emit `SymFact{Function}` for matches above the
  score threshold. Total effort **XL** (mostly the database-building pipeline).

- **(d) Verdict: defer (lowest priority).** Highest cost (Listing tier + mask
  accessor + hasher + a whole database-building pipeline) for a benefit largely
  overlapped by what kuna already does for *dynamically*-linked ELF (PLT/GOT naming
  + libproto). FID's win is for *statically*-linked stripped binaries — real, but
  niche. Build only if static-binary RE becomes an explicit goal, and only after
  1.0 exists.

---

## Group 4 — Cosmetic / low-payoff (already discovered)

Briefly, for completeness (full detail in Increments 8 & 15):

- **`_INIT_<i>` / `_FINI_<i>` array-element naming** — `s1_entry` already
  *discovers* the `.init_array`/`.fini_array` element addresses (Increment 5,
  oracle 2); only the Ghidra-style *names* differ from `sub_<addr>`. Delivering them
  needs reshaping the `entries` fact from `Vec<u64>` to a named-entry shape — a
  small, cross-cutting fact-type change. Listing-cosmetic (the decompiled body is
  identical). **Being handled separately as a small port.**

- **`$d` data-run markup** — the ARM mapping-symbol data-run analog of the `$t`/`$a`
  code markup (Increment 8). Listing-cosmetic; deferred. No subsystem needed.

Both are **defer**: low payoff, no new subsystem, not part of scope B's
build-plan list.

---

## If we take scope B: the prioritized sequence

Build subsystems in this order — each line states *why it ranks there*:

1. **The post-disassembly Listing tier (1.0) — FIRST.** It is the only item that
   unlocks **three** analyzers at once (`FindNoReturnFunctionsAnalyzer`, the sound
   substitute for AIF, and a prerequisite for FID), and it is reusable by any
   future flow-based heuristic. Highest leverage; everything flow-based in this
   document is downstream of it. (Effort XL — but it is the keystone.)

2. **Go pclntab recovery (3.1) — SECOND.** The highest *standalone* value: it is
   **independent of 1.0** (pure byte-parsing), Go-only-gated (zero risk), and
   delivers the biggest naming win for stripped Go binaries. Can proceed in parallel
   with 1.0 since they share nothing.

3. **`FindNoReturnFunctionsAnalyzer` (1.1) — THIRD.** The cheapest consumer of 1.0
   (effort S given the Listing tier + the proven no-return commit arm); modest but
   real value.

4. **PE/COFF loader (2.1) — FOURTH, only if Windows scope is taken.** Self-contained
   second-platform unlock; gated behind a deliberate scope decision; add mingw-w64
   to the container for fixtures. (Mach-O / osxcross is the same shape, after PE.)

5. **FID (3.2) — LAST.** Highest cost (needs 1.0 + the mask accessor + a hasher + a
   database-building pipeline) for a static-binary-only, partly-overlapped benefit.

**Decline outright** (do not build, even in scope B): `AggressiveInstructionFinderAnalyzer`
itself (build the sound Listing-tier consumers instead — 1.3 verdict) and the
operand/reference markup family as producing passes (1.2 verdict — printer-shadowed,
ELF-default-off, corrective-of-a-bug-kuna-lacks).
