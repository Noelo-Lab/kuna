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
}

impl AnalysisOutput {
    /// Fold another output into this one (concatenation; dedup is the driver's job).
    pub fn merge(&mut self, other: AnalysisOutput) {
        self.symbols.extend(other.symbols);
        self.entries.extend(other.entries);
        self.noreturn.extend(other.noreturn);
        self.readonly.extend(other.readonly);
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
