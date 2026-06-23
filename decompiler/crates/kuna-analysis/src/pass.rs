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

/// The facts one analysis contributes. Every field is additive and may be empty;
/// merging two outputs is concatenation (the driver dedups by address).
#[derive(Default, Debug)]
pub struct AnalysisOutput {
    /// `(addr, name, kind)` -> seed `Database::add_function` / symbol map.
    pub symbols: Vec<SymFact>,
    /// Discovered function entry points (for stripped targets).
    pub entries: Vec<u64>,
    /// Functions that do not return (`exit`, `abort`, ...), by name.
    pub noreturn: Vec<String>,
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
