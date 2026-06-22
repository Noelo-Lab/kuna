//! The default program-prep pass list + the driver entry point the bootstrap
//! seam calls.
//!
//! [`default_passes`] is the registry of enabled analyses, in stage order. Each
//! pass's [`AnalysisPass::id`](crate::pass::AnalysisPass::id) is its gate name;
//! the passes here are **bound to the real-ELF bootstrap path only** (the XML
//! `<binaryimage>` datatest path never calls [`run_default_analyses`]), so the
//! 675/675 datatest parity oracle is structurally untouched regardless of what
//! is enabled — see the downstream-compatibility note in
//! `docs/missing-analyses.md`.

use kuna_decomp::architecture::Architecture;

use crate::loadimage_object::ObjectLoadImage;
use crate::pass::{run_analyses, AnalysisCtx, AnalysisOutput, AnalysisPass};

/// The default program-prep passes, in stage order.
///
/// These run at load time over the parsed ELF and produce an additive
/// [`AnalysisOutput`] the console commits into the engine. Adding a new analysis
/// is: implement [`AnalysisPass`] in an `s1_*` module, then add it here.
pub fn default_passes() -> Vec<Box<dyn AnalysisPass>> {
    vec![
        // S1 loader: known no-return functions (exit/abort/…). Mirrors Ghidra's
        // default-on `NoReturnFunctionAnalyzer`.
        Box::new(crate::s1_loader::noreturn::NoReturnKnownPass),
        // S1 strings: NUL-terminated ASCII string-literal detection. Mirrors
        // Ghidra's `StringsAnalyzer` (min length 5, require-NUL-end).
        // S1 library prototypes: seed common libc signatures (puts(char*), …) so
        // call arguments get typed. Mirrors Ghidra's `ApplyDataArchiveAnalyzer`.
        // THIS is what renders string literals in kuna: typing a call argument
        // `char *` lets the printer's pointer-char-constant path read the readonly
        // bytes via the StringManager and emit `puts("Username: ")`.
        Box::new(crate::s1_protos::LibProtoPass),
        // S1 strings (StringLiteralPass) is implemented + tested but **disabled by
        // default**: kuna's printer renders a constant that maps to a *named* global
        // symbol as that symbol's NAME (`s_400915`), which SHADOWS the string-literal
        // path. So planting a `char[N]` data symbol (Ghidra's StringsAnalyzer
        // mechanism) actually *blocks* `puts("Username: ")` in kuna, where the literal
        // instead comes from type-driven rendering (libproto/usage `char *` typing +
        // readonly + StringManager). Re-enabling it requires teaching the printer to
        // render a pointer to a readonly char-array symbol as the literal (the Ghidra
        // behavior) — a deferred printer change. See docs/analysis-port-log.md.
        // Box::new(crate::s1_strings::StringLiteralPass { min_len: 5 }),
    ]
}

/// Parse `bytes` as an object file, build an [`AnalysisCtx`], and run every
/// [`default_passes`] pass, returning the merged [`AnalysisOutput`].
///
/// This keeps `object::File` an internal concern of `kuna-analysis` (the crate
/// that already depends on `object`): the console hands in the image bytes, the
/// opened [`ObjectLoadImage`], and the resolved [`Architecture`], and gets back
/// flat facts to commit. A parse failure yields an empty output (the analyses
/// are additive and never fail — the funcsym stream is committed separately).
pub fn run_default_analyses(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
) -> AnalysisOutput {
    let Ok(file) = object::File::parse(bytes) else {
        return AnalysisOutput::default();
    };
    let ctx = AnalysisCtx { file: &file, image, arch };
    run_analyses(&ctx, &default_passes())
}
