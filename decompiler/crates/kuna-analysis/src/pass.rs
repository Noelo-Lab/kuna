//! The program-prep analysis-pass interface.
//!
//! Generalizes the de-facto contract already used by [`crate::loader::elf_plt`]
//! (`resolve_plt_imports(&object::File) -> Vec<PltSym>`): a focused module that
//! reads the parsed object and produces a flat, deduplicated set of *facts*,
//! never panicking and never failing -- it only ever contributes *more*
//! knowledge (names, types, entries). A driver merges the facts from all enabled
//! passes and commits them once into the engine's symbol/type tables.
//!
//! This is the boundary every new analysis (string detection, demangling, DWARF,
//! function-start discovery, library prototypes, ...) plugs into; see
//! `docs/missing-analyses.md` for the roadmap. The concrete passes will land as
//! `loader`/`analyzers/*` modules implementing [`AnalysisPass`].

use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name, NameChars};

use crate::listing::Listing;
use crate::loadimage_object::ObjectLoadImage;

/// The phase a pass feeds, mirroring the kuna phase model (`docs/phases.md`).
/// Program-prep analyses are P0/P1; a few (e.g. jump-table post-typing
/// refinement) feed back into P2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// P0 -- knowledge & configuration plane (symbol DB facts).
    P0,
    /// P1 -- image & code partition (the loader/analyzer tier).
    P1,
    /// P2 -- flow & op-graph recovery (post-typing feedback).
    P2,
}

/// What kind of symbol a [`SymFact`] names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymKind {
    /// A function entry point.
    Function,
    /// A typed/untyped data object.
    Data,
}

/// A recovered symbol fact: an address gets a name.
#[derive(Clone, Debug)]
pub struct SymFact {
    /// Virtual address the symbol is at.
    pub addr: u64,
    /// The recovered (already demangled, if applicable) name.
    pub name: String,
    /// Function vs data.
    pub kind: SymKind,
}

/// A named data global: a symbol-table / DWARF data object at `addr` whose
/// storage spans `size` bytes.
///
/// Distinct from a plain [`SymFact`]`{Data}`: it carries the object's **byte
/// size** so the commit can map a covering `SymbolEntry` of the right extent.
/// A `SymFact{Data}` is installed with a size-1 code type, so a 4-/8-byte
/// memory access (`mov [max_width], eax`) queries `queryContainer(addr, 4)` and
/// finds nothing — the global renders `dat_<addr>` instead of `max_width`. This
/// fact maps the symbol with an `undefined<size>` type so the container query
/// matches at the real access width. Matches how **IDA Pro / Ghidra** name data
/// globals from the symbol table (`ApplyDataArchiveAnalyzer` / the symtab import).
///
/// Produced by the DWARF pass (top-level `DW_TAG_variable` with `DW_OP_addr`),
/// which resolves the variable's `DW_AT_type` to its byte size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataObjectFact {
    /// Virtual address of the data object.
    pub addr: u64,
    /// The recovered symbol name.
    pub name: String,
    /// Byte size of the object's storage (from `DW_AT_type`), `>= 1`.
    pub size: u32,
}

/// A detected NUL-terminated string literal: a `char[len]` data object at `addr`.
///
/// `len` is the byte length **including** the terminating NUL (visible chars + 1),
/// matching what `DataUtilities.createData` / a `char[N]` array should span — the
/// length the commit boundary passes to `get_type_array` so the printer renders the
/// literal.  Produced by [`crate::strings`] (the kuna analog of Ghidra's
/// `StringsAnalyzer`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StringFact {
    /// Virtual address of the first string byte.
    pub addr: u64,
    /// Array length in bytes, including the trailing NUL (visible_len + 1).
    pub len: u32,
}

/// A processor-context decode-mode paint: set context variable `var` to `value`
/// starting at `addr` (up to the next change point, or — if `end` is set — over
/// the explicit `[addr, end)` range). Produced by [`crate::loader::arm_markers`]
/// (the kuna analog of ARM's `ARM_ElfExtension`/`ArmSymbolAnalyzer` `TMode`
/// painting) and applied by the console's `commit_analysis_output` via the
/// engine's `ContextDatabase` (`set_variable` / `set_variable_region`, the exact
/// analog of Ghidra's `programContext.setValue(TMode, …)`).
///
/// `var` is a `&'static str` because the only producer paints SLEIGH-defined
/// context registers known at compile time (`"TMode"`); the commit boundary swallows
/// a "variable not registered" error so painting a var the active language does
/// not define (e.g. `TMode` on x86-64) is a faithful no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextPaint {
    /// Virtual address the paint starts at (already normalized — e.g. the Thumb
    /// FUNC LSB is masked off before this is emitted).
    pub addr: u64,
    /// Optional explicit end (exclusive) of the painted range. `None` paints from
    /// `addr` to the next change point (Ghidra's per-symbol `setValue(v,a,a,val)`
    /// point-set shape); `Some(end)` paints the bounded range `[addr, end)`.
    pub end: Option<u64>,
    /// The SLEIGH context variable name to paint (e.g. `"TMode"`).
    pub var: &'static str,
    /// The (unshifted) value to set.
    pub value: u32,
}

/// A function the known-no-return list matched: the engine should mark its
/// FunctionSymbol no-return. Produced by [`crate::loader::noreturn`] (the kuna
/// analog of Ghidra's `NoReturnFunctionAnalyzer`, "Known").
///
/// Carries the symbol **address** as well as the matched name. The address is the
/// stable key: demangling renames the funcsym before it is installed (a mangled
/// C++ no-return symbol like `_ZSt9terminatev` becomes `std::terminate` in scope
/// `std`), so a name-based commit (`query_global_function("_ZSt9terminatev")`)
/// misses it. The commit instead resolves the FunctionSymbol by address
/// (`Database::find_function_across_scopes(addr)`) — the same address the funcsym
/// was installed at — and only falls back to the name path when the address does
/// not resolve. Mirrors how `CallFixupAnalyzer`/`NoReturnFunctionAnalyzer` operate
/// on the already-installed function object, not on the raw symbol string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoReturnFact {
    /// Virtual address of the matched function symbol (the install address; stable
    /// across demangling).
    pub addr: u64,
    /// The matched function's original (pre-demangle) symbol name — the fallback
    /// resolution key when no function is installed at `addr` (e.g. an import that
    /// only exists as a PLT-named stub).
    pub name: String,
}

/// A named, typed function-LOCAL recovered from DWARF (`DW_TAG_variable` /
/// `DW_TAG_formal_parameter` with a `DW_OP_fbreg` stack location), to be installed
/// in the owning function's stack scope so the decompiler renders `int count`
/// instead of `local_1c`. Produced by [`crate::dwarf`] (subtask 3 — the kuna
/// analog of `DWARFFunctionImporter.processSubprogram`'s `commitLocal` loop +
/// `DWARFVariable.readLocalVariableStorage`).
///
/// `stack_offset` is already in **kuna stack-space coordinates** (the same space
/// the console `map addr s<off> ...` directive addresses): the producing pass has
/// applied the per-arch `call_frame_cfa` constant to the raw `DW_OP_fbreg` offset
/// (`stack_offset = call_frame_cfa + fbreg`, faithful to Ghidra's
/// `DWARFExpressionEvaluator` — `DW_OP_call_frame_cfa` pushes a stack varnode at
/// `getCallFrameCFA()`, then `DW_OP_fbreg` adds the LEB128 offset; see
/// `x86-64.dwarf` `<call_frame_cfa value="8"/>`). It is the raw (signed) stack
/// offset; the commit boundary wraps it to the stack space's unsigned address.
///
/// The commit parks each fact on its owning function (keyed by `func_addr`) and
/// re-seeds it into the freshly-rebuilt `Funcdata`'s `ScopeLocal` at decompile
/// time via the same `seed_mapped_symbols` path the console `map addr` directive
/// uses — a typelock|namelock stack symbol. Bound to the real-ELF DWARF path
/// (gated on the `dwarf` pass flag); the XML datatest path never produces one.
#[derive(Clone, Debug)]
pub struct LocalFact {
    /// Virtual address of the OWNING function's entry (`DW_AT_low_pc`); the key the
    /// commit uses to attach the local to the right function's stack scope.
    pub func_addr: u64,
    /// The DWARF source name of the local (e.g. `file`, `elf_header`).
    pub name: String,
    /// The recovered kuna [`Datatype`] (mapped from the variable's `DW_AT_type` DIE
    /// by the same DIE→Datatype mapper the typed-signature path uses).
    pub type_: std::rc::Rc<kuna_decomp::dtype::Datatype>,
    /// The stack offset in kuna stack-space coordinates (`call_frame_cfa + fbreg`),
    /// signed; the commit wraps it to the unsigned stack address.
    pub stack_offset: i64,
}

/// A source-line annotation to attach to a decompiled instruction: at `addr`,
/// the instruction came from source location `text` (e.g. `debug_symbol.c:122`).
/// The commit boundary (`engine.rs::commit_analysis_output`) installs each into the
/// architecture's `commentdb` as a `Comment::user2` (the instruction-comment
/// type the C printer emits as a `/* ... */` line at the op's address).
///
/// The kuna analog of Ghidra's `DWARFLineInfoCommentScript.addSourceLineInfo`,
/// which walks each compilation unit's `.debug_line` rows
/// (`DWARFLine.getAllSourceFileAddrInfo`) and `appendComment(addr, EOL,
/// "%s:%d".formatted(fileName, lineNum))`. Produced only by
/// [`crate::dwarf::DwarfLinesPass`] (the `.debug_line` parse, gated on the
/// `dwarf_lines` flag, default-off); empty otherwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentFact {
    /// Virtual address of the OWNING function's entry; the C printer keys comments
    /// by `(funcaddr, addr)`, so a comment is placed in the right function.
    pub func_addr: u64,
    /// The instruction virtual address the comment is attached to (a `.debug_line`
    /// row's `address`).
    pub addr: u64,
    /// The comment body — the source location, `file:line` (no `/* */`; the printer
    /// adds the C delimiters).
    pub text: String,
}

/// A tracked register **value** to seed at a function entry: at `func_addr`, the
/// register named `reg` holds the constant `value`. Distinct from
/// [`ContextPaint`] (which steers SLEIGH instruction *decode* via a context BIT):
/// this seeds a register-VALUE the decompiler's constant analysis propagates (the
/// kuna analog of Ghidra's `ProgramContext.setRegisterValue` / the console
/// `set track <reg> <val> <start> <end>`). Produced by
/// [`crate::loader::mips_markers`] (the kuna analog of Ghidra's
/// `MipsAddressAnalyzer`), which seeds `t9 = func_addr` per function — the MIPS PIC
/// ABI convention (a function is reached via `jalr t9`, so on entry `t9` holds the
/// callee entry). That lets a PIC prologue's `addu gp,gp,t9` fold to the real `$gp`,
/// so `$gp`-relative GOT/`.sdata` loads resolve to concrete addresses.
///
/// The commit boundary (`engine.rs::commit_analysis_output`) resolves `reg` to its
/// register varnode and calls `create_set(func_addr, func_addr+1)` + pushes a
/// `TrackedContext{loc, val}` — the exact `IfcSettrackedrange` recipe. The
/// per-function `build_arch_handle` then snapshots the track base into the handle,
/// and `ActionConstbase` (S3) emits `COPY #val -> reg` at the entry block, feeding
/// constant propagation. `reg` is a `&'static str` because the only producer names
/// SLEIGH registers known at compile time (`"t9"`); the commit boundary swallows a
/// "register not found" error so seeding a register the active language does not
/// define is a faithful no-op (belt-and-suspenders on top of the MIPS-only gate).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackedRegFact {
    /// Virtual address of the function entry the value is seeded at.
    pub func_addr: u64,
    /// The SLEIGH register name to seed (e.g. `"t9"`).
    pub reg: &'static str,
    /// The constant value the register holds at `func_addr`.
    pub value: u64,
}

/// A function whose name matched a cspec call-fixup `<target>`: the engine should
/// tag it with that fixup's inject id so the CALL is replaced by the fixup body.
/// Produced by [`crate::callfixup`] (the kuna analog of Ghidra's
/// `CallFixupAnalyzer`).
///
/// Carries the **original installed name** (not the matched fixup name), like
/// [`AnalysisOutput::noreturn`] does: the commit boundary resolves the real
/// FunctionSymbol via `query_global_function(func_name)` and re-derives the fixup
/// the same way the pass did. A name that no longer resolves (or already carries a
/// fixup) is the faithful no-op (Ghidra only sets when `getCallFixup()==null`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallFixupFact {
    /// The matched function's installed name (e.g. `mcount`, `__fentry__`).
    pub func_name: String,
}

/// A function the FID matcher re-identified by full-hash fingerprint: at `addr`,
/// the function's instruction-stream fingerprint matched a known-library record
/// whose name is `name` (e.g. `kuna_crc32`). Produced by [`crate::fid::FidPass`]
/// (the kuna analog of Ghidra's FID identification analyzer) — the capability that
/// renames a `FUN_<addr>`/`sub_<addr>` in a **stripped** binary back to its library
/// name purely by fingerprint.
///
/// Distinct from a [`SymFact`]: the commit SymFact path is an *idempotent add* and
/// would no-op on the already-installed (placeholder-named) function. FID instead
/// **renames** the existing function — resolved by ADDRESS — and only when it still
/// carries an engine `FUN_*`/`sub_*` placeholder (the label gate); a real
/// `.symtab`/DWARF name is never overwritten. See `engine.rs::commit_analysis_output`
/// (the FID rename arm) and `docs/history/fid-design.md` §6.2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FidMatch {
    /// Virtual address of the re-identified function's entry (the rename target).
    pub addr: u64,
    /// The library name the full-hash fingerprint matched (e.g. `kuna_crc32`).
    pub name: String,
}

/// The facts one analysis contributes. Every field is additive and may be empty;
/// merging two outputs is concatenation (the driver dedups by address).
#[derive(Default, Debug)]
pub struct AnalysisOutput {
    /// `(addr, name, kind)` -> seed `Database::add_function` / symbol map.
    pub symbols: Vec<SymFact>,
    /// Named, sized data globals (the symbol-table / DWARF data objects). Mapped
    /// into the global scope with an `undefined<size>` type so a memory access of
    /// the object renders its NAME (`max_width`) not `dat_<addr>`. See
    /// [`DataObjectFact`]. Produced by the DWARF pass; empty otherwise.
    pub data_objects: Vec<DataObjectFact>,
    /// Discovered function entry points (for stripped targets).
    pub entries: Vec<u64>,
    /// (kuna) `.eh_frame` FDE bodies that describe exactly one function, as
    /// `[pcBegin, pcBegin + pcRange)`. The commit boundary rejects any
    /// [`Self::entries`] VMA strictly inside one of these — a discovered entry in
    /// the middle of a frame is a landing pad / gap-walk artifact, not a function.
    /// Produced only by [`crate::entry::kuna_fdeinterior`] (`fdeinterior`), so the
    /// list is empty whenever that gate is off and the suppression is a no-op.
    pub fde_bodies: Vec<(u64, u64)>,
    /// Optional Ghidra-faithful names for a *subset* of [`Self::entries`], keyed by
    /// VMA. An overlay the commit boundary consults: when a discovered entry VMA appears
    /// here, the commit names the function with the supplied name (e.g. `_INIT_0`,
    /// `_FINI_0`, `_DT_INIT`) instead of the generic `name_function` `sub_<addr>`;
    /// an entry absent from the overlay is named generically, exactly as before.
    ///
    /// This is the kuna analog of `ElfProgramBuilder.createDynamicEntryPoints`, which
    /// names each `DT_INIT`/`DT_FINI` array element `_INIT_<i>` / `_FINI_<i>` /
    /// `_PREINIT_<i>` (i = the element's index in the array) and the single
    /// `DT_INIT`/`DT_FINI` entries `_DT_INIT` / `_DT_FINI`. Produced ONLY by
    /// [`crate::entry`]'s dynamic oracle (oracle 2); the other oracles leave the
    /// overlay empty so their entries keep the generic `sub_<addr>` name. Kept as a
    /// parallel overlay (NOT folded into [`Self::entries`]) so the *discovery* set —
    /// WHICH VMAs are found — is byte-identical to before this overlay existed.
    ///
    /// The commit keeps its idempotent `find_function` no-op, so a non-stripped
    /// binary's real `.symtab`/`.dynsym` name still wins — only a genuinely new
    /// (never-symboled) array-element start receives the `_INIT_<i>` name.
    pub entry_names: Vec<(u64, String)>,
    /// Functions that do not return (`exit`, `abort`, ...), by `(addr, name)`.
    /// The commit resolves each by ADDRESS (stable across demangling), falling
    /// back to the name path. See [`NoReturnFact`].
    pub noreturn: Vec<NoReturnFact>,
    /// (kuna, Ghidra-gap) Addresses of `call error(nonzero,…)` / `error_at_line`
    /// sites whose fall-through must be PRUNED — glibc `error()` with a nonzero
    /// constant status calls `exit()` and never returns, so the CALL must not fall
    /// through (Ghidra flags it "Subroutine does not return"). Committed as
    /// `CALL_RETURN` flow overrides at the decompile boundary; without this the
    /// flow-follower walks past the call into the NEXT function and absorbs it
    /// (the boundary-overrun class, ~50% of kuna's Ghidra GED gap).
    pub no_fallthru_calls: Vec<u64>,
    /// Extra read-only address ranges (e.g. `.got` after relocation).
    pub readonly: Vec<(u64, u64)>,
    /// (kuna) Address ranges holding an **external reference** — an import slot
    /// whose contents the dynamic loader fills in with the address of a function
    /// living in another image (a PE Import Address Table entry). Committed as
    /// `Varnode::externref` in the symbol-table property map, which is the one
    /// flag `ActionDeindirect` requires before it will resolve a `CALLIND` through
    /// a global to the FunctionSymbol registered at that address. Produced by the
    /// `peimportcall` pass; empty for every other format. See
    /// [`crate::loader::kuna_peimportcall`].
    pub externref: Vec<(u64, u64)>,
    /// Detected NUL-terminated string literals (a typed `char[N]` per address).
    pub strings: Vec<StringFact>,
    /// Library-function prototypes to seed onto matching FunctionSymbols (the kuna
    /// analog of Ghidra's `ApplyDataArchiveAnalyzer` / `.gdt` archives). Each is
    /// parked on its named callee via `set_function_prototype_pieces`, so a caller
    /// copies the callee signature and the argument constants get typed (e.g.
    /// `puts(char*)` types `0x400915` as `char*`, rendering the string literal).
    pub prototypes: Vec<kuna_decomp::fspec::PrototypePieces>,
    /// Processor-context decode-mode paints (the kuna analog of ARM's
    /// `ARM_ElfExtension`/`ArmSymbolAnalyzer` `programContext.setValue(TMode,…)`).
    /// Each sets a SLEIGH context variable over an address range; the commit boundary
    /// applies them to the engine's `ContextDatabase` BEFORE any instruction is
    /// decoded, steering ARM/Thumb instruction decode. Produced only on the ARM
    /// path (see [`crate::loader::arm_markers`]); empty otherwise.
    pub context_paints: Vec<ContextPaint>,
    /// Tracked register-values to seed at function entries (the kuna analog of
    /// Ghidra's `MipsAddressAnalyzer` `ProgramContext.setRegisterValue` / the
    /// console `set track <reg> <val> <start> <end>`). The commit boundary resolves
    /// each register and seeds the value over `[func_addr, func_addr+1)`; the
    /// per-function `build_arch_handle` snapshots it and `ActionConstbase` emits
    /// `COPY #val -> reg` at the entry, which constant propagation consumes (so a
    /// MIPS PIC `$gp`-relative load resolves). Produced only on the MIPS path (see
    /// [`crate::loader::mips_markers`]); empty otherwise.
    pub tracked_regs: Vec<TrackedRegFact>,
    /// Functions whose names matched a cspec call-fixup `<target>` (e.g. the `-pg`
    /// `mcount`/`__fentry__` profiling stubs). The commit boundary tags each with the
    /// matched fixup's inject id so the engine replaces the CALL with the fixup
    /// body. The kuna analog of Ghidra's `CallFixupAnalyzer` install loop.
    pub call_fixups: Vec<CallFixupFact>,
    /// Named, typed stack LOCALS recovered from DWARF (`DW_OP_fbreg` variables /
    /// parameters), each to be installed in its owning function's stack scope so the
    /// decompiler renders the source name+type instead of `local_*`/`iVar*`. The
    /// commit parks each by `func_addr` and re-seeds it into the rebuilt `Funcdata`'s
    /// `ScopeLocal` at decompile time (the `map addr` / `seed_mapped_symbols` path).
    /// Produced only by the DWARF pass (subtask 3); empty otherwise. See
    /// [`LocalFact`].
    pub locals: Vec<LocalFact>,
    /// Source-line annotations recovered from DWARF `.debug_line`, each to be
    /// installed as a `Comment::user2` so the decompiled output carries the
    /// `file:line` of the instruction. The commit boundary writes each into the
    /// architecture's `commentdb`. Produced only by the DWARF lines pass
    /// (`dwarf_lines`, default-off); empty otherwise. See [`CommentFact`].
    pub comments: Vec<CommentFact>,
    /// Functions the FID matcher re-identified by full-hash fingerprint, by
    /// `(addr, name)`. The commit boundary **renames** each (resolved by ADDRESS) only
    /// when it still carries an engine `FUN_*`/`sub_*` placeholder (the label gate)
    /// — the stripped-binary re-identification the FID pass exists for. Produced
    /// only by [`crate::fid::FidPass`] (`--option fid on`, default-off, real-ELF
    /// path only, and only when a `.fid` database is configured); empty otherwise.
    /// See [`FidMatch`].
    pub fid_names: Vec<FidMatch>,
    /// (kuna `cppproto`) The DWARF recoveries that only the C++ arm of the DWARF
    /// pass produces, held APART from the streams above so the commit boundary can
    /// drop them wholesale under `--option cppproto off`. The producing pass runs
    /// at load, upstream of the `option` commands, so the gate cannot live inside
    /// it — this split is what makes the flag live. See [`CppDwarfFacts`].
    pub cpp_dwarf: CppDwarfFacts,
    /// (kuna `cppsig`) The prototypes recovered from the MANGLED SYMBOL, split by
    /// how certain the pass is that argument 0 is an implicit `this`. Held apart
    /// for the same reason as [`Self::cpp_dwarf`] — the producing pass runs at
    /// load, upstream of the `option` commands — and additionally because the
    /// gate is three-valued (`off|proven|inferred`), so the commit boundary picks
    /// which tiers to apply. See [`CppSigFacts`].
    pub cpp_sig: CppSigFacts,
}

impl AnalysisOutput {
    /// (kuna `symbolnamechars`, GH-340) Apply the symbol-name character
    /// sanitizer to every NAME this output carries, at the mint.
    ///
    /// The loader's own `.symtab`/`.dynsym`/PLT walks sanitize inside
    /// [`crate::loadimage_object`]; this is the same treatment for the second
    /// channel a name arrives through — a DWARF `DW_AT_name`, a Go `pclntab`
    /// entry, a PDB public symbol, an RTTI class label — all of which are
    /// equally binary-supplied and reach emitted C through the same printer.
    ///
    /// Only names are touched. A `CommentFact`'s text is not a name and is left
    /// alone deliberately: it is emitted inside a `/* … */` by the printer, and
    /// escaping THAT belongs with the printer's comment emission, not with a
    /// symbol-name gate.
    pub fn sanitize_names(&mut self, mode: NameChars) {
        if mode == NameChars::Off {
            return;
        }
        let fix = |s: &mut String| {
            if let std::borrow::Cow::Owned(v) = sanitize_symbol_name(s, mode) {
                *s = v;
            }
        };
        let fix_proto = |p: &mut kuna_decomp::fspec::PrototypePieces| {
            fix(&mut p.name);
            p.innames.iter_mut().for_each(fix);
        };
        self.symbols.iter_mut().for_each(|s| fix(&mut s.name));
        self.data_objects.iter_mut().for_each(|d| fix(&mut d.name));
        self.entry_names.iter_mut().for_each(|(_, n)| fix(n));
        self.locals.iter_mut().for_each(|l| fix(&mut l.name));
        self.fid_names.iter_mut().for_each(|f| fix(&mut f.name));
        self.prototypes.iter_mut().for_each(fix_proto);
        self.cpp_dwarf.symbols.iter_mut().for_each(|s| fix(&mut s.name));
        self.cpp_dwarf.locals.iter_mut().for_each(|l| fix(&mut l.name));
        self.cpp_dwarf.prototypes.iter_mut().for_each(|(_, p)| fix_proto(p));
        self.cpp_sig.proven.iter_mut().for_each(|(_, p)| fix_proto(p));
        self.cpp_sig.inferred.iter_mut().for_each(|(_, p)| fix_proto(p));
    }
}

/// (kuna `cppsig`) Prototypes read off a C++ mangled symbol, in two certainty
/// tiers.
///
/// Itanium mangling does not distinguish a static member function from a
/// non-static one, and adding a `this` that is not there shifts EVERY parameter
/// by one position — strictly worse than leaving the function untouched. So the
/// pass computes both tiers at load and the commit boundary selects with
/// `--option cppsig`: `proven` applies only the first, `inferred` applies both.
/// Both are keyed by ENTRY ADDRESS, like [`CppDwarfFacts::prototypes`] and for
/// the same reason.
#[derive(Default, Debug)]
pub struct CppSigFacts {
    /// The mangling ENTAILS the answer: a constructor, a destructor, a
    /// cv-/ref-qualified member (all three take `this`), an unqualified global
    /// name or an explicit MSVC `__cdecl` (none of which can).
    pub proven: Vec<(u64, kuna_decomp::fspec::PrototypePieces)>,
    /// A nested name whose `this`-ness was decided from class evidence mined out
    /// of the binary's own symbols (a scope owning a ctor/dtor/cv-member or a
    /// `_ZTV`/`_ZTI`/`_ZTS` symbol is a class; one with no such witness is a
    /// namespace).
    pub inferred: Vec<(u64, kuna_decomp::fspec::PrototypePieces)>,
}

/// (kuna `cppproto`) The DWARF facts recovered by resolving a subprogram DIE
/// through its `DW_AT_specification`/`DW_AT_abstract_origin` link and qualifying
/// it by its namespace/class ancestry — the C++ shapes the name-only walk drops.
///
/// Fields mirror the same-named [`AnalysisOutput`] streams; when the gate is on
/// the commit folds `symbols`/`locals` into those streams and applies
/// `prototypes` through the address-keyed binding below.
#[derive(Default, Debug)]
pub struct CppDwarfFacts {
    /// Function symbols for definition DIEs whose name lives on the linked
    /// declaration (an out-of-line member/namespace definition), carrying the
    /// namespace-qualified name (`Account::deposit`).
    pub symbols: Vec<SymFact>,
    /// Named, typed `DW_OP_fbreg` stack locals of those same definitions.
    pub locals: Vec<LocalFact>,
    /// Prototypes keyed by the subprogram's ENTRY ADDRESS (`DW_AT_low_pc`) rather
    /// than by name. Address is what the parked-prototype store is keyed by on the
    /// read side, and it is the only key that survives C++: a demangled template
    /// name (`maxof<int>`) is normalized to `maxof` in kuna's symbol table, and a
    /// qualified name (`Account::deposit`) lives in a nested scope the global
    /// by-name query never reaches.
    pub prototypes: Vec<(u64, kuna_decomp::fspec::PrototypePieces)>,
}

impl AnalysisOutput {
    /// Fold another output into this one (concatenation; dedup is the driver's job).
    pub fn merge(&mut self, other: AnalysisOutput) {
        self.symbols.extend(other.symbols);
        self.data_objects.extend(other.data_objects);
        self.entries.extend(other.entries);
        self.fde_bodies.extend(other.fde_bodies);
        self.entry_names.extend(other.entry_names);
        self.noreturn.extend(other.noreturn);
        self.no_fallthru_calls.extend(other.no_fallthru_calls);
        self.readonly.extend(other.readonly);
        self.externref.extend(other.externref);
        self.strings.extend(other.strings);
        self.prototypes.extend(other.prototypes);
        self.context_paints.extend(other.context_paints);
        self.tracked_regs.extend(other.tracked_regs);
        self.call_fixups.extend(other.call_fixups);
        self.locals.extend(other.locals);
        self.comments.extend(other.comments);
        self.fid_names.extend(other.fid_names);
        self.cpp_dwarf.symbols.extend(other.cpp_dwarf.symbols);
        self.cpp_dwarf.locals.extend(other.cpp_dwarf.locals);
        self.cpp_dwarf.prototypes.extend(other.cpp_dwarf.prototypes);
        self.cpp_sig.proven.extend(other.cpp_sig.proven);
        self.cpp_sig.inferred.extend(other.cpp_sig.inferred);
    }
}

/// Read-only inputs an analysis sees: the parsed object, the loaded image
/// (vma -> bytes), and the resolved architecture (endianness/pointer size).
pub struct AnalysisCtx<'a> {
    /// The parsed object file (read-only `object` crate view).
    pub file: &'a object::File<'a>,
    /// The raw image bytes `file` was parsed from. Threaded to the import boundary
    /// (`format::resolve_imports`), which some formats (PE/Mach-O) re-parse with a
    /// typed parser; the ELF impl ignores it.
    pub bytes: &'a [u8],
    /// The loaded image backing byte reads at a vma.
    pub image: &'a ObjectLoadImage,
    /// The engine architecture (read access to types/spaces/options).
    pub arch: &'a Architecture,
    /// The optional Listing/xref disassembly model (`--option listing on`,
    /// default-off). `None` unless the Listing tier is enabled and built at load
    /// (real-ELF path only). PR1 always leaves this `None`; PR2 builds it.
    pub listing: Option<&'a Listing>,
}

/// One program-prep analysis. Implementors mirror the `elf_plt` contract: pure
/// over the context, additive, never failing.
pub trait AnalysisPass {
    /// The phase this pass feeds (for ordering + `phase map` registry parity).
    fn phase(&self) -> Phase;
    /// Stable id used to gate the pass on/off as a settable assertion
    /// (registered in `phases.toml`, flippable via `--option <id> on|off`).
    fn id(&self) -> &'static str;
    /// Gather facts. Pure over `ctx`; produces output, performs no engine mutation.
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput;
}

/// Run every enabled pass over the same context and merge their outputs.
///
/// The merged [`AnalysisOutput`] is then committed into the engine in one place
/// (the bootstrap, `kuna-console`'s `engine::bootstrap_from_object`). Committing is
/// added alongside the first pass that seeds the symbol/type tables; today the
/// PLT/GOT path still commits inline in [`crate::loadimage_object`].
pub fn run_analyses(ctx: &AnalysisCtx, passes: &[Box<dyn AnalysisPass>]) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    for pass in passes {
        out.merge(pass.run(ctx));
    }
    out
}
