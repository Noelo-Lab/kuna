# Kuna Stage Mapping

This document maps every upstream Ghidra decompiler source module — the C++ tree kuna was ported from (anchored as `decompiler/cpp/<file>.{cc,hh}`) — to the decompilation stage model. There are **115 upstream translation units** (`.cc`), each with a matching header (`.hh`); a header always maps to the same stage as its `.cc`, so they are listed as a single `file` entry. The Rust port under `decompiler/crates/kuna-decomp/src/` is **physically organized into stage-named folders** matching this mapping (`s1_partition/`…`s9_emit/`, plus `substrate/`, `p0_knowledge/`, `infra/`; see the layout table in `docs/stages.md`), and the live registry is queryable at the console (`stage list/map/catalog`).

**Two numberings coexist here.** Section 0 maps every file to the **current normative model** (`docs/stages.md` / `docs/stage-model.md`: P0 plane + S1–S9 with Band B) — this is the mapping to use. The remainder of the document retains the original **legacy 19-stage** mapping (Stage 00–18) for history and for its per-file role descriptions and source anchors, which remain accurate; only its stage *numbering* is superseded. The legacy model's four phantom stages (06 Dataflow, 10 Memory, 12 Aggregate Types, 15 Interproc) have no place in the new model — they dissolve into Band B fixed points (see `docs/stage-model.md` §1).

**Method.** Stages were assigned by combining three signals: (1) the Makefile/module partition of the source tree; (2) the canonical decompile pass order built by `ActionDatabase::universalAction` (`coreaction.cc:5609-5896`); and (3) the frontend/backend endpoints — bytes enter through `LoadImage`/`Translate` and source text leaves through `PrintLanguage`. A file's **primary** stage is where its core data structures or pass logic live; straddlers are placed by *dominant owned artifact* per `docs/stage-model.md` §15. The new-model column is kept consistent with the kuna stage registry (`decompiler/crates/kuna-decomp/stages.toml`), which maps action/rule *groups* the same way; a file's stage is the stage of its primary group.

---

## 0. New-model mapping (P0 / S1–S9)

### 0.1 Legacy → new correspondence

| Legacy 19-stage | New model | Note |
|---|---|---|
| 00 Loader | S1 | image & code partition (thin by design — §5 honesty note) |
| 01 Decode, 02 P-code Lift, 03 Disassembly, 04 IR Containers | S2 | one artifact: lifted ops + CFG + jump tables + work queues (legacy over-split) |
| 05 Simplify, 07 SSA | S3 *(Band B)* | definition web; simplification quiescence is S3's gate |
| 06 Dataflow | — | phantom: dissolves into Band B fixed points |
| 08 Calls | S4 *(Band B)* | call & prototype model |
| 09 Stack | S6 *(Band B)* | stack-frame layout is the variable/storage model's concern |
| 10 Memory | — | phantom: folded into S3 heritage of stack/global ranges |
| 11 Scalar Types, 14 Range | S5 *(Band B)* | value & type facts |
| 12 Aggregate Types | — | phantom: sub-concern of S5 (type system + union resolution) |
| 13 Variables | S6 *(Band B)* | variable & storage model |
| 15 Interproc | — | phantom: emergent from S4 prototypes + restarts (edge 5) |
| 16 Structuring | S7 + S8 | region hierarchy vs schema/goto quality (one legacy stage, two artifacts) |
| 17 C Lowering, 18 C Render | S9 | one inheritance chain (`PrintLanguage` → `PrintC`), one stage |
| (no legacy home) | P0 | the knowledge/configuration plane: Symbol DB, Override, options |

### 0.2 Per-file mapping

| New stage | Files |
|---|---|
| **P0** | `database` (Symbol DB — the assertion store), `override` (per-function directives surviving restarts), `options` (pipeline configuration) — all legacy-INFRA, but they ARE the P0 artifact |
| **S1** | `loadimage`, `loadimage_xml`, `loadimage_bfd`, `raw_arch`, `xml_arch`, `bfd_arch` |
| **S2** | `translate`, `sleigh`, `sleighbase`, `sleigh_arch`, `globalcontext`, `context`, `pcoderaw`, `opcodes`, `opbehavior`, `float`, `userop`, `flow`, `pcodeinject`, `jumptable`, `varnode`, `op`, `block`, `address`, `space`, `funcdata`, `funcdata_op`, `funcdata_varnode`, `funcdata_block` |
| **S3** *(Band B)* | `ruleaction`, `subflow`, `condexe`, `expression`, `transform`, `heritage` |
| **S4** *(Band B)* | `fspec`, `modelrules` |
| **S5** *(Band B)* | `type`, `typeop`, `unionresolve`, `rangeutil`, `double`, `bitfield`, `constseq`, `prefersplit` |
| **S6** *(Band B)* | `varmap`, `variable`, `merge`, `cover`, `dynamic` |
| **S7** | `blockaction` (spans S8: structuring actions own the region tree AND the schema matching — §15 straddler) |
| **S8** | `kuna_gotoreduce` (opt-in goto-reduction by return-tail duplication); schema matching and goto selection otherwise live in `blockaction`, quality signal in kuna's `quality` metric |
| **S9** | `printlanguage`, `cast`, `printc`, `printjava`, `stringmanage`, `prettyprint`, `comment` |
| **INFRA** | everything in the legacy Infrastructure table except `database`/`override`/`options` (promoted to P0): serialization (`xml`, `marshal`, `slaformat`, `compression`, `crc32`, `filemanage`, `multiprecision`), framework (`architecture`, `action`, `coreaction`, `capability`, `graph`, `cpool`, `callgraph`, `libdecomp`), console (`interface`, `ifacedecomp`, `ifaceterm`, `consolemain`, `codedata`), SLEIGH compiler (`semantics`, `pcodecompile`, `pcodeparse`, `grammar`, `slgh_compile`, `slghparse`, `slghscan`, `slghsymbol`, `slghpatexpress`, `slghpattern`, `rulecompile`, `unify`), injection (`inject_sleigh`, `inject_ghidra`), Ghidra-IPC glue (`ghidra_*`, `comment_ghidra`, `cpool_ghidra`, `database_ghidra`, `loadimage_ghidra`, `string_ghidra`, `typegrp_ghidra`, `signature_ghidra`), emulator (`emulate`, `emulateutil`, `memstate`), signatures (`signature`, `analyzesigs`, `paramid`), tests (`test`, `testfunction`, `sleighexample`) |

Straddler notes (placement by dominant owned artifact; see `docs/stage-model.md` §15):
- `jumptable` → S2 home; phase-2 `matchModel`/`recoverLabels` runs post-Band-B and can restart (edge 2).
- `block` → S2 home (BlockBasic/CFG containers); the structured-block hierarchy (`BlockIf`/`BlockWhileDo`…) is S7's artifact.
- `funcdata*` → S2 home (the op-graph artifact owners); coordinate Band B and S7 (`jumpvec`/`bblocks`/`sblocks` are three distinct members).
- `userop` → S2 home (lift semantics); volatile-memory modeling touches S3/S5; display control touches S9.
- `heritage` → S3 home; LoadGuards refined by S5 value-set analysis (two-phase).
- `ruleaction` → S3 home; many rules read/write S5 facts (the Band-B coupling, not a mis-mapping).
- `cast` → S9 home (`ActionSetCasts` is render-prep); decisions derive from S5's type lattice.
- `blockaction` → S7 home; schema precedence and goto selection are S8 decisions implemented by the same collapse engine.

### 0.3 kuna-owned files (not part of the upstream 115)

| File | Stage | Role |
|---|---|---|
| `kuna_stages` | P0 | the stage registry: group→stage map + sub-stage catalog + surface routing |
| `kuna_console` | P0 | `IfaceKunaCapability`: `stage list/map/status`, `pipeline`, `quality`, `restarts` |
| `kuna_assert` | P0 | `kassert` typed assertion API (stage-model.md §12) over the existing stores |
| `kuna_restartlog` | P0 | restart observability side table (mechanism c reasons) |
| `kuna_compareform` | S3→S9 | GH-558 comparison-canonicalization sub-stage split (`canonicalcompare`/`presentcompare`) |
| `kuna_arraynotation` | S9 | GH-558 pointer-notation sub-stage (`option arraynotation`) |
| `kuna_dedupvardecls` | S9 | naming-policy sub-stage: collapse duplicate scalar local declarations (`option dedupvardecls`, angr/DIV-7) |
| `kuna_thumbfuncptr` / `kuna_inferfuncentry` | S5 | const-pointer inference (GH-8471 / GH-6930) |
| `kuna_returnpair` | S4 | trial-finalization return-register join (GH-6990) |
| `kuna_booleanmask` / `kuna_ovlesssimplify` / `kuna_addcarrychain` | S3/S5 | simplification-quiescence rules (GH-1282 / GH-7190 / GH-8913) |
| `kuna_memsetsequence` | S5 | const-sequence memset recovery (GH-9230/1537) |
| `kuna_v850indbranch` | S2 | flow-classification reclassify (GH-8817) |
| `kuna_tailcalljump` | S2 | flow-classification: recover an -O2 tail jump (direct `jmp` to another function's entry, e.g. `jmp setlocale@plt`) as a tail call — BRANCH→CALL+RETURN (angr tee-O2 tail-jumps; `option tailcalljump`, default-off) |
| `kuna_flagcompare` | S3 | flag-modelled-compare folding — `(b<<k)s<0`, N==V SBORROW (GH-1276/8777) |
| `kuna_switchmodbound` | S2 | modulo/and-mask LOAD-table jumptable index bound (GH-9191) |
| `kuna_stackprobeloop` | S2/S6 | stack-pointer normalization across a stack-probe loop (GH-8017) |
| `kuna_dynamichashmax` | S6 | DynamicHash collision budget 8→16 (GH-8467) |
| `kuna_stackalias` | S3/S6 | preserve store-through-a-stack-pointer-alias (GH-8500) |
| `kuna_arraystride` | S3/S5 | strided-induction array index recovery (GH-8724) |
| `kuna_sparcstructret` | S2/S4 | SPARC struct-return post-call `unimp` fall-through (GH-6882) |
| `kuna_condexeplace` | S3 | keep ActionConditionalConst const-COPY out of loop blocks (GH-9203) |
| `kuna_inputvarnodeadjust` | S6 | absorb overlapping input varnodes instead of aborting (GH-9218) |
| `kuna_stackguard` | S7 | strip the -fstack-protector canary epilogue (angr StackCanarySimplifier port; `option stackguard`, default-off) so the shared-return goto is eliminated |
| `kuna_regiongraph` | S7 | graph substrate for the angr RegionIdentifier port (mutable digraph, dominators, incremental frontiers) |
| `kuna_regionid` | S7 | angr RegionIdentifier port: analysis-only nested region tree over bblocks; `region tree/blocks/walk` (`docs/regions.md`) |
| `kuna_loopbreak_recovery` | S8 | lower loop-exit `goto <successor>` edges to structured `break;` (port of Ghidra `BlockGraph::scopeBreak`, run in `ActionFinalStructure` before `markUnstructured`; `option loopbreak_recovery`, DIV-10 default-on) |
| `kuna_gotoreduce` | S8 | angr SAILR/Phoenix goto-reduction (ReturnDuplicator) port: after `ActionFinalStructure`, duplicate a small return tail into an `if`-goto so the cross-edge becomes a structured early return (`option gotoreduce`, default-off) |

---

## Phase 1 — Frontend (Bytes to IR)

### Stage 00 — Loader

| File | Role | Pipeline/Source anchor |
|---|---|---|
| loadimage | `LoadImage` abstract interface (loadFill/getNextSymbol/getNextSection) plus simple `RawLoadImage`; supplies raw executable bytes by address to the decoder. | `loadimage.hh:73` class LoadImage; loadFill() `loadimage.hh:80` |
| loadimage_xml | `LoadImageXml`: LoadImage backed by an XML `<binaryimage>` schema (byte chunks, symbols, readonly ranges). | `loadimage_xml.hh:35` class LoadImageXml : public LoadImage |
| loadimage_bfd | `LoadImageBfd`: LoadImage using GNU BFD to parse real object/executable files and serve bytes by address. | `loadimage_bfd.hh:59` class LoadImageBfd : public LoadImage |
| loadimage_bfd (kuna ext) | **(kuna, DIV-8)** `kuna-analysis::loadimage_object::ObjectLoadImage` adds an `ET_REL` relocatable-object (`.o`) path (`from_relocatable` → `s1_loader::elf_reloc`): section layout above `0x400000` + `.rela.*` relocation application + symbol rebasing/externs. Gated by the file type + `option relocobjects` (env `KUNA_RELOC_OBJECTS`, default on). | `s1_loader/elf_reloc.rs::layout_relocatable` |
| raw_arch | `RawBinaryArchitecture`: capability wiring a flat raw binary into a SleighArchitecture via `buildLoader()`. | `raw_arch.hh:42`; buildLoader() |
| xml_arch | `XmlArchitecture`: capability loading an executable from an XML save-file, building a `LoadImageXml`. | `xml_arch.hh:42`; buildLoader() |
| bfd_arch | `BfdArchitecture`: capability loading executables via BFD, building `LoadImageBfd` and resolving the arch. | `bfd_arch.hh:43`; buildLoader() |

### Stage 01 — Decode

| File | Role | Pipeline/Source anchor |
|---|---|---|
| translate | `Translate` abstract engine interface (oneInstruction→pcode, printAssembly, instructionLength) plus AddrSpaceManager/JoinRecord; the contract turning instruction bytes into p-code/disassembly. | `translate.hh:302` class Translate; oneInstruction() `translate.hh:435` |
| sleigh | Concrete SLEIGH decode engine: parses one machine instruction (DisassemblyCache/ParserContext), drives SleighBuilder/PcodeCacher to emit p-code + assembly. | `sleigh.hh:162` class Sleigh; oneInstruction()/printAssembly() `sleigh.hh:182-183` |
| sleighbase | `SleighBase`: common core reading a compiled `.sla` spec (symbol table, register xrefs, user-op names); backs the runtime register/space model. | `sleighbase.hh:60` class SleighBase : public Translate |
| sleigh_arch | `SleighArchitecture` + `.ldefs` descriptions: selects/loads `.sla/.pspec/.cspec` and builds the Sleigh engine for the chosen processor. | `sleigh_arch.hh:66` class LanguageDescription; SleighArchitecture |
| globalcontext | `ContextDatabase`/ContextBitRange and tracked-register values: address-keyed processor context (x86 16/32-bit, ARM THUMB) steering disassembly. | `globalcontext.hh:40` class ContextBitRange; ContextDatabase |
| context | Per-instruction SLEIGH parse context: Token, FixedHandle, ParserContext/ParserWalker, ContextCache used while resolving one instruction's constructor tree. | `context.hh:27` class Token; FixedHandle `context.hh:45` |

### Stage 02 — P-code Lift

| File | Role | Pipeline/Source anchor |
|---|---|---|
| pcoderaw | `VarnodeData` (space/offset/size) and raw p-code op descriptions: the bare lifted-pcode data emitted before Varnode/PcodeOp construction. | `pcoderaw.hh:35` struct VarnodeData |
| opcodes | `OpCode` enum (CPUI_COPY/LOAD/STORE/INT_*/…) naming every p-code op, plus opcode-name lookup; the vocabulary of the lifted language. | `opcodes.hh:37` enum OpCode |
| opbehavior | Per-opcode `OpBehavior` classes giving concrete semantics (evaluateUnary/Binary, recoverInput) for constant folding and emulation. | `opbehavior.hh:44` class OpBehavior |
| float | `FloatFormat`: IEEE754 encode/decode of floating-point formats and host conversions backing the CPUI_FLOAT_* p-code op semantics. | `float.hh:32` class FloatFormat |
| userop | Specialized CALLOTHER user ops (VolatileRead/Write, SegmentOp, JumpAssistOp, InternalStringOp, DatatypeUserOp) + UserOpManage giving black-box ops semantics/types/display. | `userop.hh:47-326` UserPcodeOp hierarchy + UserOpManage |

### Stage 03 — Disassembly

| File | Role | Pipeline/Source anchor |
|---|---|---|
| flow | `FlowInfo`: follows control flow from entry, calls Translate to generate all reachable raw p-code (generateOps) and splits into basic blocks (generateBlocks); handles injection, inlining, jump-table prep. | `flow.hh:58` class FlowInfo; generateOps()/generateBlocks() `flow.hh:151-152` |
| pcodeinject | P-code injection framework (InjectPayload/InjectContext/InjectParameter, PcodeInjectLibrary) supplying call-fixup and CALLOTHER-replacement p-code spliced in during flow. | `pcodeinject.hh:55` class InjectParameter; InjectPayload `pcodeinject.hh:70` |
| jumptable | `JumpTable` + JumpModel/JumpBasic strategies recover indirect-branch (switch) targets by analyzing/emulating the switch variable's data-flow, feeding new flow destinations. | `jumptable.cc:2271` JumpTable::recoverModel |

### Stage 04 — IR Containers

| File | Role | Pipeline/Source anchor |
|---|---|---|
| varnode | `Varnode` (SSA value node: address+size with flags/def/descend) and `VarnodeBank`; the fundamental variable object of the IR. | `varnode.hh:73` class Varnode; VarnodeBank |
| op | `PcodeOp` (opcode + in/out Varnodes, flow flags, SeqNum), `PcodeOpBank`, and IopSpace; the operation-node container of the IR. | `op.hh:63` class PcodeOp; IopSpace `op.hh:37` |
| block | `FlowBlock`/`BlockBasic` (basic-block container of PcodeOps with edges), `BlockGraph`, and the structured-block hierarchy (BlockIf/WhileDo/Switch…). | `block.hh:73` class FlowBlock; BlockBasic `block.hh:469`; BlockGraph `block.hh:372` |
| address | Address, SeqNum, Range, RangeList primitives (an AddrSpace + offset) identifying every Varnode/PcodeOp location. | `address.hh:16` class Address/Range/RangeList |
| space | `AddrSpace` and the spacetype taxonomy (constant/processor/spacebase/internal/fspec/iop/join) underpinning every Address and VarnodeData. | `space.hh:30` enum spacetype; class AddrSpace `space.hh:52` |
| funcdata | Core of the central `Funcdata` IR container the whole pipeline mutates: function lifecycle (clear/startProcessing/stopProcessing), flags, warnings, state coordination. | `funcdata.hh:56` class Funcdata; Funcdata::startProcessing |
| funcdata_op | PcodeOp-manipulation half of the Funcdata API: opSetOpcode/opSetInput/opSetOutput, op creation/insertion/removal used by every pass. | `funcdata_op.cc` Funcdata::opSetOpcode / opSetOutput |
| funcdata_varnode | Varnode-manipulation half of the Funcdata API: newVarnode/destroyVarnode, location/def-tree maintenance, laned-register and input-Varnode handling. | `funcdata_varnode.cc` Funcdata::destroyVarnode / setVarnodeProperties |
| funcdata_block | Funcdata block management: maintain the BlockGraph (clear/remove/join, remove unreachable blocks/branches, link/install jump tables, structureReset). | `funcdata_block.cc:34` Funcdata::clearBlocks; removeUnreachableBlocks `funcdata_block.cc:364` |

---

## Phase 2 — Analysis

### Stage 05 — Simplify

| File | Role | Pipeline/Source anchor |
|---|---|---|
| ruleaction | The giant simplification rule set: 114 `Rule*::applyOp` implementations (algebraic/boolean/pointer-arith/load-store normalization) run by the oppool ActionPool passes. | 114 Rule*::applyOp in `ruleaction.cc`; oppool1 `coreaction.cc:5658-5797` |
| subflow | SubvariableFlow/SplitFlow analyses + rules (RuleSubvarAnd/Subpiece/Shift/Zext, RuleSplitFlow/Copy/Load/Store, RuleSubfloatConvert) shrinking/splitting big Varnodes. | `subflow.hh:43` class SubvariableFlow; RuleSplitCopy `subflow.hh:317` |
| condexe | ConditionalExecution + RuleOrPredicate merging/simplifying control flow with shared conditional expressions (directsplit, OR-predicate collapse). | `condexe.hh` class ConditionalExecution; RuleOrPredicate `condexe.hh:172` |
| double | Double-precision recovery: SplitVarnode + RuleDoubleIn/Out/Load/Store and arithmetic-pair matchers reconstructing values split across lo/hi pairs. | `double.hh:32` class SplitVarnode; RuleDoubleIn `double.hh:321` |
| expression | Expression-matching helpers for simplify rules: TermOrder (additive collection/sorting) and PullExpression/InsertExpression for restructuring expression trees. | `expression.cc:237` TermOrder::collect; Pull/InsertExpression |
| bitfield | Bitfield transforms: BitFieldExpression + RuleBitFieldStore/Out/Load/In and RulePullAbsorb/InsertAbsorb normalizing bit-field access patterns. | `bitfield.hh:181` RuleBitFieldStore; cleanup pool `coreaction.cc:5841-5864` |
| constseq | ArraySequence/StringSequence detection with RuleStringCopy/Store folding contiguous constant stores into one memcpy/string CALLOTHER. | `constseq.hh:120` RuleStringCopy; cleanup pool `coreaction.cc:5841-5864` |
| transform | TransformManager/TransformVar/TransformOp infrastructure for large-scale data-flow transforms (placeholder graph then apply); engine under subflow/double rewrites. | `transform.hh:31` class TransformVar; TransformManager |
| prefersplit | PreferSplitManager splits configured registers/varnodes into hi/lo pieces (COPY/ZEXT/PIECE/SUBPIECE/LOAD/STORE) so paired sub-registers become independent variables. | `prefersplit.cc` PreferSplitManager::split; `prefersplit.hh:33` |

### Stage 06 — Dataflow

*No module is primary to Stage 06.* Dead-code elimination, COPY propagation, and general data-flow normalization are driven by Action classes in `coreaction.cc` (ActionDeadCode etc.) operating over the Stage-04 IR; the transforms themselves live in `ruleaction`/`subflow`/`heritage`. See Cross-references.

### Stage 07 — SSA

| File | Role | Pipeline/Source anchor |
|---|---|---|
| heritage | `Heritage`: places phi-nodes and renames Varnodes to build SSA form (dominance frontiers, LocationMap of heritaged ranges, multi-pass renaming). | `heritage.hh:207` class Heritage; driven by ActionHeritage("base") `coreaction.cc:5639` |

### Stage 08 — Calls

| File | Role | Pipeline/Source anchor |
|---|---|---|
| fspec | Function-prototype machinery: FuncProto, ProtoModel, ParamList*, FuncCallSpecs modeling calling conventions, param/return storage, and call-site recovery. | `fspec.hh:748` class ProtoModel; class ParamList `fspec.hh:425` |
| modelrules | Datatype-to-storage mapping rules for prototype models: ModelRule, AssignAction subclasses (MultiSlotAssign, HiddenReturnAssign, ConsumeAs, ExtraStack), DatatypeFilter/QualifierFilter. | `modelrules.hh:537` class ModelRule; AssignAction `modelrules.hh:262` |

### Stage 09 — Stack

| File | Role | Pipeline/Source anchor |
|---|---|---|
| varmap | `ScopeLocal` + MapState/RangeHint/AliasChecker reconstructing the stack address-space layout (mapped locals vs parameters) and detecting aliasing. | `varmap.cc` ScopeLocal::restructure; ActionRestructureVarnode `coreaction.cc:5655` |

### Stage 10 — Memory

*No module is primary to Stage 10.* Memory/global heritaging is handled within `heritage` (secondary 10) and volatile-memory effects within `userop` (secondary 10); there is no dedicated memory-analysis translation unit. See Cross-references.

### Stage 11 — Scalar Types

| File | Role | Pipeline/Source anchor |
|---|---|---|
| type | Core type system: `TypeFactory` and the `Datatype` hierarchy (int/float/char/pointer/array/struct/union/enum) with construction, comparison, propagation, and the inference machinery used by ActionInferTypes. | `type.cc`/`type.hh` TypeFactory/Datatype; ActionInferTypes `coreaction.cc:5654` |
| typeop | Associates each p-code opcode with its data-type behavior (getInputCast/getOutputToken/propagateType), driving type flow across ops and operator tokenization. | `typeop.cc:318` TypeOp::propagateType; getInputCast/getOutputToken `typeop.cc:283-296` |
| unionresolve | `ScoreUnionFields` scores data-flow to resolve which union/struct field a Varnode access refers to, producing ResolvedUnion records cached via ResolveCache. | `unionresolve.cc` ScoreUnionFields::run; ResolvedUnion `unionresolve.hh:39` |

### Stage 12 — Aggregate Types

*No module is primary to Stage 12.* Aggregate (struct/array/union) handling is distributed across `type` (the Datatype hierarchy, primary 11), `unionresolve` (field disambiguation, primary 11), and `constseq`/`stringmanage`; no standalone aggregate-type pass module exists. See Cross-references.

### Stage 13 — Variables

| File | Role | Pipeline/Source anchor |
|---|---|---|
| variable | High-level variable model: `HighVariable` (a set of SSA Varnodes for one source variable) plus VariableGroup/VariablePiece for overlapping pieces and HighIntersectTest. | `variable.hh:44-233` HighVariable / VariableGroup |
| merge | The `Merge` engine fusing low-level Varnodes into HighVariables via forced merges (MULTIEQUAL/INDIRECT, globals, mapped stack, address-tied) and speculative merges, inserting COPY trims. | `merge.cc` Merge::mergeAddrTied/mergeMarker; ActionMerge* `coreaction.cc:5870-5891` |
| cover | Cover/CoverBlock describe the topological (def-use) scope of a Varnode; PcodeOpSet tests intersections that gate whether Varnodes can merge into one HighVariable. | `cover.cc` Cover::intersect / addRefPoint; `cover.hh:108` |
| dynamic | `DynamicHash` identifies ephemeral temporary Varnodes (and constants) by hashing local data-flow sub-graphs so dynamic Symbols can attach during dynamic mapping/naming. | `dynamic.cc` DynamicHash::uniqueHash; ActionDynamicMapping `coreaction.cc:5651` |

### Stage 14 — Range

| File | Role | Pipeline/Source anchor |
|---|---|---|
| rangeutil | `CircleRange` (symbolic integer ranges with pull-back/push-forward) plus ValueSet/ValueSetSolver value-set analysis with widening, computing the value set of each Varnode. | `rangeutil.cc` ValueSetSolver::solve / establishValueSets; RuleRangeMeld |

### Stage 15 — Interproc

*No module is primary to Stage 15.* Interprocedural results are emergent: `fspec` prototypes (primary 08) plus per-callsite re-analysis driven by `coreaction.cc` prototype-recovery Actions; `modelrules` (primary 08) also contributes. See Cross-references and Section 5.

---

## Phase 3 — Backend (Structure and Emit)

### Stage 16 — Structuring

| File | Role | Pipeline/Source anchor |
|---|---|---|
| blockaction | Control-flow structuring Actions (ActionBlockStructure, FinalStructure, PreferComplement, StructureTransform, NormalizeBranches, ReturnSplit, NodeJoin) plus CollapseStructure/TraceDAG collapsing the CFG into if/loop/switch. | `blockaction.cc:2170` ActionBlockStructure::apply; FinalStructure `blockaction.cc:2187` |

### Stage 17 — C Lowering

| File | Role | Pipeline/Source anchor |
|---|---|---|
| printlanguage | Language-agnostic emitter base (`PrintLanguage`): walks the structured AST/varnodes, lowers ops/types into an RPN token stack (pushOp/pushVnExplicit/recurse) with precedence/casting. | `printlanguage.hh:138` class PrintLanguage (pushOp:292, recurse:303) |
| cast | CastStrategy/CastStrategyC encoding the C rules for when an explicit cast is required (integer promotion, sign/zext/subpiece-as-cast), used to insert cast ops before emission. | `cast.cc:300` CastStrategyC::castStandard; ActionSetCasts `coreaction.cc:5892` |

### Stage 18 — C Render

| File | Role | Pipeline/Source anchor |
|---|---|---|
| printc | C-language back-end (`PrintC`/PrintCCapability): concrete renderer emitting C expressions, statements, declarations, prototypes, and structured control flow as final tokens. | `printc.hh:66` class PrintC; PrintCCapability::buildLanguage |
| printjava | Alternate Java render back-end (`PrintJava` extends PrintC) overriding constant-pool, LOAD/STORE, and type printing for Java byte-code output. | `printjava.hh:57` class PrintJava : public PrintC |
| stringmanage | StringManager/StringManagerUnicode detect, decode (UTF8/16/32), and cache string data at addresses so constants render as quoted literals. | `stringmanage.cc` StringManagerUnicode::getStringData; `stringmanage.hh:40` |
| prettyprint | Emit-support: pretty-printing/markup engine (Emit, EmitMarkup, TokenSplit) inserting line breaks/indentation and recording AST-to-token markup. | `prettyprint.hh:52` Emit interface |
| comment | Emit-support: comment database (Comment/CommentDatabase) holding per-address high-level comments the renderer interleaves into source. | `comment.hh:43` class Comment |

---

## Infrastructure / Not a decompile stage

These files never appear in `ActionDatabase::universalAction` and supply no per-function pipeline data structure. Grouped by category.

| File | Category | Why |
|---|---|---|
| xml | Serialization & util | Lightweight XML parser for marshaling decompiler data to/from streams. |
| marshal | Serialization & util | Encode/decode framework (AttributeId/ElementId, Encoder/Decoder) for streaming objects in XML/packed form. |
| slaformat | Serialization & util | Encoding constants/helpers for the compiled `.sla` SLEIGH file format. |
| compression | Serialization & util | zlib deflate/inflate wrappers (Compress/Decompress) used by the `.sla` format. |
| crc32 | Serialization & util | CRC32 table and update helper. |
| filemanage | Serialization & util | POSIX file/path search and management helpers (locating `.lspec`/`.sla` files). |
| multiprecision | Serialization & util | Standalone 128-bit integer arithmetic (add/sub/shift/div/compare) serving constant-folding rules; not a stage. |
| architecture | Framework/architecture | Central `Architecture` container wiring translate/database/types/loadimage/options/actions; owns context, not a single pass. |
| action | Framework/architecture | Base `Action`/`Rule`/`ActionGroupList`/`ActionDatabase` machinery; the pass-engine framework that `coreaction.cc` populates. |
| coreaction | Pipeline orchestration | Hosts `ActionDatabase::universalAction` (`coreaction.cc:5609-5896`, the canonical 19-stage pass order) plus ~67 concrete Action classes spanning nearly every Phase-2/3 stage. Too cross-cutting for one stage — see Cross-references for the stages it drives. |
| capability | Framework/architecture | `CapabilityPoint` extension-registration infrastructure (static singleton plugin discovery). |
| options | Framework/architecture | Architecture configuration-option processing (ArchOption classes and marshaling elements). |
| graph | Framework/architecture | Dataflow/control-flow/dominator graph dumping utilities for visualization/debug. |
| database | Framework/architecture | Symbol table — Symbol, Scope, SymbolEntry, Database (local/global scopes, namespaces) consumed across the pipeline. |
| cpool | Framework/architecture | Constant-pool support (ConstantPool/CPoolRecord) for deferred-compilation languages such as Java byte-code. |
| callgraph | Framework/architecture | Inter-function CallGraph/CallGraphNode/CallGraphEdge structure used by the console driver, not a single pass. |
| libdecomp | Framework/architecture | Library entry point — startDecompilerLibrary/shutdownDecompilerLibrary registering all capabilities and SLEIGH specs. |
| override | Framework/architecture | Per-function user directives (force-goto, deadcode delay, indirect/proto overrides, flow overrides) applied across restarts; config container. |
| interface | Console UI | Generic command-line interface framework (IfaceStatus/IfaceCommand/IfaceCapability). |
| ifacedecomp | Console UI | Decompiler-specific console commands (IfaceDecompCapability, decompile/print command set). |
| ifaceterm | Console UI | Terminal/readline capabilities layered on the CLI. |
| consolemain | Console UI | `main()` driver for the standalone decompiler console. |
| codedata | Console UI | Console-driven byte-level code/data flow tooling (CodeDataAnalysis, CodeUnit), separate from the decompile pipeline. |
| semantics | SLEIGH compiler & parsers | P-code semantic templates (ConstTpl, HandleTpl, OpTpl) used when compiling/expanding SLEIGH constructors. |
| pcodecompile | SLEIGH compiler & parsers | Compiles p-code semantic sections (PcodeCompile, ExprTree, Location) from SLEIGH source into templates. |
| pcodeparse | SLEIGH compiler & parsers | Lexer/parser for compiling standalone p-code snippets against an existing SLEIGH language. |
| grammar | SLEIGH compiler & parsers | C-type declaration grammar lexer/parser (GrammarToken/GrammarLexer) for user datatype/prototype text. |
| slgh_compile | SLEIGH compiler & parsers | High-level control of compiling `.slaspec` specs into `.sla` (SleighCompile driver). |
| slghparse | SLEIGH compiler & parsers | Generated Bison grammar interface for parsing SLEIGH specification source. |
| slghscan | SLEIGH compiler & parsers | Generated Flex lexer for tokenizing SLEIGH specification source. |
| slghsymbol | SLEIGH compiler & parsers | SLEIGH symbol-table classes (SleighSymbol hierarchy, SymbolTable) built during spec compilation. |
| slghpatexpress | SLEIGH compiler & parsers | SLEIGH pattern-expression machinery (TokenPattern, PatternExpression) for constructor matching. |
| slghpattern | SLEIGH compiler & parsers | Low-level bitstream mask/value pattern classes (PatternBlock, Pattern) for instruction matching. |
| rulecompile | SLEIGH compiler & parsers | Lexer/compiler (RuleLexer) for the experimental Rule specification language (CPUI_RULECOMPILE). |
| unify | SLEIGH compiler & parsers | Pattern-unification datatypes (UnifyDatatype, UnifyCPrinter) backing the rule-compiler that emits C++ Rule code. |
| inject_sleigh | P-code injection | P-code injection implemented via the internal SLEIGH engine (InjectContextSleigh/PcodeInjectLibrarySleigh). |
| inject_ghidra | P-code injection | P-code injection marshaling context to a Ghidra client to obtain injected p-code. |
| ghidra_arch | Ghidra-IPC glue | ArchitectureGhidra and the command/marshaling protocol connecting the decompiler to a Ghidra client. |
| ghidra_context | Ghidra-IPC glue | ContextDatabase fetching tracked-register context from a Ghidra client. |
| ghidra_process | Ghidra-IPC glue | Command dispatch loop (GhidraCommand/GhidraCapability) implementing the request protocol. |
| ghidra_translate | Ghidra-IPC glue | Translate implementation querying a Ghidra client for instruction p-code. |
| comment_ghidra | Ghidra-IPC glue | CommentDatabase backed by querying a Ghidra client for function comments. |
| cpool_ghidra | Ghidra-IPC glue | ConstantPool backed by a Ghidra client via XML queries. |
| database_ghidra | Ghidra-IPC glue | Scope implementation querying a Ghidra client for Symbol info (the global scope). |
| loadimage_ghidra | Ghidra-IPC glue | LoadImage fetching program bytes from a Ghidra client. |
| string_ghidra | Ghidra-IPC glue | StringManager (GhidraStringManager) fetching/decoding string data via a Ghidra client. |
| typegrp_ghidra | Ghidra-IPC glue | TypeFactoryGhidra overriding findById to marshal data-type requests to a Ghidra client; pure IPC bridge. |
| signature_ghidra | Ghidra-IPC glue / Signatures | GhidraSignatureCapability registering feature-vector generation commands issued by the Ghidra client. |
| emulate | Emulator | P-code emulation engine (Emulate, BreakTable, breakpoints) for executing p-code; not part of the decompile pass. |
| emulateutil | Emulator | Lightweight emulation of PcodeOp/Varnode syntax-tree snippets built on the emulate engine. |
| memstate | Emulator | Memory-state bookkeeping (MemoryBank, page/hash overlays) backing the p-code emulator. |
| signature | Signatures | Generates BSim feature/signature hash vectors from a decompiled function's graphs; downstream consumer, not a stage. |
| analyzesigs | Signatures | Console commands for feature/signature generation (IfaceAnalyzeSigsCapability, signature print/save). |
| paramid | Signatures / analysis tool | Standalone parameter-identification (ParamMeasure/ParamIDAnalysis) ranking input/output storage; a separate export tool, not in universalAction. |
| test | Test harness & examples | Minimal unit-test framework (TEST/ASSERT macros and runner). |
| testfunction | Test harness & examples | Data-driven single-function decompile test framework (FunctionTestProperty, regex matching over output). |
| sleighexample | Test harness & examples | Standalone demo dumping raw p-code for a hard-coded x86 binary via SLEIGH. |

---

## Cross-references

Files whose logic materially spans multiple stages. Each is listed once in a primary table above (or in Infrastructure); the **secondary** stages below indicate additional pipeline phases they participate in.

| File | Primary | Secondary stages | Note |
|---|---|---|---|
| coreaction | INFRA (orchestration) | 05, 06, 07, 08, 11, 13, 15, 16 | Hosts `ActionDatabase::universalAction` (the 19-stage pass order, `coreaction.cc:5609-5896`) plus ~67 Action classes spanning nearly every Phase-2/3 stage — heritage trigger, deadcode, type-infer trigger, prototype recovery (ActionActiveParam:1754, ActionReturnRecovery:1937, ActionInputPrototype:4852, ActionOutputPrototype:4910), stackptr-flow, block-structure, merge/naming, casts. Too cross-cutting for one stage_id. |
| translate | 01 | 02, 03, 04 | Decode interface (oneInstruction/printAssembly), but AddrSpaceManager/JoinRecord/SpacebaseSpace are space/IR infra (04) and PcodeEmit feeds lift (02). |
| sleigh | 01 | 02, 03 | Decode engine that also emits the p-code (02) and assembly (03) for each instruction. |
| sleighbase | 01 | INFRA | Mostly `.sla` spec-file loading (build-time infra) but instantiates the runtime Translate model. |
| sleigh_arch | 01 | INFRA | Mostly `.ldefs`/spec loading; instantiates the runtime Sleigh engine and loader selection. |
| pcoderaw | 02 | 04 | Raw lifted-pcode data that becomes the IR Varnode/PcodeOp containers. |
| opbehavior | 02 | 05 | Op semantics reused heavily by simplify constant folding (and the emulator). |
| float | 02 | 05 | IEEE754 format support; consumed by float constant-folding in simplify. |
| userop | 02 | 10, 17 | CALLOTHER op definitions (02); models volatile reads/writes as memory effects (10); controls printing (17). |
| flow | 03 | 02, 08 | Generates raw p-code (02) and discovers FuncCallSpecs / sets up call specs (08) during flow following. |
| pcodeinject | 03 | 02, 08 | Splices injected p-code (02), notably call-fixups (08). |
| jumptable | 03 | 14, 01 | Switch recovery depends on rangeutil value-set analysis (14) and feeds decode/flow (01). |
| funcdata_block | 04 | 16, 03 | BlockGraph container (04) but also installs jump tables (03) and supports block structuring (16). |
| funcdata | 04 | 07, 08, 16 | The object every pass mutates; coordinates heritage (07), call recovery (08), structuring (16). |
| funcdata_op | 04 | 02, 06, 07 | Op-edit API used by lift fixups (02) and all data-flow/SSA passes (06/07). |
| funcdata_varnode | 04 | 07, 09, 13 | Varnode-edit API used throughout heritage (07), stack mapping (09), and merge (13). |
| varnode | 04 | 07 | IR container whose heritage flags (activeheritage/written/input) and VarnodeBank tie to SSA (07). |
| block | 04 | 16 | BlockBasic is a pure IR container (04); the structured-block hierarchy is consumed by structuring (16). |
| heritage | 07 | 06, 09, 10 | SSA construction (07) that also drives data-flow (06) and heritages stack (09) and global memory (10) ranges. |
| ruleaction | 05 | 06, 11, 12 | Simplify rules that also normalize data-flow (06) and pointer/aggregate access (11/12). |
| subflow / condexe / double / expression / transform | 05 | 06 | Simplify-cluster analyses operating on the data-flow graph (06). |
| constseq | 05 | 12 | Folds constant store sequences; touches aggregate/array types (12). |
| bitfield | 05 | 12 | Bit-field normalization touching aggregate/struct field types (12). |
| prefersplit | 05 | 13 | Register splitting that affects variable modeling (13). |
| fspec | 08 | 15, 09, 13 | Prototype machinery load-bearing for interproc (15), stack/param storage (09), and variable recovery (13). |
| modelrules | 08 | 15, 11, 12 | Datatype-to-storage rules feeding interproc (15) and type-driven storage (11/12). |
| varmap | 09 | 13, 10 | Reconstructs stack layout (09) and builds local Symbols / name recommendations (13); aliasing touches memory (10). |
| type | 11 | 12 | Whole type system; the Datatype hierarchy includes aggregates (12). |
| typeop | 11 | 17, 02 | Type-propagation table (11), render-time token/cast metadata (17), inherits p-code behavior (02). |
| unionresolve | 11 | 12 | Field disambiguation over union/struct aggregates (12). |
| cast | 17 | 11 | Cast-insertion decisions (ActionSetCasts, backend lowering) derived from the type system (11). |
| stringmanage | 18 | 12 | String rendering (18) recognizing char-array/aggregate string data (12). |
| printlanguage | 17 | 18 | Abstract emitter (17) that printc/printjava specialize for final render (18) — the 17/18 split here is one inheritance chain, not two passes. |
| printc / printjava | 18 | 17 | Concrete renderers (18) built on the PrintLanguage lowering base (17). |
| signature | INFRA | — | (Multi-claimed: resolved to INFRA, BSim feature generation, not a pipeline stage.) |
| signature_ghidra | INFRA | — | (Multi-claimed: resolved to INFRA Ghidra-IPC glue registering signature commands.) |

---

## Stages that do not cleanly map

- **Stage 06 Dataflow** and **Stage 10 Memory** have no dedicated translation unit. Dataflow normalization (dead-code, COPY/MULTIEQUAL propagation) is realized as Action classes inside `coreaction.cc` operating on the Stage-04 IR, with the transforms living in `ruleaction`/`subflow`/`heritage`. Memory analysis is folded into SSA heritaging of stack/global ranges (`heritage`, secondary 10) and volatile-memory modeling (`userop`, secondary 10).
- **Stage 12 Aggregate Types** is not a separate module. Struct/array/union handling is embedded in `type` (the Datatype hierarchy) and `unionresolve` (field selection), with `constseq`/`bitfield`/`stringmanage` contributing aggregate-aware transforms. It is a sub-concern of the type system rather than a distinct pass.
- **Stage 15 Interproc** is emergent, not modular. It arises from `fspec` prototype models combined with per-callsite re-analysis orchestrated by prototype-recovery Actions in `coreaction.cc` (the decompiler iteratively re-runs the per-function pipeline as caller/callee prototypes firm up); `modelrules` supplies the storage rules. No file is dedicated solely to cross-function propagation.
- **Stage 01 Decode vs Stage 02 P-code Lift** overlap inside `sleigh`/`translate`: a single `oneInstruction` call both decodes the instruction (01) and emits its p-code (02). The split is conceptual — `translate`/`sleigh` are placed at 01 (decode contract/engine) while the p-code *vocabulary and data* (`opcodes`/`pcoderaw`/`opbehavior`/`float`/`userop`) sit at 02.
- **Stage 00 Loader is thin**: `loadimage*` are small byte-supplier classes, and the three `*_arch` capabilities are mostly architecture-selection glue whose decompiler-facing duty is `buildLoader()`; they straddle Loader and INFRA (kept at 00 with INFRA secondary). `loadimage_ghidra` is pure IPC and classed INFRA.
- **`sleighbase`/`sleigh_arch`** straddle the runtime pipeline and build-time spec loading; kept at Stage 01 (they instantiate the runtime Translate engine) with INFRA secondary, since `.sla`/`.ldefs` parsing is closer to offline infrastructure than per-function decompilation.