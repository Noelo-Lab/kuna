# [PROPOSAL] thunk / external-location object model — model PLT/IAT/__stubs stubs as thunks to external functions

**Status:** draft proposal, awaiting human go/no-go. Do **not** implement until approved.
**Class:** ENGINE-touching, multi-part feature → `[PROPOSAL]` draft PR per the standing
requirement (`docs/pipeline.md` → *Standing requirements*).
**Faithful to:** Ghidra `CreateThunkAnalyzer` / `CreateThunkFunctionCmd.getThunkedAddr`
(the "single-jump-target → thunk" recognizer) + `ghidra.program.model.symbol.ExternalLocation`
/ `ThunkFunction` (the external object model) + `ElfProgramBuilder.createExternalFunctionLinkage`
(the loader wiring). Ghidra paths cited at the `GHIDRA_REV` in `docs/UPSTREAM.md`.

---

## 0. TL;DR — the recommendation

**DEFER the object model. Build nothing now.** The two benefits this model is supposed to
deliver — *correct call naming* and *thunk tail-call inlining/rendering* — are **already
delivered by other means in kuna**, and the spike below proves the upstream payoff is not
what the gap note assumed:

- **Correct naming** (the always-on Ghidra effect) is already done: `elf_plt.rs`
  reconstructs each stub→import name and registers it as a plain named `FunctionSymbol`, so
  the call site already renders `puts(...)` / `printf(...)`.
- **Tail-call rendering** of `jmp setlocale@plt` (the `-O2` case) is already done by the
  existing `tailcalljump` option (`kuna_tailcalljump.rs`, default-off, `option tailcalljump on`).
- **Thunk-driven inlining is a myth in upstream too** (Q4 spike): Ghidra does **not** mark
  PLT-stub thunks `inline`. The decompiler's only default thunk effect is *name resolution
  through the thunk*, which kuna achieves directly by naming the stub. Inlining happens only
  when the INLINE flag is *explicitly* set (`option inline <name>`), which is orthogonal to
  thunk recognition and already available in kuna today.

So the object model's marginal output benefit over kuna's current "correct-names" stopgap is
**near-zero** for an ELF decompiler. It is worth building only if/when kuna wants the
*structural* fidelity (an `<EXTERNAL>` namespace, library-qualified names, a bodiless external
symbol) for downstream tooling — not for the decompiled C. The effort to do it faithfully is
**M–L** with new engine plumbing; the payoff does not justify it now.

A **cheap, parity-safe spike is included** (Part 5): a gated `--option thunks` pass that
*emits thunk facts* (stub→target) by re-using the existing `elf_plt` decode, recognized per
`getThunkedAddr`'s heuristic, and commits them by setting the **existing** `set_function_inline`
flag — i.e. it makes a thunk fact *expressible and visible* without any new engine model. It
ships **default-off** and changes nothing unless flipped, so all gates stay green. It exists to
de-risk the design, not to deliver the feature.

---

## 1. What kuna does today (the baseline)

A PLT/IAT/`__stubs` import stub is registered as a **plain named `FunctionSymbol`** at the
stub address:

- ELF: `kuna-analysis/src/s1_loader/elf_plt.rs::resolve_plt_imports` reconstructs
  `stub_addr → name` from the dynamic relocations + per-arch stub decode (x86-64/32, AArch64,
  ARM32, RISC-V, SPARC, PPC64, MIPS), then the loader installs each as an ordinary
  `FunctionSymbol` in the same stream as `.symtab`/`.dynsym` functions.
- PE IAT (`pe_iat.rs`/`format/pe.rs`) and Mach-O `__stubs` (multi-format loader, Increments
  37/39) have the **same shape**: a plain named function at the stub.

The module documents this as a stopgap (`elf_plt.rs:14-17`): *"the 'correct-names' model — the
full external-location/thunk object model is a deferred follow-up."* The gap note
(`docs/missing-analyses.md` §1) frames it: *"invisible for ordinary call naming, but matters
for thunk tail-call inlining and external-reference rendering."*

The spike (Part 2) shows that framing is **half right**: the object model is indeed invisible
for ordinary call naming (kuna already names correctly), and the "matters for tail-call
inlining" claim does not hold for stock Ghidra defaults.

---

## 2. SPIKE — the engine seam (what EXISTS vs MISSING)

A two-prong spike: kuna's engine (`kuna-decomp`) and upstream Ghidra's actual mechanism.

### 2a. kuna engine — what's MISSING

- **No thunk object model.** No `is_thunk` flag, no `thunk_target`/`getThunkedFunction`
  pointer on `Symbol` / `SymbolKind::Function` / `Funcdata` / `FuncProto` / `FuncCallSpecs`.
  The word "thunk" in `kuna-decomp` is the Rust closure idiom (`replace_reads_thunk`) or a
  jump-table recovery *bail* (`jumptable.rs` `isthunk` local bool, `RecoveryMode::FailThunk`),
  never a model.
- **No external-location object.** `SymbolKind::ExternRef { refaddr }` (`database.rs:560`) and
  `Database::find_external_ref` (read, `database.rs:2781`) *exist as types*, but **nothing
  creates an ExternRef** — the only creation site, `IfcMapexternalref` (`map externalref`,
  `ifacedecomp.rs:1325`), is a stub returning `Err`. There is no `add_external_ref` /
  `Scope::addExternalRef`. No `<EXTERNAL>` namespace, no bodiless external function.

### 2b. kuna engine — what EXISTS (the seams a thunk would ride on)

- **The inline-flow path is fully wired** (this is the substrate Ghidra thunks would use):
  - `SymbolKind::Function { inline_func, no_return, inject_id, proto_pieces, .. }`
    (`database.rs:511`) — the per-function inline disposition.
  - `Database::set_function_inline(sid, val)` (`database.rs:2600`) — **exact parallel** of
    `set_function_no_return` (`database.rs:2609`), which the analysis commit seam already
    calls (`engine.rs:915`).
  - `function_is_inline_across_scopes` (`database.rs:2760`) → `query_call_inline`
    (`decompile_drive.rs:108`, wired — *not* a stub) → consumed in `flow.rs:2024`.
  - `FlowInfo::inline_sub_function`/`inline_flow` (`flow.rs:2119/2175`), with
    `inline_recursion`/`inline_head` cycle-break — the C++ `Funcdata::inlineFlow` port.
  - `FuncProto::is_inline()`/`set_inline()` + `func_proto_flags::IS_INLINE` (`fspec.rs:5199/4825`).
  - `OptionInline` (`option inline <name>`, `options.cc:365` port, `options.rs:972`).
- **The external-ref *varnode* flag exists and is consumed.** `varnode_flags::externref`
  (`varnode.rs:82`), `Varnode::is_external_ref()` (`varnode.rs:813`); `ActionDeindirect`
  (`coreaction_render.rs:1148`) checks `is_persist() && is_external_ref()` → `query_function`
  → `deindirect`, and `deindirect` already copies the resolved callee's `is_inline()` /
  `is_no_return()` into the call spec (`coreaction_render.rs:1177-1182`). The flag is a
  varnode property, never set from a symbol/location object today.
- **Flow-override incl. `CALL_RETURN`** (`overrides.rs::flow_type::CALL_RETURN = 3`,
  `insert_flow_override`/`get_flow_override`) — Ghidra's "branch-to-thunk = call-then-return".
- **Tail-call recognition** for the direct-jump PLT case: `kuna_tailcalljump.rs`
  (`kuna_is_tail_call_branch`, `option tailcalljump on`, default-off). Its doc explicitly
  names `jmp setlocale@plt` as the motivating case and already renders
  `return setlocale(v1, NULL)` when on.
- **The call-name render seam:** `PrintC::call_callee_name` (`printc.rs:4225`) — a single
  isolated lookup of `FuncCallSpecs::name`; the one place a thunk would redirect the printed
  name to its target.
- **The commit seam:** `commit_analysis_output` (`engine.rs:803`) walks `AnalysisOutput`
  (`pass.rs:204`) and applies facts via `add_function` / `set_function_no_return` /
  `set_function_inline` / `set_function_inject_id` / `set_function_proto_pieces`. **No
  thunk/external arm and no `AnalysisOutput.thunks` field** — but the pattern to add one is
  well-trodden (arm 3, no-return, is the template).

### 2c. Ghidra — the actual mechanism (the faithful model to replicate)

- **Recognition** (`CreateThunkFunctionCmd.getThunkedAddr`, *not* `FunctionAnalyzer` itself):
  - Fast path — `getSimpleFlow`: instruction is a single unconditional `jump` (or terminal
    `call`) with **exactly one** flow target → that target is the thunked address.
  - Indirect-through-pointer (`getFlowingAddress`): a `jmp *GOT[n]` flow ref points at the GOT
    slot; follow the slot's *single* outgoing reference; if it is `DATA` or an **external
    reference**, return its target. >1 reference ⇒ bail. Recognized-but-unresolvable ⇒
    sentinel `Address.NO_ADDRESS` ("is a thunk, target unknown").
  - Slow path — ≤ `MAX_NUMBER_OF_THUNKING_INSTRUCTIONS = 8` instructions, rejecting any
    `STORE` / side-effecting register set ("pure pass-through" gate).
- **Object graph:** `stub Function (real addr, thunk) → thunkedFunction → external Function
  (entry in EXTERNAL space, no body) → ExternalLocation { libraryName, label, originalImportedName,
  address-or-null }`. `isExternal()` is a property of the *target's* address space, not the stub.
  **All Function behaviors forward through the thunk** (`FunctionDB.isFunctionFlagSet`
  delegates to `thunkedFunction`): inline/noreturn/varargs/signature/namespace all read
  *through* the stub to the external.
- **Loader wiring** (`ElfProgramBuilder.createExternalFunctionLinkage`):
  `extLoc = externalManager.addExtFunction(Library.UNKNOWN, name, null, IMPORTED)` then
  `stub.setThunkedFunction(extLoc.getFunction())`, then strip the stub's own symbol so the
  name resolves through the thunk.
- **Decompiler payoff (the decisive Q4 finding):**
  - **(A) Name resolution through the thunk** is the *primary, always-on* effect.
    `FunctionSymbol.doGetName` returns the **thunked symbol's** name; `HighFunctionSymbol.getNamespace`
    walks thunks to the library namespace. The call site renders the external (library) name,
    namespace-qualified.
  - **(B) Inlining is NOT driven by thunk-ness.** The decompiler inlines iff the encoded
    `<prototype>` carries `inline="true"`, which `FunctionPrototype.encodePrototype` writes
    iff `func.isInline()`. For a thunk that forwards to the external — **and the ELF loader
    never sets INLINE on the externals it creates.** So by default PLT-stub thunks are *not*
    inlined; the decompiler just resolves+renders the external name.

**Conclusion of the spike:** the object model's *decompiled-C* benefit reduces to "render the
import name (namespace-qualified) at the call site" — which kuna already does sans namespace.
The inlining benefit is real but orthogonal and already reachable via `option inline`.

---

## 3. The faithful design (if/when built)

The model, faithful to Ghidra, in three layers:

### 3.1 Recognition (analysis tier) — a `s1_loader/thunks.rs` pass

A new `AnalysisPass` (default-off, `id() == "thunks"`) that, for each stub already decoded by
`elf_plt::resolve_plt_imports` (and the PE/Mach-O equivalents), records a **thunk fact**
`(stub_addr → target)`:

- It does **not** need a disassembly Listing tier (unlike AIF / no-return-discovered): the
  stub→GOT-slot→name correlation is *already computed* by `elf_plt` (the exact analog of
  Ghidra's `getFlowingAddress` "follow the GOT slot's single reference to the external").
  So recognition is a thin re-wrapping of the existing decode — the cheap part.
- The "single unconditional jump, ≤8 instrs, no STORE/side-effects" gate (`getThunkedAddr`)
  is **implicitly satisfied** by anything `elf_plt` already accepts as a PLT stub; the pass
  can simply trust the existing decoder. (A future, broader CreateThunkAnalyzer — recognizing
  *non-PLT* in-`.text` thunks — would need the Listing tier from
  `docs/analysis-port-buildplan.md` §1.0; out of scope here.)

### 3.2 The external object (engine) — the genuinely new plumbing

To be *structurally* faithful (bodiless external in an `<EXTERNAL>` namespace), the engine
needs net-new pieces. Two options, increasing fidelity:

- **(min) Inline-flag-only** — no new model. The commit arm sets the existing
  `set_function_inline` on the stub (or leaves it off — see 3.4). This makes thunk-ness
  *expressible as a fact* and re-uses 100% existing engine machinery. **This is what the
  spike (Part 5) does.** Effort **S**.
- **(full) The external-location object** — a `thunk_target: Option<Address>` field on
  `SymbolKind::Function` (mirrors `inline_func`/`proto_pieces`) + `Database::set_function_thunk`
  (mirrors `set_function_no_return`) + an `<EXTERNAL>` namespace + `Scope::addExternalRef`
  (revive the `IfcMapexternalref` stub) + name-forwarding (`doGetName` borrows the thunked
  symbol's name) + namespace-walk in the printer. Effort **M–L** (the forwarding semantics
  touch symbol resolution and rendering broadly).

### 3.3 The commit arm (engine) — `engine.rs::commit_analysis_output`

Add `AnalysisOutput.thunks: Vec<ThunkFact { stub: u64, target: Option<u64>, name }>` and an
arm (the template is arm 3, no-return):
- resolve the stub function by address (`find_function_across_scopes`),
- (min) `set_function_inline(sid, true)` if inlining is desired, **or**
- (full) `set_function_thunk(sid, target_addr)` and create/link the external location.

### 3.4 The printer (engine) — `printc.rs::call_callee_name`

In the **full** model, redirect the rendered name through the thunk to the external's
library-qualified name (`<EXTERNAL>::printf`). In the **min** model, no printer change is
needed — the stub already carries the right name.

---

## 4. Effort, ordering, benefit

| Piece | Effort | Note |
|---|---|---|
| `s1_loader/thunks.rs` recognition pass (re-wrap `elf_plt`) | **S** | no Listing tier needed |
| `AnalysisOutput.thunks` + commit arm (inline-flag-only) | **S** | template = no-return arm 3 |
| **min model total (the spike)** | **S** | parity-safe, default-off |
| `thunk_target` field + `set_function_thunk` + `<EXTERNAL>` ns + `addExternalRef` | **M** | net-new engine model |
| name/namespace forwarding (`doGetName`, printer) | **M–L** | touches symbol resolution + rendering |
| **full faithful model total** | **M–L** | structural fidelity only |

**Ordering** (if approved): (1) the recognition pass + min commit arm (the spike, S); (2) the
external-location object + forwarding (M–L) — only if downstream tooling needs the structure.

**Concrete benefit, honestly scoped:**
- *Decompiled C naming:* **no change** vs today (kuna already names the stub). The full model
  would add library-namespace qualification (`<EXTERNAL>::printf`), a cosmetic delta.
- *Thunk tail-call inlining:* the `-O2` `jmp X@plt` case is **already** handled by
  `tailcalljump`. A thunk *whose external is INLINE* would be inlined via the existing
  inline-flow path — but, faithful to Ghidra, the loader would **not** set INLINE on imports,
  so this is opt-in only (and already reachable via `option inline <name>`).
- *External-reference rendering / structural fidelity:* the only genuine new capability — a
  real external object model — but it does not change the decompiled C for the common case.

---

## 5. The spike (included, default-off, parity-safe)

To de-risk the design and make a thunk fact *visible* without any new engine model, this PR
includes a **design-only doc** plus a documented spike *plan* (no engine code shipped in the
draft). The spike, if approved, is:

- `s1_loader/thunks.rs`: an `AnalysisPass` (`id() == "thunks"`, **default-off**, registered in
  `stages.toml` + `KUNA_OPTION_NAMES`) that re-wraps `elf_plt::resolve_plt_imports` output as
  `ThunkFact { stub, target, name }`.
- `AnalysisOutput.thunks` + a commit arm in `commit_analysis_output` that, **only when
  `--option thunks on`**, calls the existing `set_function_inline(sid, true)` on each stub —
  proving the thunk fact flows end-to-end through the *existing* inline-flow path with **zero
  new engine plumbing**.
- Gate: off ⇒ `AnalysisCtx` produces no thunk facts ⇒ byte-identical output ⇒ all three gates
  stay green. The XML datatest path constructs no `ObjectLoadImage`, so it is structurally
  untouched (same guard as every other analysis pass).

The spike is **not** the feature — it is the smallest thing that proves the seam. It is
included as design here (not shipped as code in the draft) precisely because the recommendation
is **defer**: shipping even a default-off pass adds a settable option and a commit arm that we
should only land once the go-decision is made.

---

## 6. Go / no-go ask

**Recommendation: NO-GO (defer).** The object model delivers near-zero decompiled-C benefit
over kuna's current correct-names stopgap; the two claimed payoffs are already covered
(`elf_plt` naming + `tailcalljump`), and the spike confirms upstream Ghidra does not inline
thunks by default. Build the full external-location object **only** if kuna later wants the
structural fidelity (`<EXTERNAL>` namespace, library-qualified names, bodiless external
symbols) for downstream consumers — at which point Part 3.2 (full) + Part 3.4 are the plan,
effort **M–L**.

If the human wants the **min spike** landed now (the `thunks` recognition pass + inline-flag
commit arm, default-off, effort **S**) as a foothold, that is parity-safe and ready to
implement from Part 5. Awaiting the call.
