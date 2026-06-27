# PROPOSAL: kuna PE/Mach-O Deep-Metadata Analyzers

**Status:** for go/no-go. **Scope:** add gated, default-off, real-binary-path analysis passes that recover runtime/debug metadata from Windows (PE) and macOS (Mach-O) targets, feeding the decompiler richer names, types, and vtable/dispatch resolution — the next analysis-porting frontier after the ELF/DWARF/FID tier.

---

## 1. Headline decision — the prioritized subset + order

Build **three** now, in this order; **defer the fourth** to a stub PROPOSAL.

| # | Analyzer | Self-contained? | Effort | ROI | Verdict |
|---|---|---|---|---|---|
| **1** | **MSVC RTTI / vftables** (`s1_rtti`) | **Yes** — in `.rdata` | **S–M** | **Highest** | **Build first** |
| **2** | **Mach-O Objective-C** (`s1_objc`) | **Yes** — in `__objc_*` | **M** | High | **Build second** |
| **3** | **PDB** (`s1_pdb`) | **No** — needs `.pdb` | **L** | Highest *value* | **Build third** |
| 4 | Mach-O Swift (`s1_swift`) | Yes (metadata) / No (demangler) | XL | Lowest | **Defer — PROPOSAL stub only** |

### The axis that drives the order: self-contained vs external-file

This is the key axis the orchestrator asked to flag. Three of the four parse **entirely out of the loaded image** (RTTI from `.rdata`, ObjC from `__objc_*`, Swift from `__swift5_*`) and therefore follow the **`s1_dwarf` precedent** exactly: read `ctx.file`/`ctx.bytes`, emit facts, commit. **PDB is the lone external-file case** — the PE carries only a fingerprint, the debug info lives in a separate `.pdb` — so it follows the **`s1_fid` precedent** (apply external knowledge only when a supplied artifact matches a fingerprint gate). That single distinction governs PDB's whole design (§4.3) and is why it sits third despite being the highest-value payoff.

### Why this order (value × effort × feasibility)

- **RTTI first** — *most feasible, highest ROI.* The MSVC demangler is **already wired** (`s1_demangle`'s `?`-arm, `demangle_name`); the structures are ~4 fixed-size models with one clean x86/x64 branch; **zero new crate deps**. It delivers C++ class names + vtable→virtual-method binding on Windows PEs. Richest seam reuse of all four.
- **ObjC second** — *clearly worth it, no demangler needed.* Selectors are plain ASCII (`-[NSString length]`), pointers mostly absolute, big macOS/iOS naming win. **Zero new crate deps.** The one real cost is the **Mach-O chained-fixup prerequisite** (PR O0) — kuna's loader does not resolve `LC_DYLD_CHAINED_FIXUPS` today, and without it `__objc_classlist` reads garbage on modern arm64e binaries.
- **PDB third** — *highest value, highest effort, only external-file case.* It is **Windows' DWARF** (names + full types + params/typed locals + lines) and lands through the **exact** `s1_dwarf`/`TypeFactory` commit path already proven. Cost is one new crate (`pdb`), a PE CodeView-record extractor, and a `TypeData→Datatype` mapper. The external-`.pdb` problem is cleanly the FID-database analog.
- **Swift deferred** — *lowest ROI, highest complexity.* It needs a **brand-new demangler dependency** (no embeddable Rust Swift demangler exists; even Ghidra shells out to `swift-demangle`) **and** the most intricate reader (53 relative-pointer/trailing-object descriptor structs) for a **naming-only** payoff on the narrowest audience (Apple Mach-O). A PROPOSAL stub, not a committed PR sequence.

---

## 2. The shared seam (proven, zero engine-path change)

All three recommended passes drop into the existing analysis seam with no new engine plumbing. Verified against the code:

- **Pass trait + ctx:** `AnalysisPass { stage(), id(), run(&AnalysisCtx) -> AnalysisOutput }`; `AnalysisCtx { file: &object::File, bytes, image, arch, listing }` — `decompiler/crates/kuna-analysis/src/pass.rs`. Pure, additive, never-failing.
- **Fact vectors (all already exist):** `AnalysisOutput { symbols: Vec<SymFact>, prototypes: Vec<PrototypePieces>, locals: Vec<LocalFact>, comments: Vec<CommentFact>, fid_names: Vec<FidMatch>, readonly: Vec<(u64,u64)>, entries, entry_names, … }` — `pass.rs:251`. `SymFact` = idempotent add; `FidMatch` = **rename a `FUN_*`/`sub_*` placeholder only** (label-gated, never clobbers a real symbol).
- **Registration:** append a `Box::new(...)` in `decompiler/crates/kuna-analysis/src/passes.rs::passes_for()`, behind a format/compiler gate (the Golang precedent: `if compiler.is_golang() { passes.push(...) }`).
- **Gating dispatch:** add an arm to `analysis_pass_enabled()` at `decompiler/crates/kuna-console/src/engine.rs:363` (e.g. `"rtti" => arch.analysis_rtti`); the flag lives on `Architecture` beside `analysis_dwarf:bool`@`architecture.rs:474` and `analysis_fid:bool`@`542`; the `--option` surface is an `on_off!(analysis_rtti, …)` arm at `engine.rs:1086` (beside `"dwarf"`@1086 / `"fid"`@1103).
- **LLM control surface:** one `[[settable]]` row in `decompiler/crates/kuna-decomp/stages.toml`, cloning the **fid row at `:2242`** (`values="on|off"`, `default="off"`, `stage="S1"`, `substage="external-refinement"`, `strength="HARD"`).
- **Commit arms (all reused verbatim):** `out.symbols`→add-function/data; `out.fid_names`→address-keyed rename; `out.prototypes`→`set_function_prototype_pieces`; `out.locals`→`dwarf_locals`/`seed_mapped_symbols`; `out.comments`→`commentdb user2`; `out.readonly`→read-only ranges.
- **Demangler:** `decompiler/crates/kuna-analysis/src/s1_demangle/mod.rs::demangle_name(raw)->Option<String>` — MSVC arm fires on a leading `?` (RTTI0 names are `.?AV…@@`; strip the `.` first).

**Parity safety is structural:** the XML datatest path never calls `run_default_analyses`, and all three passes are **default-off**, so `make test` (675/675), `make test-stages`, and `make rust-test` stay green by construction regardless of what these emit. Both precedents are exact: **`s1_fid`** = the default-off, externally-gated, *rename*-emitting pass shape; **`s1_dwarf`** = the type+prototype+local-emitting pass with the `build_datatype`/`TypeFactory` mapper.

---

## 3. Per-analyzer specs

### 3.1 Analyzer 1 — MSVC RTTI / vftables (`s1_rtti`) — BUILD FIRST

**(a) What it parses + where.** Five structures + the vftable, **entirely in the PE image** (almost always `.rdata`; scans `.data`/`.text` too). No external file. Ghidra source: `RttiAnalyzer.java`, `Rtti{1..4}Model`/`VfTableModel`/`RttiUtil`, `RTTI{0..4}DataType.java`, `TypeDescriptorModel.java`.
- RTTI0 `TypeDescriptor` — carries the `.?A…`-mangled class name (the only text-bearing struct).
- RTTI4 `CompleteObjectLocator` (COL, the root, 20 B) → RTTI0 + RTTI3; sits immediately before the vftable (`meta = vftable_addr - ptr`).
- RTTI3 `ClassHierarchyDescriptor` (16 B) → RTTI2 array of `numBaseClasses` RTTI1s.
- RTTI2 `BaseClassArray` — the inheritance list (4-byte entries on **both** arches).
- RTTI1 `BaseClassDescriptor` (28 B) — per-base `pmd{mdisp,pdisp,vdisp}` subobject offsets.
- vftable — array of `ptr_size` virtual-method pointers into `.text`.

**Walk** (port of `RttiAnalyzer.added()`): find the common `type_info::vftable` (scan `.data*` for `.?A`; word at `string−2*ptr` is the candidate `pVFTable`; ≥5 hits agreeing = it) → locate RTTI0s (words pointing at it) → for each, byte-search `.rdata`/`.data`/`.text` for a ref to it, `−12` = COL base, validate the COL→RTTI3→RTTI2→RTTI1→RTTI0 reachability chain → byte-search a ref to each COL, the word *after* = vftable base, size by walking slots to a non-`.text`/non-function/NULL boundary.

**(b) Decompiler-visible win (before/after).** Source: `struct Shape{virtual int area();}; struct Box:Shape{int s; int area() override{return s*s;}};`
- *Before:* the virtual call is `(**(code **)*plVar1)()`, the vtable slot is `DAT_<addr>`, `.rdata` is raw bytes, no class names.
- *After (`--option rtti on`):* the vftable VA is labeled `Box::vftable`, its slot points at a function named from `Box::area`, RTTI0 is `Box::RTTI_Type_Descriptor`, and `Box`/`Shape` exist as recovered class names — the virtual dispatch resolves to the named method.

**(c) Module layout (gated default-off, the `s1_fid`/`s1_dwarf` precedent).**
```
kuna-analysis/src/s1_rtti/
  mod.rs      RttiPass (impl AnalysisPass, stage=S1, id="rtti"): the added()-style walk
  models.rs   RTTI0..4 + vftable byte-layout readers (the 5 *Model.java ports)
  refkind.rs  ptr_or_disp dispatch on is64Bit: VA (x86) vs IBO32 imagebase+disp32 (x64)
  vftable.rs  VfTableModel.getVfTableCount walk + meta-ptr / NULL terminator
```

**(d) Seams reused.** Format loader (PE `.rdata`/`.data`/`.text` already wired); **demangler `s1_demangle` MSVC arm — zero new demangler code**; `build_datatype`-style `get_type_struct(name)` for named-opaque class types; commit via `out.symbols` (RTTI_*/`type_info::vftable` labels + per-slot virtual-method functions), `out.fid_names` (rename a virtual method `sub_*` to its demangled form), `out.readonly` (mark RTTI/vftable ranges).

**(e) Fixture + testcase.** `msvc_rtti.cpp` (one polymorphic class) → linked PE, built in `kuna-dev` (the `msvc_mangled.obj` recipe already proves `clang -target x86_64-pc-windows-msvc` emits the real MSVC C++ ABI; RTTI needs a *linked* PE with `.rdata`, so add `lld-link` / `-fuse-ld=lld`):
```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc '
  F=decompiler/crates/kuna-analysis/tests/fixtures
  clang -target x86_64-pc-windows-msvc -fuse-ld=lld -O1 $F/msvc_rtti.cpp -o $F/msvc_rtti.exe'
```
Check in `msvc_rtti.exe` + `.cpp` + README entry pinning COL/RTTI0/vftable VMAs (from `llvm-readobj`/`objdump -s .rdata`). Build an **x64** (IBO32 path) and an **x86** (raw-VA path) variant to exercise the branch. `kuna-console/tests/verify_rtti.rs` asserts the labels + demangled names appear gated-on, absent gated-off.

**(f) Effort/risk.** **S–M.** Risk: the **x86/x64 ref-kind branch** is the one place a port goes wrong — `is64Bit` makes inter-structure refs **4-byte image-base displacements** (`imageBase+disp32`), *not* 8-byte pointers, RTTI0 name moves offset 8→16, RTTI2 entries stay 4 bytes. Isolated in `refkind.rs`.

**(g) Documented LOSS/scope.** MSVC dialect only (Itanium `_ZTV`/`_ZTI` RTTI = a separate future pass reusing the *Itanium* `s1_demangle` arm). Full class-struct/inheritance reconstruction is Ghidra's **script-tier** `RTTIWindowsClassRecoverer` — the analyzer ships the labeled/typed metadata graph (RTTI1 `pmd` offsets as labels), not reconstructed struct types. The x64 COL `pSelf` 6th field is ignored (5-field model suffices for discovery, faithful to Ghidra).

---

### 3.2 Analyzer 2 — Mach-O Objective-C (`s1_objc`) — BUILD SECOND

**(a) What it parses + where.** ObjC 2.0 metadata, **entirely in the Mach-O** `__DATA`/`__DATA_CONST` `__objc_*` sections (strings in `__TEXT` `__objc_methname`/`__objc_classname`). No dSYM/dyld-cache for a normal dylib/exe. Ghidra source: `ObjcTypeMetadataAnalyzer`/`Objc2TypeMetadata` + `Objc2Class`/`Objc2ClassRW`/`Objc2Method`/`Objc2MethodList` + `ObjcMethod.applyTo`/`ObjcUtils.createMethods`; `ObjcMessageAnalyzer` (the dispatch half).

**Walk** (minimal viable path): confirm ObjC (any `__objc_*` block) → **pre-rebase chained fixups** if present (the load-bearing wrinkle, see PR O0) → walk `__objc_classlist` (`class_t*`, stride=ptr); per class read `data`, mask `FAST_DATA_MASK` (`~0x7`/`~0x3`) → `class_ro_t`; `.name`→class name; `.baseMethods`→`method_list_t`; follow `class_t.isa`→metaclass baseMethods for `+` methods → walk `method_list_t` (`entsizeAndFlags & 0x80000000` = small/relative list), per `method_t` resolve `name` (selector), `types` (encoding), `imp` (function VA; small-list `imp=listBase+offset`; strip 32-bit ARM Thumb LSB).

**(b) Decompiler-visible win (before/after).** Source: a `Greeter` class with `-(int)greet:(int)n`.
- *Before:* `int sub_1000038c0(undefined8 self, undefined8 _cmd, int n){ return n*3+7; }`.
- *After (`--option objc on`):* the IMP is **named** `-[Greeter greet:]` and (PR O2) signed `id self, SEL _cmd, int n`.

**(c) Module layout.**
```
kuna-analysis/src/s1_objc/
  mod.rs       ObjcMetadataPass (impl AnalysisPass, id="objc"): orchestration + run()
  sections.rs  __objc_* locate (Objc2Constants port) + ptrSize/endian
  classt.rs    class_t / class_ro_t reader (FAST_DATA_MASK)
  methods.rs   method_list_t / method_t walk (small-list relative form)
  thumb.rs     32-bit ARM Thumb-bit strip on IMP addrs
```
(the chained-fixup resolver — PR O0 — lands shared in `s1_loader/format/macho.rs`, not here.)

**(d) Seams reused.** Format loader (`MachOFormat` typed path already reads `__stubs`/section flags); **no demangler** (selectors are ASCII); `build_datatype`→named-opaque class structs + a small ObjC type-encoding decoder (not a demangler) for method/ivar types; commit via `out.fid_names`/`out.symbols` (IMP `-[…]` names + `_OBJC_CLASS_$_*` symbols + selector strings), `out.prototypes` (IMP signatures), `out.readonly`.

**(e) Fixture + testcase.** `macho_objc.m` (one ObjC class) → self-contained Mach-O, built in `kuna-dev` with bare `clang -target …-apple-macos` + rustup's `ld64.lld` and `-undefined dynamic_lookup` (the **exact** `macho_imports` recipe, which proves linked self-contained Mach-O builds without a macOS SDK):
```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc '
  F=decompiler/crates/kuna-analysis/tests/fixtures
  clang -target x86_64-apple-macos11 -fobjc-arc -O1 -c $F/macho_objc.m -o m.o
  LLD=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV|sed -n "s/host: //p")/bin/gcc-ld/ld64.lld
  "$LLD" -arch x86_64 -platform_version macos 11.0 11.0 \
         -undefined dynamic_lookup -e _main -o $F/macho_objc m.o'
```
x86-64 first (**no chained fixups → MVP-clean**, like `macho_imports`); an arm64 variant lands with PR O0. **Fallback if bare clang can't synthesize `__objc_*` without SDK objc headers:** check in a real macOS-built `.dylib` slice + a "built on macOS" README note (the same posture as `macho_fat` being hand-assembled). `verify_objc.rs` pins the IMP/selref/class VMAs and asserts the rename gated-on, absent gated-off.

**(f) Effort/risk.** **M.** Risk concentrates in the **chained-fixup prerequisite** (PR O0): on `LC_DYLD_CHAINED_FIXUPS` binaries the raw `__objc_*` slots are packed fixup entries, not addresses — a parser that skips rebasing reads garbage from `__objc_classlist`. Mitigated by sequencing the x86-64 (no-fixups) naming win first and gating arm64 behind PR O0.

**(g) Documented LOSS/scope.** ObjC **v2 only** (legacy `__module_info` v1 out of scope). DYLD-shared-cache extraction (`__objc_opt_ro` relative-selector base) out of scope (standalone Mach-O only). Chained-fixup: plain rebase + arm64e auth-rebase handled; bind/import-ordinal chains out of scope. Full `objc_msgSend` **call-override** deferred — PR O3 ships a selref→selector EOL `CommentFact` annotation (the cheap high-value half); rewriting the callee needs a `CALL_OVERRIDE` mechanism kuna lacks at this tier + a receiver def-use back-trace.

---

### 3.3 Analyzer 3 — PDB (`s1_pdb`) — BUILD THIRD (the external-file case)

**(a) What it parses + where.** PDB is **Windows' DWARF**: function names+addresses, full types (TPI/IPI), params + typed stack locals, source lines — living in a **separate `.pdb` file** the PE only references. The PE's CodeView debug-directory entry holds only a fingerprint (`NB10{sig,age,path}` or `RSDS{guid,age,path}`). Ghidra source: `PdbUniversalAnalyzer.java` (the pure-Java one; the MS-DIA `PdbAnalyzer` is Windows-native — never port), `PdbInfoCodeView`/`PdbInfoDotNet` (the PE record), `pdb2/pdbreader` (the parser — substituted, §5).

**(b) Decompiler-visible win.** On stripped-but-PDB: `FUN_00401040` → `WinMain`; global `DAT_*` → real names; call sites get typed params; stack `local_*`/`iVar*` → source name+type; (gated) `file:line` comments. The DWARF-for-Windows payoff.

**(c) Module layout (modeled on `s1_dwarf/mod.rs`).**
```
kuna-analysis/src/s1_pdb/
  mod.rs       PdbPass (impl AnalysisPass, id="pdb"): the s1_dwarf-shaped run()
  codeview.rs  PE debug-directory CodeView record extractor (NB10/RSDS → {guid|sig, age, path})
  locate.rs    tier-1 --option pdb_path + (tier-2) same-dir/local-store; fingerprint gate
  typemap.rs   pdb::TypeData -> Rc<Datatype> (the build_datatype tag-switch shape)
  walk.rs      global+module symbol/scope walk -> SymFact/PrototypePieces/LocalFact
```

**(d) Seams reused — the strongest reuse story.** PDB names+types land through the **exact** `s1_dwarf` machinery: the `pdb::TypeData→Datatype` mapper is structurally identical to `build_datatype` (recursion cap, `get_type_pointer`/`get_type_array`/`get_type_struct`/`get_base`/`get_type_void`) — **same `TypeFactory` API, no new engine type machinery**; the scope walk reuses `build_pieces` (params) and the `collect_fbreg_locals` `LocalFact` shape (PDB `RegisterRelative` offset → `LocalFact.stack_offset`, the `DW_OP_fbreg` analog — the `S_FRAMEPROC` register replaces the per-arch `call_frame_cfa` guess). **All four commit arms apply unchanged** (`symbols`/`prototypes`/`locals`/`comments`). Only-new-code = `s1_pdb/` + the CodeView extractor.

**(e) Fixture + testcase.** A small PE built `clang -target x86_64-pc-windows-msvc -g -gcodeview` + `lld-link /debug` to emit both `foo.exe` (with an RSDS record) and `foo.pdb`, in `kuna-dev`. Two fixtures: PR P0 pins CodeView-record extraction (no `.pdb` yet); PR P1+ supplies the matching `.pdb` and asserts `FUN_*`→`WinMain` with `--option pdb on --option pdb_path foo.pdb`, absent gated-off. `verify_pdb.rs`.

**(f) Effort/risk.** **L.** Risk: the `TypeData→Datatype` mapper is net-new (comparable in scale to kuna's DWARF mapper, ~880 LOC) but bounded by staying **name-level-opaque** for the MVP (deepen to field-level later). The `pdb` crate is mature (Firefox symbolication, `pdb-addr2line`) and exposes the two streams that matter at the gimli abstraction.

**(g) Documented LOSS/scope.** **In scope (MVP):** RSDS+NB10 extraction; **tier-1 explicit `.pdb`** with GUID/age match; Public/Procedure→names, arglist→typed prototypes, RegisterRelative→typed locals; name-level types. **Cheap faithful add:** tier-2 location (same-dir + `<name>/<GUID><age>/` local store), no network. **Deferred/LOSS:** tier-3 symbol-server/msdl download + cabinet `.pd_` decompression (network/trust); full field-level struct/class layout, C++ vtables/namespaces (start name-level like `s1_dwarf`, deepen incrementally); line info default-off (`pdb_lines`, the `dwarf_lines` posture); the MS-DIA `PdbAnalyzer`.

---

## 4. Cross-cutting decisions

### 4.1 New crate deps (documented dependency-substitutions)

| Analyzer | New workspace dep | Already present | LOSS doc |
|---|---|---|---|
| RTTI | **none** | `object` 0.39 (PE), `msvc_demangler` | n/a |
| ObjC | **none** (hand-written chained-fixup resolver) | `object` 0.39 (Mach-O) | n/a |
| PDB | **`pdb = "0.8"`** (gimli's PDB analog) | `object` 0.39 | **yes** — beside `gimli`/`object` |
| Swift (deferred) | swift-demangle shell-out/FFI (worst story) | `object` 0.39 | yes — new shell-out LOSS |

`pdb` goes in `decompiler/Cargo.toml` `[workspace.dependencies]` next to `object = "0.39"`/`gimli = "0.33"`, with the dependency-substitution rationale in `docs/rust-port/losses.md` (Ghidra hand-rolls `pdb2/pdbreader` = ~196 type + ~280 symbol classes; kuna substitutes the mature `pdb` crate exactly as it substituted `gimli` for DWARF, `object` for BFD, `cpp_demangle`/`rustc_demangle`/`msvc_demangler` for the demanglers). `pdb.workspace = true` in `kuna-analysis/Cargo.toml`. Optionally `pdb-addr2line` for line info. **Adding the `pdb` dep is itself PROPOSAL-gated** (it's the one new external dependency).

### 4.2 The PDB external-file decision (the FID-database analog)

The PE carries only the GUID/sig+age fingerprint, never the debug info, so PDB recovery is **conditional on the user supplying the `.pdb`** — the same shape as FID needing a `.fid` DB. The decision:

- **Tier 1 (MVP, the only required path): an explicit `--option pdb_path <file>`.** This is the per-decompilation control-surface analog of FID's `kuna_fid_db` **env var**. We recommend a real `--option pdb_path <file>` rather than an env var because PDB is **per-binary** (FID's DB is per-corpus, which is why FID uses an env var). Mirrors `PdbUniversalAnalyzer.setPdbFileOption`. Settable via `--option pdb on` + `--option pdb_path <file>`.
- **Tier 2 (cheap faithful sidecar, optional PR):** same-dir `<name>.pdb` next to `<name>.exe`, plus the local-store layout `<name>/<GUID><age>/<name>.pdb`. No network.
- **Tier 3 (out of scope, LOSS):** symbol-server / msdl download + cabinet `.pd_` decompression (network/trust).
- **The fingerprint gate (FID-hash-match analog):** open the supplied `.pdb`, verify `pdb_information().guid/age == the CodeView record`. Mismatch/absent → empty output. Never apply a stale/wrong PDB — the exact "don't apply wrong external knowledge" discipline FID's full-hash bucket-collapse enforces.

---

## 5. Ordered PR breakdown

Every PR: a gated default-off pass + one gate arm (`analysis_pass_enabled` + `on_off!` + an `analysis_<id>` flag on `Architecture`, default-false) + one `[[settable]]` row (clone the fid row at `stages.toml:2242`) + **one fixture testcase** + `make test`/`make rust-test`/`make test-stages` green + real-binary-path only.

**RTTI (highest ROI — start here):**
- **PR R1** — `s1_rtti` skeleton + PE/VS-Clang gating + in-`.rdata` walk (x86 path): find `type_info::vftable`, locate RTTI0s, demangle names (reuse MSVC arm), emit class `SymFact`s + `RTTI_*` labels. *Files:* `s1_rtti/{mod,models,refkind}.rs`, `passes.rs`, `engine.rs`, `architecture.rs`, `stages.toml`. *Fixture:* x86 `msvc_rtti.exe` + `verify_rtti.rs`. *Deps:* none. *Risk:* low. **The class-name before/after lands here.**
- **PR R2** — x64 IBO32 displacement branch (image-base-relative refs, name offset 16) in `refkind.rs`. *Fixture:* x64 `msvc_rtti.exe`. *Risk:* the one critical branch; isolated.
- **PR R3** — vftable discovery (`vftable.rs`) + per-slot virtual-method function definition (`SymFact{Function}`) + `<Class>::vftable`/`vftable_meta_ptr` labels + `out.readonly` ranges + named-opaque class struct types. *Risk:* medium (vftable bounding).
- **PR R4 (optional)** — RTTI1/2/3 inheritance graph as labeled metadata (`pmd` offsets); full struct-layout reconstruction stays deferred.

**ObjC (second):**
- **PR O0 (prerequisite)** — Mach-O chained-fixup resolver in `s1_loader/format/macho.rs` (rebase `__DATA_CONST` slots; strip auth/PAC/next bits). *Fixture:* arm64 dylib with `LC_DYLD_CHAINED_FIXUPS`. *Risk:* **highest of the ObjC line** — bit-twiddling correctness; shared by ObjC and any future Swift.
- **PR O1** — `s1_objc` skeleton + Mach-O/`__objc_*` gating + classlist→class_ro_t→method walk (absolute-ptr, large method lists, 64-bit) → IMP `-[Class sel]` naming + function definition + class/selector symbols. *Fixture:* x86-64 `macho_objc` (no-fixups) + `verify_objc.rs`. *Deps:* none. **The headline before/after lands here.**
- **PR O2** — small relative method lists + ARM Thumb bit + 32-bit; ObjC type-encoding decoder → method/ivar `PrototypePieces`/types; ivar `Class::ivar` labels. *Fixture:* arm64 `macho_objc` (needs PR O0).
- **PR O3 (deferable)** — `objc_msgSend` selref→selector EOL `CommentFact` annotation. Own `objc_msgsend` flag. Full call-override remains deferred.

**PDB (third — highest value, external-file):**
- **PR P0** — CodeView record extractor (`codeview.rs`, `PdbInfoCodeView`/`PdbInfoDotNet` port) in/under `s1_loader/format/pe.rs`; surface `{guid|sig, age, path}`. *Fixture:* PE with an RSDS record (assert extraction; no `.pdb` yet). *Deps:* none. *Risk:* low.
- **PR P1** — **PROPOSAL-gated `pdb` crate add**; `s1_pdb` skeleton + tier-1 `--option pdb_path` + fingerprint gate (guid/age match) + `S_PUB32`/`S_GPROC32`→`SymFact`/`FidMatch` function naming. *Fixture:* PE + matching `.pdb` (stripped→`WinMain`) + `verify_pdb.rs`. *Risk:* medium (crate integration).
- **PR P2** — `TypeData→Datatype` mapper (`typemap.rs`, name-level opaque, the `build_datatype` shape) + arglist→`PrototypePieces` typed prototypes. *Risk:* medium (the largest net-new code).
- **PR P3** — `S_REGREL32` scope walk → typed stack `LocalFact`s (the `seed_mapped_symbols` path).
- **PR P4 (optional)** — tier-2 sidecar location (same-dir + `<name>/<GUID><age>/`); `pdb_lines` C13 source-line `CommentFact`s (default-off).

**Swift (deferred):**
- **PR S-PROPOSAL only** — a `docs/proposals/` stub documenting the relative-pointer reader cost (53 trailing-object structs), the `swift-demangle` shell-out LOSS, and the naming-only scope (type/field names + `$s` symbol demangling; skip generics/witness/conformance). No committed PR sequence until prioritized.

**PROPOSAL-gating (per the hard constraint):** three items each get a `docs/proposals/` PROPOSAL before code — (1) the **`pdb` crate dependency add** (PR P1), (2) the **Mach-O chained-fixup subsystem** (PR O0), (3) any **Swift** work. PR R1–R3, O1, P0 are incremental and each ship a fixture + `verify_*.rs` directly.

---

## 6. Risks, parity safety, and what is explicitly deferred

**Parity safety (the strongest guarantee).** All three passes are **default-off** and **real-binary-path only**. The XML datatest path never calls `run_default_analyses`, so `make test` (675/675), `make test-stages`, and `make rust-test` are **byte-identical regardless of what these passes emit** — exactly the `s1_fid`/`s1_dwarf` posture. Each PR re-runs all three gates before commit.

**Top risks (ranked).**
1. **Mach-O chained-fixup resolver (PR O0)** — the single cross-cutting prerequisite; without it `__objc_classlist` reads garbage on modern arm64e. *Mitigation:* PROPOSAL-gated; ship the x86-64 (no-fixups) ObjC naming win first (PR O1); gate arm64 behind O0; handle plain + arm64e auth-rebase, document bind/ordinal chains as out of scope.
2. **RTTI x86/x64 ref-kind branch (PR R2)** — 4-byte IBO32 displacements vs 8-byte pointers; the classic port-error point. *Mitigation:* isolated in `refkind.rs`; both-arch fixtures.
3. **PDB type mapper scale (PR P2)** — largest net-new code. *Mitigation:* name-level-opaque MVP matching `s1_dwarf`, deepen later.
4. **Fixture synthesis without macOS SDK** — bare clang may not emit `__objc_*`. *Mitigation:* checked-in real macOS-built slice + README note (the `macho_fat` precedent).

**Explicitly deferred.** Swift entirely (PROPOSAL stub); ObjC `objc_msgSend` call-override (annotation-only in O3); ObjC v1 + DYLD-shared-cache; RTTI full class-struct/inheritance reconstruction (script-tier) + Itanium RTTI; PDB symbol-server/cabinet (tier-3), full field-level types/vtables/namespaces, line info (default-off). All recorded in `docs/rust-port/losses.md` + each pass's module doc.

---

## 7. Bottom line

Build **RTTI → ObjC → PDB**, in that order. RTTI is the cheapest, highest-ROI, zero-new-dep win (the MSVC demangler is already wired). ObjC is the big macOS naming win, also zero-new-dep, gated behind one shared chained-fixup prerequisite. PDB is the highest *value* (names + full types, through the exact proven `s1_dwarf` commit path) but is the lone external-file case — cleanly handled by the FID-database precedent (`--option pdb_path` + a GUID/age fingerprint gate) and one new crate (`pdb`). **Swift is deferred** to a PROPOSAL stub: a new demangler shell-out dependency plus the most intricate reader for a naming-only payoff on the narrowest audience. Every recommended pass is a default-off, real-binary-path-only `AnalysisPass` that drops into the existing `AnalysisOutput`/`analysis_pass_enabled`/`stages.toml` seam with zero engine-path change, keeping all three parity gates green by construction.

**Key grounding paths:** seam `decompiler/crates/kuna-analysis/src/pass.rs` (`AnalysisOutput`@251) + `passes.rs`; templates `s1_dwarf/mod.rs` (type/proto/local) + `s1_fid/mod.rs` (default-off external-artifact); gate `decompiler/crates/kuna-console/src/engine.rs:363` + `on_off!`@1086 + flags `decompiler/crates/kuna-decomp/src/infra/architecture.rs:474,542`; settable schema `decompiler/crates/kuna-decomp/stages.toml:2242` (fid row); demangler `s1_demangle/mod.rs:200`; format readers `s1_loader/format/{pe,macho}.rs`; crate deps `decompiler/Cargo.toml` `[workspace.dependencies]` (`object`@33, `gimli`@70); fixtures + recipes `decompiler/crates/kuna-analysis/tests/fixtures/README.md` (§§ msvc_mangled@397, macho_imports@424).
