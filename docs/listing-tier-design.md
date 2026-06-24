# kuna Listing/Xref Tier — Implementation Plan

> **Transcriber's note (folded-in corrections):** the settable count test asserts **34**
> (not 31 as a stale header comment said), and the no-return commit path uses
> `NoReturnFact { addr, name }` resolved via `find_function_across_scopes` /
> `query_global_function` then `set_function_no_return` — it is proven and exists.

**Status:** spec for the implementation fan-out. Every file:line below is verified against the working tree at the start of this task.

**Goal.** Add an optional, default-OFF post-disassembly Listing/xref subsystem to the `kuna-analysis` crate. It performs program-wide recursive-descent disassembly over loadimage bytes — reusing the ported SLEIGH decoder (`Sleigh::one_instruction`) and a *lifted copy* of the S2 flow classifier (`s2_lift/flow.rs`) — to build three sub-models (instruction / cross-reference / discovered-function). It is the keystone that unlocks three Ghidra analyzers kuna cannot port today. It runs **only on the real-ELF bootstrap path**, never the XML datatest path, so the 675/675 + 158/158 parity oracles are structurally untouched. It ships as a sequence of small PRs, each with a fixture testcase and all three gates green.

The plan **adopts DRAFT B's three-sub-model faithfulness** (the consumers were written against `Listing`/`ReferenceManager`/`FunctionManager` as separate contracts, and collapsing them breaks consumer transliteration) but **adopts DRAFT A's tighter PR sequencing and its "build-at-load, gated, in-memory-not-a-fact" stance**. Where the two drafts disagreed on build timing, this plan resolves it explicitly in §5.3.

---

## 1. Module layout & invocation

### 1.1 New module tree

All new code lives in the `kuna-analysis` crate:

```
decompiler/crates/kuna-analysis/src/listing/
  mod.rs        # Listing facade + Listing::build(file, image, arch, translate, seeds); re-exports
  model.rs      # Insn, FlowType/FlowKind, Reference/RefKind, DiscoveredFunction, CodeUnit
  decode.rs     # decode_one(): drive Sleigh::one_instruction at a VMA + capture ops/mnemonic
  classify.rs   # classify(): the lifted xref_control_flow switch (the single source of truth)
  walk.rs       # recursive-descent driver: VisitStat map + intra/inter-function worklists
  context.rs    # decode-context resolution (ARM TMode / MIPS ISA_MODE) before decode
```

The three sub-models (instruction / xref / function) live as fields of one `Listing` facade in `mod.rs` over types in `model.rs` — they share a single decode pass. We keep them queryable as three distinct API surfaces (§6) so each consumer's upstream code transliterates method-for-method.

### 1.2 Touched existing files (additive)

| File | Change | Verified anchor |
|---|---|---|
| `kuna-analysis/src/lib.rs` | `pub mod listing;` | module list ~`lib.rs:46-58` |
| `kuna-analysis/src/pass.rs` | add `pub listing: Option<&'a Listing>` to `AnalysisCtx` | `pass.rs:292` |
| `kuna-analysis/src/passes.rs` | build the Listing once (flag-gated) before the pass loop in both drivers; thread `translate` param | `passes.rs:175`, `passes.rs:197` (the two `let ctx = AnalysisCtx {...}` sites) |
| `kuna-analysis/src/s1_entry/mod.rs` | promote `executable_sections` (`:224`), `existing_function_addrs` (`:257`), `in_executable_section` (`:249`) to `pub(crate)` (`collect_entries` `:132` is already `pub`) | confirmed private today |
| `kuna-decomp/src/infra/architecture.rs` | the `analysis_listing` flag (field/default/reset/`set_kuna_option` arm) | field block `:360-381`, defaults `:574-575`, reset `:667-668`, `set_kuna_option` analysis arms `:759-771` |
| `kuna-decomp/src/p0_knowledge/options.rs` | `"listing"` in `KUNA_OPTION_NAMES` | allowlist (`KUNA_OPTION_NAMES`) |
| `kuna-console/src/engine.rs` | thread `translate` into the driver call; add `"listing" => arch.analysis_listing` to `analysis_pass_enabled` | driver call `engine.rs:634-635`, `analysis_pass_enabled` `engine.rs:271-285` |
| `kuna-decomp/stages.toml` | a `[[settable]]` analysis-enablement row | analysis-gates block `:1805+` (addrtable row at `:1807`) |
| `kuna-decomp/src/p0_knowledge/kuna_stages/tests.rs` | bump `settable_count_is_34` → `35` | `tests.rs:31,37,38` |
| `docs/assertions.md` | regenerate via `kuna catalog --markdown` | — |

### 1.3 Invocation (built once, shared with consumer passes)

`Listing::build` is invoked **inside `run_default_analyses_per_pass`** (`passes.rs:188`), the real-ELF-only driver, whose sole caller is `bootstrap_from_elf` (`engine.rs:547`) via the driver call at `engine.rs:634-635`. The decoder is in hand there: `sleigh.base().unwrap()` is already passed as the third arg (`engine.rs:635`). We add a fourth arg `translate: &dyn Translate` (the same `sleigh.base().unwrap().translate()` the bootstrap already holds), and in the driver:

```rust
let seeds = { /* existing_function_addrs(&file) ∪ collect_entries(&file), exec-filtered, sorted/deduped */ };
let listing = arch.analysis_listing
    .then(|| crate::listing::Listing::build(&file, image, arch, translate, &seeds));
let ctx = AnalysisCtx { file: &file, image, arch, listing: listing.as_ref() };
```

The `Listing` is owned by the driver, outlives the pass loop (same lifetime shape as `file`), and is borrowed read-only by every consumer pass via `ctx.listing`. It decodes eagerly into owned `model.rs` structs and drops the `translate` borrow when `build` returns — it holds no long-term engine borrow. This honors the `AnalysisPass::run(&self, ctx: &AnalysisCtx)` "pure-over-ctx, additive, never-failing" contract (`pass.rs:309`): the Listing is just a richer read-only ctx borrow.

`run_default_analyses` (the non-per-pass driver, `passes.rs:163`) gets the same fourth param for signature symmetry but, since only `run_default_analyses_per_pass` is wired into the live engine (`engine.rs:635`), the build call only needs to be live in the per-pass driver. Add the param to both; build the Listing only where a caller passes a real translate.

---

## 2. Core data model (`model.rs`)

Three sub-models behind one facade. Each field is justified by a buildable consumer (Consumer A) or a near-term one (the CodeUnit partition feeds both A's "fell into data" signal and a future AIF gap walk).

### 2.1 Instruction model

```rust
/// One decoded machine instruction. Owned; produced by the walker.
pub struct Insn {
    pub addr: u64,                  // getMinAddress
    pub len: u32,                   // bytes consumed incl. delay slots (one_instruction return)
    pub fall_through: Option<u64>,  // None iff !flow.has_fallthrough (terminal)
    pub flow: FlowType,             // classified from emitted pcode — NOT a separate API result
    pub flows: Vec<u64>,           // static control targets (getFlows): branch/call dest VMAs
    pub mnemonic: String,           // x86 INT3, ARM nop/mov/ld*, ELF add/push dispatch
    pub pcode: Option<Vec<RawOp>>, // lazy: only skipNOPS / isUsedForCalculation need it
}

/// Faithful projection of Ghidra's FlowType predicate set.
pub struct FlowType {
    pub kind: FlowKind,
    pub is_call: bool,
    pub is_jump: bool,
    pub is_terminal: bool,
    pub is_computed: bool,     // indirect/computed target (BRANCHIND/CALLIND)
    pub is_indirect: bool,
    pub is_conditional: bool,
    pub has_fallthrough: bool,
}

/// OpCode → flow-class projection (mirrors typeop.rs:370-405 / op.rs:73 flags).
pub enum FlowKind {
    Fallthrough,           // emits ops; last op not a flow break
    UnconditionalBranch,   // last op CPUI_BRANCH (4), in0 non-constant
    ConditionalBranch,     // CPUI_CBRANCH (5) — also falls through
    ComputedJump,          // CPUI_BRANCHIND (6) — indirect, target unresolved
    Call,                  // CPUI_CALL (7) — falls through
    ComputedCall,          // CPUI_CALLIND (8) — indirect, falls through
    CallOther,             // CPUI_CALLOTHER (9) — syscall/userop, falls through
    Return,                // CPUI_RETURN (10) — terminal
    Unimplemented,         // decode Err(Unimpl): has length, no flow effect
}

/// Minimal captured op (only when pcode is materialized).
pub struct RawOp { pub opcode: OpCode, pub in0: Option<VarnodeData> /* enough for skipNOPS */ }
```

*Why `FlowType` carries the predicate set, not just `FlowKind`:* Consumer A queries `hasFallthrough() && !isFallthrough()` (a distinct test from `isTerminal()`), `isCall`, `isComputed`, `isIndirect` individually. The predicates are derived once from the OpCode + the `pcodeop_flags` table (the same flags `set_opcode` ORs onto each op, `substrate/op.rs:73,458-477`), so this is the same bit-logic Ghidra uses, not a re-classification. *Why `pcode` is `Option`/lazy:* `one_instruction` produces ops during decode; only `skipNOPS`/`isUsedForCalculation` (a niche of Consumer A, and Consumers B/C which are deferred) read them. Keep `None` in the common case.

### 2.2 Cross-reference model

```rust
pub enum RefKind { Call, Code /* branch/jump */, Data, Read, Write }

pub struct Reference {
    pub from: u64,
    pub to: u64,
    pub kind: RefKind,
    pub op_index: Option<u8>,   // present-but-mostly-None; reserved for operand markup
}
```

The keystone **populates only `Call` and `Code` edges** (control-flow), generated during the walk from each `Insn.flows`. `Data/Read/Write` and `op_index` are in the type for faithfulness so the type never changes if operand markup is ever revisited (only population density would) — but **no reference-writing API is exposed** (Consumer B is declined; §8). The model is a bidirectional multimap (`refs_to` + `refs_from`), because Consumer A reads `to`-wards (callers of X) and a future operand consumer reads `from`-wards.

### 2.3 Discovered-function model

```rust
pub struct DiscoveredFunction {
    pub entry: u64,
    pub name: Option<String>,         // from funcsym / entry_names overlay; None ⇒ sub_<addr>
    pub from_symbol: bool,            // seeded by a real funcsym (existing_function_addrs) vs discovered via CALL
    pub has_no_return: bool,          // seeded from s1_loader::noreturn Known-list (cheap; refined by Consumer A)
    pub call_fixup: Option<String>,   // from s1_callfixup facts (skip-modeled-callees)
}
```

Backed by `BTreeMap<u64, DiscoveredFunction>` so `function_containing` / `next_function_after` are ordered interval lookups (`range(..=vma).next_back()` / `range(vma+1..).next()`) — the faithful analog of Ghidra's ordered FunctionManager and Consumer A's strongest signal.

### 2.4 CodeUnit partition

```rust
pub enum CodeUnit { Instruction(u64 /*insn addr*/), Data(u64, u32), Undefined }
```

Computed once at end of build: every VMA inside an executable range (`s1_entry::executable_sections`) that is not the start-or-interior of a decoded `Insn` and not a symbol-typed data record is `Undefined`. At this tier "Data" comes from the symbol/section model + `.rodata` strings (`s1_strings`); the conservative partition is `Instruction` (decoded) / `Data` (non-exec or symbol-typed) / `Undefined` (everything else in exec ranges). This is faithful enough for the two relevant consumers: A's "fell into data" reduces to `!is_instruction_start(fall_vma)`; AIF's gap walk uses `first_undefined_after`.

### 2.5 The facade

```rust
pub struct Listing {
    insns: BTreeMap<u64, Insn>,                 // instruction model
    refs_to: BTreeMap<u64, Vec<Reference>>,     // xref: incoming (callers)
    refs_from: BTreeMap<u64, Vec<Reference>>,   // xref: outgoing
    funcs: BTreeMap<u64, DiscoveredFunction>,   // ordered function model
    covered: RangeList,                         // kuna_base::address::RangeList — instruction coverage
    exec_ranges: Vec<(u64, u64)>,               // coverage universe for the partition / gaps
}
```

`RangeList` is the same type `get_readonly`/`engine.rs:600` already use (insert/subtract/iterate), so gap = `exec_ranges` minus `covered` is free when PR8 needs it.

---

## 3. The recursive-descent algorithm

### 3.1 Seeding

Root set = `existing_function_addrs(file) ∪ collect_entries(file)`, both already exec-section-filtered (`collect_entries` filters; `existing_function_addrs` are real funcsyms), then sorted/deduped. `existing_function_addrs` (`s1_entry/mod.rs:257`) = `.symtab`/`.dynsym` FUNC + PLT stubs; `collect_entries` (`s1_entry/mod.rs:132`) = e_entry ∪ DT_INIT/FINI arrays ∪ `.eh_frame` FDE pcBegin ∪ `_start`→`main` idiom ∪ x86-64 prologue patterns, funcsym-dedup'd. Each seed becomes a `DiscoveredFunction { from_symbol = (came from existing_function_addrs), name = funcsym/entry-overlay name }` and goes on the **function worklist**.

### 3.2 Decode one instruction (the decoder reuse — see §4)

`decode.rs::decode_one(translate, vma, code_space) -> Result<(len, Vec<RawOp>, mnemonic), DecodeErr>`. Builds the `Address` in the code space, paints context (§4.2), drives `Translate::one_instruction` with a capturing `PcodeEmit`, and `print_assembly` with a capturing `AssemblyEmit` for the mnemonic.

### 3.3 Successor computation (lifted classifier — see §4.3 and `classify.rs`)

`classify(ops, vma, len) -> (FlowType, flows: Vec<u64>, fall_through: Option<u64>)`. A transliteration of `s2_lift/flow.rs::xref_control_flow` (`flow.rs:1039-1185`). Exact rules in §4.3.

### 3.4 The worklist (two-level, the program-wide extension)

Mirrors `FlowInfo`'s `VisitStat`/`addrlist` *design* (`flow.rs:155,425`) but lightweight (no `Funcdata`, no banks, no blocks):

```
func_worklist  = seeds                          # function-entry worklist
visited_funcs  = {}
while let Some(entry) = func_worklist.pop():
    if entry in visited_funcs: continue
    visited_funcs.insert(entry)
    insn_worklist = [entry]                      # intra-function fall-through/branch walk
    while let Some(vma) = insn_worklist.pop():
        if vma in insns: continue               # the VisitStat dedup (overlap detection free)
        if vma not in any exec_range: continue   # out-of-bounds gate (flow.rs:891 analog)
        match decode_one(translate, vma):
            Err(Unimpl|BadData) => continue      # decode-error policy: stop this path, mark gap
            Ok((len, ops, mnem)):
              (flow, targets, fall) = classify(ops, vma, len)
              insns.insert(vma, Insn { addr:vma, len, flow, flows:targets.clone(), fall_through:fall, mnemonic:mnem, pcode:None })
              covered.insert(vma, vma + len - 1)
              for t in targets:
                  if flow.is_call:                          # CALL/CALLIND target → NEW function entry
                      funcs.entry(t).or_insert(discovered); func_worklist.push(t)
                      file_ref(vma, t, RefKind::Call)
                  else:                                      # branch target → same-function successor
                      insn_worklist.push(t)
                      file_ref(vma, t, RefKind::Code)
              if let Some(f) = fall:
                  insn_worklist.push(f)
                  file_ref(vma, f, RefKind::Code)            # fall-through edge
# after the loop:
partition_code_units()   # exec_ranges minus covered → Undefined
```

`file_ref(from,to,kind)` pushes into both `refs_to[to]` and `refs_from[from]`.

**Key choices, each grounded:**
- **Cross-function driver** is the one thing `FlowInfo` lacks (it stops at RETURN, treats CALL as fall-through, never recurses into callees): CALL/CALLIND targets become **new function entries** on `func_worklist`. This is the program-wide walk.
- **Indirect/unresolved targets:** `ComputedJump` (BRANCHIND) and `ComputedCall` (CALLIND) push **no static successor** (and CALLIND seeds no new entry — target unknown). They are *recorded* (so a consumer sees "there is computed flow here" via `is_computed`/`is_indirect`) but unresolved. This is the documented "partial flow, assume no out-branches" path the engine itself takes (`flow.rs:1523-1525` `findJumpTable==0`). **Jump-table resolution is deferred** (§8): it is intrinsically post-dataflow (needs the action pipeline + `EmulateFunction` over the SSA tree, `s2_lift/jumptable.rs`); no cheap pre-decompile form exists.
- **Termination:** `visited_funcs` bounds the function worklist; `insns` membership bounds the instruction worklist; both are monotonic over a finite address universe ⇒ terminates.
- **Coverage:** `covered: RangeList` accrues `[vma, vma+len-1]` per decoded insn; gap = `exec_ranges − covered` (PR8).

---

## 4. The decoder reuse

### 4.1 Driving SLEIGH for one instruction outside the decompiler

The engine returns **length + a flat list of p-code ops** via a `PcodeEmit` sink; there is no `getFlow()`. Drive it with a capturing sink:

```rust
struct OpCapture { ops: Vec<RawOp> }
impl PcodeEmit for OpCapture {
    fn dump(&mut self, _addr:&Address, opc:OpCode, _out:Option<&VarnodeData>, vars:&[VarnodeData]) {
        self.ops.push(RawOp { opcode: opc, in0: vars.first().cloned() });
    }
}
// decode:
let addr = Address::new(Rc::clone(code_space), vma);
let mut cap = OpCapture::default();
let len = translate.one_instruction(&mut cap, &addr)?;   // returns fall-through byte length incl. delay slots
```

`PcodeEmit::dump` signature is verified at `translate.rs:166`: `fn dump(&mut self, addr:&Address, opc:OpCode, outvar:Option<&VarnodeData>, vars:&[VarnodeData])`. The mnemonic comes from a parallel `AssemblyEmit` capture via `translate.print_assembly(&mut asm_cap, &addr)` (`AssemblyEmit::dump(addr, mnem, body)` at `translate.rs:217`). The code space comes from the engine's space manager (`get_default_code_space` / `manage().get_space_by_name("ram")`).

`Translate` is the public trait (`translate.rs:386`); `one_instruction` (`:472`), `instruction_length` (`:459`), `print_assembly` (`:481`). These are `&self` (interior mutability) — no engine setup beyond what `bootstrap_from_elf` already did (`.sla` loaded, `ObjectLoadImage` attached as the loader).

### 4.2 Context-mode gotchas (`context.rs`)

Context (ARM `TMode`, MIPS `ISA_MODE`/MIPS16) **must be painted before decoding at an address**, or you get an A32-vs-Thumb misdecode. kuna already produces these paints: `s1_loader/arm_markers.rs` (ARM TMode) and `s1_loader/mips_markers.rs` (MIPS ISA_MODE) emit `ContextPaint`/`tracked_regs` facts (`pass.rs:257`). `context.rs` resolves, per seed/per address, the right context value and paints it via the context DB (`set_variable(name, addr, value)`, reached through the engine's context DB the way `verify_arm_thumb_decode.rs` does) before the first decode at that address. x86-64 needs nothing — **PR3-PR5 target x86-64 only**; the context-paint correctness (PR6) is isolated to ARM/MIPS and gated off by default.

The decode also self-applies context *commits* the constructor itself sets (e.g. an ARM mode switch on `bx`) inside `one_instruction` (`apply_commits`) — those are handled by the engine, not us.

### 4.3 The exact classifier (`classify.rs`)

A transliteration of `xref_control_flow` (`flow.rs:1039-1185`). Two load-bearing rules below.

```
fall_vma = vma + len
last_op  = ops.last()
flows = []
for (opcode, in0) in ops:
  CPUI_CBRANCH(5):  if in0.is_constant() => intra-insn relative (resolves to fall-through; ignore)
                    else => flows.push(in0.get_addr().get_offset()); kind = ConditionalBranch; is_conditional=true
  CPUI_BRANCH(4):   if in0.is_constant() => relative (ignore)
                    else => flows.push(in0 offset); kind = UnconditionalBranch
  CPUI_BRANCHIND(6):kind = ComputedJump; is_jump=true; is_computed=true; is_indirect=true   # no static target
  CPUI_CALL(7):     flows.push(in0 offset); kind = Call; is_call=true                        # target == in0 (flow.rs:1765)
  CPUI_CALLIND(8):  kind = ComputedCall; is_call=true; is_computed=true; is_indirect=true     # no static target
  CPUI_CALLOTHER(9):kind = CallOther
  CPUI_RETURN(10):  kind = Return; is_terminal=true
  else:             no flow effect

# fall-through (flow.rs:1170-1183): falls through UNLESS the LAST op ∈ {BRANCH, BRANCHIND, RETURN}
has_fallthrough = !matches!(last_op.opcode, CPUI_BRANCH|CPUI_BRANCHIND|CPUI_RETURN)
fall_through    = has_fallthrough.then_some(fall_vma)
# an instruction that emits NO ops falls through (has_fallthrough = true)
```

**Gotcha 1 — constant-space in0 is p-code-relative, not a VMA.** Always test `in0.is_constant()` first. A constant-offset BRANCH/CBRANCH is intra-instruction (a multi-pcode-op instruction branching within itself); for single-instruction flow it resolves to fall-through, never a real target. Only a non-constant in0 carries an absolute target VMA (`flow.rs:768-790`). `VarnodeData::get_addr()` is at `pcoderaw.rs:143`; `Address::is_constant()`/`get_offset()` confirmed.

**Gotcha 2 — fall-through is decided by the LAST op, not the first.** A multi-op instruction's *targets* come from every op, but its *fall-through* is the last-op test only. CBRANCH/CALL/CALLIND/CALLOTHER all fall through; BRANCH/BRANCHIND/RETURN do not.

**Gotcha 3 — delay slots** (SPARC/MIPS) are already folded into `len` by `one_instruction` (`sleigh.rs:2064-2083`), so `fall_vma = vma + len` is correct — do **not** decode the delay slot again.

A golden test (PR2) pins `classify` against `flow.rs` for the same bytes, so the two stay in sync by construction.

---

## 5. The `--option listing` flag (default OFF) + parity safety

### 5.1 Wiring (follows the `addrtable`/`formatstring` template exactly)

- **`architecture.rs`**:
  - field, in the `analysis_*` block after `analysis_formatstring` (`:381`): `pub analysis_listing: bool,`
  - constructor default after `:575`: `analysis_listing: false,`
  - `reset_defaults_internal` after `:668`: `self.analysis_listing = false; // Listing/xref tier default-off`
  - `set_kuna_option` arm in the analysis block after `:771`: `"listing" => on_off!(analysis_listing, "Listing/xref disassembly tier"),`
- **`options.rs`**: add `"listing",` to `KUNA_OPTION_NAMES`.
- **`engine.rs::analysis_pass_enabled`** (`:271-285`): add `"listing" => arch.analysis_listing,`. The default arm is `_ => true` (fail-OPEN, `:284`), so this MUST be added explicitly, and so must any consumer's id (PR7).
- **`stages.toml`**: a `[[settable]]` row in the analysis-gates block (after the `formatstring` row at `:1823`), copying the `addrtable` shape (`:1807`): `option="listing"`, `values="on|off"`, `default="off"`, `stage`/`substage`/`strength`/`rewind`, `summary`, `use_when`, `example`, `source_decompiler`, `change_kind="analysis-enablement"`, **no `live_field`** (analysis gates flip a plain bool). Fix the stale header count comment `stages.toml:8` (currently "settable=31", actually 34 → make it 35).
- **count test**: bump `settable_count_is_34` → `35` at `tests.rs:31,37,38` (and the header comment at `tests.rs:4`).
- **docs**: `make binaries` then `kuna catalog --markdown > docs/assertions.md`. The `build.rs` codegen regenerates `SETTABLE_TABLE`/`OptionValues` from `stages.toml` at compile time automatically.
- **drift gate**: `kuna catalog --check` cross-checks the catalog against `KUNA_OPTION_NAMES` in-process; run it before committing.

### 5.2 Parity-safety argument (three independent layers)

1. **Structural (the real one).** `Listing::build` is invoked only from `run_default_analyses_per_pass` (`passes.rs:188`), whose only caller is `bootstrap_from_elf` (`engine.rs:547`, driver call `:634-635`), reached only via the `\x7fELF` magic dispatch in `bootstrap_from_file` (`engine.rs:1021`). The XML datatest path (`bootstrap_program`/`bootstrap_from_root`) never calls the analysis driver and never builds `pending_analysis`. **No datatest can build or observe a Listing.** The 675/675 oracle runs on a code path the Listing tier does not touch.
2. **Default-OFF.** Even on the real-ELF path, `analysis_listing` defaults `false`, so `arch.analysis_listing.then(|| Listing::build(...))` is `None` — no decode work, `ctx.listing = None`. The real-ELF bootstrap is byte-identical to today by default. The 158/158 stage-model oracle is untouched.
3. **In-memory intermediate, not a fact.** The Listing is a borrow in `AnalysisCtx`, never an `AnalysisOutput` field, never committed into the engine IR. Only a *consumer's* derived `AnalysisOutput` (e.g. `NoReturnFact`) reaches the decompiler, through commit arms that already exist and are individually gated + additive (`engine.rs:781-803`). The `AnalysisOutput::merge`-is-concatenation invariant (`pass.rs:277`) is untouched.

Because of layer 1, `make test` and `make test-stages` are guaranteed untouched by every PR; PRs only need to keep `make rust-test` green (new unit tests on the new module) and `kuna catalog --check` clean.

### 5.3 Build-timing resolution (where the drafts disagreed)

DRAFT A noted a tension: the per-pass *commit* gate (`analysis_pass_enabled`) runs late (at `read symbols`, after `--option` flags are applied), but the Listing *build* happens at load (inside the driver). **Resolution: build-at-load gated on the flag is correct.** The CLI emits `--option listing on` *before* load for build-time gates (the same as `addrtable`), so `arch.analysis_listing` is already set when `bootstrap_from_elf` runs. Default-off ⇒ default-not-built ⇒ the zero-cost path. We do **not** build unconditionally-then-gate-consumers (DRAFT B's fallback), because default-not-built is strictly cheaper and is the parity-safe path. If a future need arises to flip the flag *after* load, revisit then; it is not needed now.

---

## 6. Consumer API & build order

The union of the three consumers' needs is exactly the §2 model. Read-only API on `Listing`:

```rust
impl Listing {
    // instruction model
    pub fn instruction_at(&self, vma: u64) -> Option<&Insn>;
    pub fn instruction_containing(&self, vma: u64) -> Option<&Insn>;
    pub fn is_instruction_start(&self, vma: u64) -> bool;
    pub fn num_instructions(&self) -> usize;                  // AIF gate
    // codeunit partition
    pub fn code_unit_at(&self, vma: u64) -> CodeUnit;
    pub fn is_data(&self, vma: u64) -> bool;                  // A's "fell into data"
    pub fn is_undefined(&self, vma: u64) -> bool;
    pub fn first_undefined_after(&self, vma: u64) -> Option<u64>;  // AIF gaps (PR8)
    // xref model (read-only is enough for the buildable consumers)
    pub fn refs_to(&self, to: u64) -> &[Reference];          // callers
    pub fn refs_from(&self, from: u64) -> &[Reference];
    pub fn ref_source_iter(&self) -> impl Iterator<Item=u64>; // call-site worklist
    pub fn has_refs_to(&self, to: u64) -> bool;
    pub fn ref_count_to(&self, to: u64) -> usize;
    // function model (ordered)
    pub fn function_at(&self, vma: u64) -> Option<&DiscoveredFunction>;
    pub fn function_containing(&self, vma: u64) -> Option<&DiscoveredFunction>;
    pub fn next_function_after(&self, vma: u64) -> Option<&DiscoveredFunction>;
    pub fn function_count(&self) -> usize;
}
```

**Build Consumer A FIRST — `FindNoReturnFunctionsAnalyzer` (discovered-no-return).** Cheapest, highest-confidence. It maps onto the API with no escape hatches:
- callers of a callee → `refs_to(callee).filter(|r| r.kind == Call)`
- call-site worklist → `ref_source_iter()`
- "fall-through landed in data" (highest-value signal) → `!is_instruction_start(fall_vma)`
- "function defined immediately after this call" → `next_function_after(call_addr)`
- per-call predicates → `insn.flow.{is_call, has_fallthrough, is_terminal, is_indirect}`, `insn.flows`
- skip already-modeled callees → `function_at(target).has_no_return / .call_fixup`
- `skipNOPS` real-NOP test → `insn.pcode` (lazy-materialized)

**Its output and commit path already exist and are proven**: it emits `NoReturnFact { addr, name }` (`pass.rs:108`), whose commit arm resolves via `find_function_across_scopes`/`query_global_function` then `set_function_no_return` (`engine.rs:781-803`). Flow-repair is *inherited* — kuna's engine already does post-no-return dead-code elimination, so Consumer A needs no `ClearFlowAndRepairCmd`/`CreateFunctionCmd`/bookmarks. Net-new logic = the evidence-tally fixpoint (callee no-return if ≥3 callers show no-valid-fall-through; 2-pass to fixpoint) — a few dozen read-only lines.

**Consumers B and C are declined** (§8) — the keystone makes them buildable but does not pull them in.

---

## 7. Ordered PR breakdown

Each PR: small, one fixture testcase, all three gates green, `kuna catalog --check` clean. No PR depends on an unbuilt consumer. Fixtures are tiny checked-in ELFs under `decompiler/crates/kuna-analysis/tests/fixtures/` (reuse existing fauxware/test ELFs the `s1_entry` tests already use at `s1_entry/mod.rs:1414+` where possible).

### PR0 — Recursive-descent core + instruction model + classifier (the keystone heart)
**Scope.** `listing/{model.rs, decode.rs, classify.rs, walk.rs}` with: the `Insn`/`FlowType`/`FlowKind`/`Reference`/`DiscoveredFunction`/`CodeUnit` types; `decode_one` over `Translate::one_instruction` + `print_assembly`; the `classify` switch (lifted from `flow.rs:1039-1185`); the two-level worklist walk seeded from a caller-supplied seed list. `Listing::build(file, image, arch, translate, seeds)` wired but **not yet invoked from the engine** (unit-tested in isolation with a test `Translate`/fixture). No flag yet, no `AnalysisCtx` change. `s1_entry` seed helpers promoted to `pub(crate)`.
**Files.** New `listing/` module + `lib.rs` `pub mod listing;`; `s1_entry/mod.rs` visibility bumps.
**Testcase.** Unit test in `listing/` over a vendored tiny x86-64 ELF: decode from `main`'s entry, assert (a) `instruction_at(entry)` has the right `len` + `mnemonic`; (b) a known CBRANCH yields two successors (target + fall_through) and `flow.kind == ConditionalBranch`; (c) a RETURN is terminal (`fall_through == None`); (d) a CALL records `RefKind::Call` and seeds a new function entry; (e) **the classifier agrees with `flow.rs` on the same bytes** (decode the same insn through a minimal FlowInfo-style harness or a recorded golden). 
**Risk.** Medium — this is the load-bearing logic. Mitigated by copying the proven switch verbatim and the explicit `flow.rs`-agreement test, plus the constant-space and last-op rules tested directly.
**Deps.** None.

### PR1 — The `--option listing` flag, default-OFF, wired end-to-end (no build yet)
**Scope.** All of §5.1: `analysis_listing` field/default/reset/`set_kuna_option`; `KUNA_OPTION_NAMES`; `analysis_pass_enabled`; `stages.toml` row + header count fix; count-test bump; `docs/assertions.md` regen. Add `pub listing: Option<&'a Listing>` to `AnalysisCtx` (always `None` — build deferred to PR2).
**Files.** `architecture.rs`, `options.rs`, `engine.rs`, `stages.toml`, `tests.rs`, `pass.rs`, `docs/assertions.md`.
**Testcase.** `kuna catalog --check` passes; a `tests/stages/` case flipping `--option listing on` asserting it parses/round-trips and changes nothing (default-off parity); `settable_count_is_35` green.
**Risk.** Low — pure plumbing, default-off.
**Deps.** PR0 (the `Listing` type must exist for the `AnalysisCtx` field).

### PR2 — Invoke the keystone from the engine (flag-gated build)
**Scope.** Thread `translate` into `run_default_analyses_per_pass` (and the symmetric `run_default_analyses` signature); build the seed union in the driver; `let listing = arch.analysis_listing.then(|| Listing::build(...))`; populate `ctx.listing`. Engine call site updated (`engine.rs:634-635`).
**Files.** `passes.rs`, `engine.rs`.
**Testcase.** A `tests/stages/` (or rust-test) case: with `--option listing on`, decompile a fixture ELF and assert it still produces identical decompiler output (the Listing is built but no consumer reads it yet ⇒ no behavior change); with flag off, byte-identical to today. A rust-test asserts `Listing::build` over a fixture populates `function_count()` and `instruction_at(entry)`.
**Risk.** Low — the build is proven by PR0's unit tests; this is the wiring + the default-off/flag-on no-op proof.
**Deps.** PR0, PR1.

### PR3 — CodeUnit partition + FunctionModel ordered queries
**Scope.** `partition_code_units()` at end of build (exec ranges minus covered → Undefined); `code_unit_at`/`is_data`/`is_undefined`/`first_undefined_after`/`num_instructions`; ordered `function_at`/`function_containing`/`next_function_after`/`function_count`.
**Files.** `listing/model.rs`, `listing/mod.rs`.
**Testcase.** Over the fixture ELF: assert the partition (a known instruction VMA → `Instruction`, a known gap VMA → `Undefined`); assert `next_function_after(main)` returns the next-by-address function; `function_containing(mid_of_main)` returns `main`.
**Risk.** Low — pure derived queries over the already-built maps.
**Deps.** PR0.

### PR4 — Reference model read API hardening
**Scope.** Bidirectional `refs_to`/`refs_from`/`ref_source_iter`/`has_refs_to`/`ref_count_to` finalized and tested (the walk already files edges in PR0; this PR pins the public API + ordering/dedup semantics).
**Files.** `listing/mod.rs`, `listing/model.rs`.
**Testcase.** Fixture with `main` calling a helper: assert `refs_to(helper)` contains the call site with `RefKind::Call`; `ref_source_iter()` includes `main`'s call address; `ref_count_to(helper)` matches call count.
**Risk.** Low.
**Deps.** PR0.

### PR5 — Context-paint correctness (ARM Thumb / MIPS ISA_MODE)
**Scope.** `context.rs`: resolve `ContextPaint`/`tracked_regs` (TMode/ISA_MODE) per seed/address and paint into the context DB before decode. x86-64 unaffected.
**Files.** `listing/context.rs`, `listing/walk.rs` (call the painter before `decode_one`).
**Testcase.** An ARM fixture with a Thumb function (TMode=1): assert the seed decodes to the expected Thumb mnemonic, not the A32 misdecode. Reuse the `verify_arm_thumb_decode.rs` pattern.
**Risk.** Medium — isolated to ARM/MIPS, gated off by default; cannot perturb the parity gates (real-ELF-path-only + default-off).
**Deps.** PR2 (needs the engine-wired build to have the arch context available).

### PR6 — Consumer A: `FindNoReturnFunctionsAnalyzer` (the payoff)
**Scope.** New gated pass `id = "noreturn_disc"`, default-OFF, its own flag (full §5.1 wiring for `noreturn_disc`, including `analysis_pass_enabled` arm), `run` short-circuits to empty when `ctx.listing.is_none()`. Read-only over the Listing: the evidence-tally fixpoint (callee no-return if ≥3 callers show no-valid-fall-through; 2-pass). Emits `NoReturnFact { addr, name }`.
**Files.** New `s1_noreturn_disc/mod.rs`; register in `lib.rs` + `passes_for` (`passes.rs:38`); `architecture.rs`/`options.rs`/`engine.rs`/`stages.toml`/`tests.rs` for the `noreturn_disc` flag (settable_count → 36); `docs/assertions.md`.
**Testcase.** `tests/stages/` case: a fixture with a static no-return wrapper (e.g. an `abort()`-style helper) called ≥3× whose call sites have no valid fall-through. With `--option listing on --option noreturn_disc on`, assert the wrapper is marked no-return (and, via the inherited engine repair, the caller's post-call dead code is eliminated). With either flag off, assert byte-identical-to-today (parity).
**Risk.** Low given PR0-PR5 — a few-dozen-line read-only fixpoint with an existing, proven output fact + commit seam (`engine.rs:781-803`). This validates the whole keystone end-to-end.
**Deps.** PR0-PR4 (PR5 only if the no-return fixture is ARM/MIPS; an x86-64 fixture needs only PR0-PR4).

### PR7 (optional, only if AIF/FID is ever taken) — gap-query groundwork
**Scope.** `covered`/`exec_ranges` already present from PR0/PR3; this only adds the gap-iteration helper (`exec_ranges − covered`) and tests. **No consumer.** Do not land unless AIF is scheduled.
**Testcase.** Assert a known uncovered exec sub-range is reported as a gap.
**Risk.** Low.
**Deps.** PR3.

---

## 8. What to DEFER (and why)

- **Jump-table / indirect-branch target resolution.** Intrinsically post-dataflow (action pipeline + `EmulateFunction` over the SSA tree, `s2_lift/jumptable.rs`). No cheap pre-decompile form. Treat BRANCHIND/CALLIND as unresolved terminals with `is_computed`/`is_indirect` set — the sound, engine-consistent behavior (`flow.rs:1523-1525`). A future consumer can branch on the predicates without the keystone lying about resolution.
- **Per-op pcode storage by default.** `Insn.pcode` stays `None` unless a consumer materializes it (only `skipNOPS`/`isUsedForCalculation` need it).
- **Consumer B (operand/reference markup family).** Its primary output is *references written into a ReferenceManager*, and kuna's decompiler consumes **no `RefFact`** (no commit arm; the engine reads bytes + symbol/type tables). It is ELF-default-OFF upstream (`ScalarOperandAnalyzer.canAnalyze = !ElfLoader.isElf`), `ElfScalarOperandAnalyzer` only *deletes* bad `.got`/`.plt` refs kuna never wrongly creates, and its one decompiler-relevant idea (scalar → `.rodata` string ⇒ `char*`) is already delivered by `s1_strings` + `s1_protos` + S5 and is printer-shadowed by MapGlobals. The keystone exposes **no reference-writing API** — faithfully, because there is nowhere for those writes to land. Decline as producing passes.
- **Consumer C (AggressiveInstructionFinder).** Needs the keystone **plus two net-new capabilities it deliberately omits**: (1) a SLEIGH instruction-**mask** accessor in `kuna-sleigh` (buildable from the existing `PatternBlock::get_mask`/`DisjointPattern::get_mask` at `slghpattern.rs:385/848`, but only with its consumer); (2) the full `PseudoDisassembler.checkValidSubroutine`/`followSubFlows` decode-with-validation (a superset of plain decode — a new `walk.rs` mode). Its *unique* contribution (guessing code in gaps with no symbol/FDE/xref) is exactly the high-false-positive case Ghidra ships `setDefaultEnablement(false)`; for a decompiler *given* entries, the sound output is already covered by `s1_entry` + `.eh_frame` FDE scanning. Decline AIF itself; the mask accessor it needs is the same one FID would need, so build it independently only if FID is taken.
- **A committed engine `Listing` fact.** Keep the Listing in-memory. Inventing a `RefFact` nobody consumes would break the additive-merge invariant for no payoff.

---

## 9. Risks & mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| **Parity-gate perturbation** (a Listing change leaks into 675/158) | High if it happened | **Structurally impossible** by layer 1 (§5.2): the build is real-ELF-path-only, never reached from the XML datatest path. Layer 2 (default-OFF) + layer 3 (in-memory, not a fact) are belt-and-suspenders. Every PR re-runs all three gates; layers 1-2 guarantee `make test`/`make test-stages` untouched, so PRs only need `make rust-test` + `kuna catalog --check` green. |
| **Classifier divergence from `flow.rs`** (the two flow models drift) | Medium | `classify` is a *lifted copy* of the single source of truth (`flow.rs:1039-1185`), not a reinvention. PR0's golden test pins `classify` against `flow.rs` on the same bytes; if upstream flow logic changes, the test catches the drift. |
| **Decode-correctness traps** | Medium | Three explicitly handled: (a) **constant-space in0 is p-code-relative** — tested first (§4.3 gotcha 1); (b) **fall-through is the *last* op** (gotcha 2); (c) **delay slots already in `len`** — don't double-decode (gotcha 3). Plus: variable-length ISAs always advance by the returned `len`; alignment errors and `Err(Unimpl/BadData)` are caught and stop that path (not assumed-success). |
| **Context misdecode** (ARM A32-vs-Thumb, MIPS16) | Medium | PR5 paints `ContextPaint`/`tracked_regs` before decode, isolated to ARM/MIPS and default-OFF. PR0-PR4 fixtures are x86-64 (no mode context), so the core lands without this risk. |
| **Whole-program disassembly cost** | Medium | The walk is O(covered bytes) with `BTreeMap`-membership dedup (each VMA decoded at most once via the `insns`/`visited_funcs` sets). It runs **only behind `--option listing on`** (default-OFF ⇒ zero cost), and only on the real-ELF path (never the 675-datatest hot loop). If a large binary is slow, the cost is opt-in and the walk is bounded; no decompilation IR is built (no `Funcdata`/banks/blocks), so it is far cheaper than a real decompile. |
| **False positives (AIF's distinctive danger)** | High *for AIF* | AIF is **declined** (§8): its unique gap-guessing is the exact case Ghidra ships default-OFF for false-positive reasons. The keystone seeds only from *sound* roots (`existing_function_addrs ∪ collect_entries` — real funcsyms + the five entry oracles), and discovered functions come only from *concrete* CALL targets (`in0` of a CPUI_CALL), never speculative gap decoding. Consumer A's no-return fixpoint requires ≥3 corroborating callers, matching Ghidra's `evidenceThresholdFunctions`. |
| **Indirect-target unsoundness** (pretending to resolve jump tables) | Low | Deferred (§8): BRANCHIND/CALLIND are unresolved terminals with `is_computed`/`is_indirect` set — the keystone never claims a target it didn't statically derive. |

---

**First value delivered:** PR6 (`FindNoReturnFunctionsAnalyzer`) — the cheapest consumer, validating the keystone end-to-end through an output fact (`NoReturnFact`) and commit seam (`engine.rs:781-803`) that already exist and are proven, with flow-repair inherited from the engine.

**Key file references (absolute):** new `/home/mahaloz/github/kuna/decompiler/crates/kuna-analysis/src/listing/` ; edits to `/home/mahaloz/github/kuna/decompiler/crates/kuna-analysis/src/{pass.rs:292, passes.rs:188/197, s1_entry/mod.rs:132/224/249/257}` , `/home/mahaloz/github/kuna/decompiler/crates/kuna-decomp/src/infra/architecture.rs:{381,575,668,771}` , `/home/mahaloz/github/kuna/decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs (KUNA_OPTION_NAMES)` , `/home/mahaloz/github/kuna/decompiler/crates/kuna-console/src/engine.rs:{271,634,781}` , `/home/mahaloz/github/kuna/decompiler/crates/kuna-decomp/stages.toml:{8,1807,1823}` , `/home/mahaloz/github/kuna/decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_stages/tests.rs:{4,31,37,38}` ; reused unchanged: `Sleigh::one_instruction`/`instruction_length` (`/home/mahaloz/github/kuna/decompiler/crates/kuna-sleigh/src/translate.rs:{459,472,481}`, `PcodeEmit::dump` `:166`), the classifier source-of-truth `/home/mahaloz/github/kuna/decompiler/crates/kuna-decomp/src/s2_lift/flow.rs:{1039-1188}` , `ObjectLoadImage::load_fill` `/home/mahaloz/github/kuna/decompiler/crates/kuna-analysis/src/loadimage_object.rs:354` .