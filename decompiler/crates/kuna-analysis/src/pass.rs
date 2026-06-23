//! The program-prep analysis-pass interface.
//!
//! Generalizes the de-facto contract already used by [`crate::s1_loader::elf_plt`]
//! (`resolve_plt_imports(&object::File) -> Vec<PltSym>`): a focused module that
//! reads the parsed object and produces a flat, deduplicated set of *facts*,
//! never panicking and never failing -- it only ever contributes *more*
//! knowledge (names, types, entries). A driver merges the facts from all enabled
//! passes and commits them once into the engine's symbol/type tables.
//!
//! This is the seam every new analysis (string detection, demangling, DWARF,
//! function-start discovery, library prototypes, ...) plugs into; see
//! `docs/missing-analyses.md` for the roadmap. The concrete passes will land as
//! `s1_*` modules implementing [`AnalysisPass`].

use kuna_decomp::architecture::Architecture;

use crate::loadimage_object::ObjectLoadImage;

/// The stage a pass feeds, mirroring the kuna stage model (`docs/stages.md`).
/// Program-prep analyses are P0/S1; a few (e.g. jump-table post-typing
/// refinement) feed back into S2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// P0 -- knowledge & configuration plane (symbol DB facts).
    P0,
    /// S1 -- image & code partition (the loader/analyzer tier).
    S1,
    /// S2 -- flow & op-graph recovery (post-typing feedback).
    S2,
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

/// A detected NUL-terminated string literal: a `char[len]` data object at `addr`.
///
/// `len` is the byte length **including** the terminating NUL (visible chars + 1),
/// matching what `DataUtilities.createData` / a `char[N]` array should span — the
/// length the commit seam passes to `get_type_array` so the printer renders the
/// literal.  Produced by [`crate::s1_strings`] (the kuna analog of Ghidra's
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
/// the explicit `[addr, end)` range). Produced by [`crate::s1_loader::arm_markers`]
/// (the kuna analog of ARM's `ARM_ElfExtension`/`ArmSymbolAnalyzer` `TMode`
/// painting) and applied by the console's `commit_analysis_output` via the
/// engine's `ContextDatabase` (`set_variable` / `set_variable_region`, the exact
/// analog of Ghidra's `programContext.setValue(TMode, …)`).
///
/// `var` is a `&'static str` because the only producer paints SLEIGH-defined
/// context registers known at compile time (`"TMode"`); the commit seam swallows
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
/// FunctionSymbol no-return. Produced by [`crate::s1_loader::noreturn`] (the kuna
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
/// instead of `local_1c`. Produced by [`crate::s1_dwarf`] (subtask 3 — the kuna
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
/// offset; the commit seam wraps it to the stack space's unsigned address.
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

/// A tracked register **value** to seed at a function entry: at `func_addr`, the
/// register named `reg` holds the constant `value`. Distinct from
/// [`ContextPaint`] (which steers SLEIGH instruction *decode* via a context BIT):
/// this seeds a register-VALUE the decompiler's constant analysis propagates (the
/// kuna analog of Ghidra's `ProgramContext.setRegisterValue` / the console
/// `set track <reg> <val> <start> <end>`). Produced by
/// [`crate::s1_loader::mips_markers`] (the kuna analog of Ghidra's
/// `MipsAddressAnalyzer`), which seeds `t9 = func_addr` per function — the MIPS PIC
/// ABI convention (a function is reached via `jalr t9`, so on entry `t9` holds the
/// callee entry). That lets a PIC prologue's `addu gp,gp,t9` fold to the real `$gp`,
/// so `$gp`-relative GOT/`.sdata` loads resolve to concrete addresses.
///
/// The commit seam (`engine.rs::commit_analysis_output`) resolves `reg` to its
/// register varnode and calls `create_set(func_addr, func_addr+1)` + pushes a
/// `TrackedContext{loc, val}` — the exact `IfcSettrackedrange` recipe. The
/// per-function `build_arch_handle` then snapshots the track base into the seam,
/// and `ActionConstbase` (S3) emits `COPY #val -> reg` at the entry block, feeding
/// constant propagation. `reg` is a `&'static str` because the only producer names
/// SLEIGH registers known at compile time (`"t9"`); the commit seam swallows a
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
/// Produced by [`crate::s1_callfixup`] (the kuna analog of Ghidra's
/// `CallFixupAnalyzer`).
///
/// Carries the **original installed name** (not the matched fixup name), like
/// [`AnalysisOutput::noreturn`] does: the commit seam resolves the real
/// FunctionSymbol via `query_global_function(func_name)` and re-derives the fixup
/// the same way the pass did. A name that no longer resolves (or already carries a
/// fixup) is the faithful no-op (Ghidra only sets when `getCallFixup()==null`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallFixupFact {
    /// The matched function's installed name (e.g. `mcount`, `__fentry__`).
    pub func_name: String,
}

/// The facts one analysis contributes. Every field is additive and may be empty;
/// merging two outputs is concatenation (the driver dedups by address).
#[derive(Default, Debug)]
pub struct AnalysisOutput {
    /// `(addr, name, kind)` -> seed `Database::add_function` / symbol map.
    pub symbols: Vec<SymFact>,
    /// Discovered function entry points (for stripped targets).
    pub entries: Vec<u64>,
    /// Functions that do not return (`exit`, `abort`, ...), by `(addr, name)`.
    /// The commit resolves each by ADDRESS (stable across demangling), falling
    /// back to the name path. See [`NoReturnFact`].
    pub noreturn: Vec<NoReturnFact>,
    /// Extra read-only address ranges (e.g. `.got` after relocation).
    pub readonly: Vec<(u64, u64)>,
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
    /// Each sets a SLEIGH context variable over an address range; the commit seam
    /// applies them to the engine's `ContextDatabase` BEFORE any instruction is
    /// decoded, steering ARM/Thumb instruction decode. Produced only on the ARM
    /// path (see [`crate::s1_loader::arm_markers`]); empty otherwise.
    pub context_paints: Vec<ContextPaint>,
    /// Tracked register-values to seed at function entries (the kuna analog of
    /// Ghidra's `MipsAddressAnalyzer` `ProgramContext.setRegisterValue` / the
    /// console `set track <reg> <val> <start> <end>`). The commit seam resolves
    /// each register and seeds the value over `[func_addr, func_addr+1)`; the
    /// per-function `build_arch_handle` snapshots it and `ActionConstbase` emits
    /// `COPY #val -> reg` at the entry, which constant propagation consumes (so a
    /// MIPS PIC `$gp`-relative load resolves). Produced only on the MIPS path (see
    /// [`crate::s1_loader::mips_markers`]); empty otherwise.
    pub tracked_regs: Vec<TrackedRegFact>,
    /// Functions whose names matched a cspec call-fixup `<target>` (e.g. the `-pg`
    /// `mcount`/`__fentry__` profiling stubs). The commit seam tags each with the
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
}

impl AnalysisOutput {
    /// Fold another output into this one (concatenation; dedup is the driver's job).
    pub fn merge(&mut self, other: AnalysisOutput) {
        self.symbols.extend(other.symbols);
        self.entries.extend(other.entries);
        self.noreturn.extend(other.noreturn);
        self.readonly.extend(other.readonly);
        self.strings.extend(other.strings);
        self.prototypes.extend(other.prototypes);
        self.context_paints.extend(other.context_paints);
        self.tracked_regs.extend(other.tracked_regs);
        self.call_fixups.extend(other.call_fixups);
        self.locals.extend(other.locals);
    }
}

/// Read-only inputs an analysis sees: the parsed object, the loaded image
/// (vma -> bytes), and the resolved architecture (endianness/pointer size).
pub struct AnalysisCtx<'a> {
    /// The parsed object file (read-only `object` crate view).
    pub file: &'a object::File<'a>,
    /// The loaded image backing byte reads at a vma.
    pub image: &'a ObjectLoadImage,
    /// The engine architecture (read access to types/spaces/options).
    pub arch: &'a Architecture,
}

/// One program-prep analysis. Implementors mirror the `elf_plt` contract: pure
/// over the context, additive, never failing.
pub trait AnalysisPass {
    /// The stage this pass feeds (for ordering + `stage map` registry parity).
    fn stage(&self) -> Stage;
    /// Stable id used to gate the pass on/off as a settable assertion
    /// (registered in `stages.toml`, flippable via `--option <id> on|off`).
    fn id(&self) -> &'static str;
    /// Gather facts. Pure over `ctx`; produces output, performs no engine mutation.
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput;
}

/// Run every enabled pass over the same context and merge their outputs.
///
/// The merged [`AnalysisOutput`] is then committed into the engine in one place
/// (the bootstrap, `kuna-console`'s `engine::bootstrap_from_elf`). Committing is
/// added alongside the first pass that seeds the symbol/type tables; today the
/// PLT/GOT path still commits inline in [`crate::loadimage_object`].
pub fn run_analyses(ctx: &AnalysisCtx, passes: &[Box<dyn AnalysisPass>]) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    for pass in passes {
        out.merge(pass.run(ctx));
    }
    out
}
