//! (kuna `libgccsigs`) Compiler-runtime contracts whose integer widths come from
//! machine modes, not the host C data model. Off by default: a caller that
//! returns arithmetic on the locked eight-byte quotient still loses a word.
//!
//! `__divdi3` is `di_int (di_int, di_int)`, with `di_int = int64_t`:
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/divdi3.c
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/int_types.h

use object::{Architecture, BinaryFormat, Object};

use super::{seed_named_prototypes, seed_resolved_prototypes, Sig, Ty, L};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

const SIGNATURES: &[(&str, Sig)] = &[
    ("__divdi3", Sig { ret: Ty::Int64, params: &[Ty::Int64, Ty::Int64], vararg: -1 }),
];

pub struct LibgccSigsPass;

impl AnalysisPass for LibgccSigsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "libgccsigs"
    }

    /// Seed the evidenced SPARC32 ELF runtime ABI without retyping other targets.
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != BinaryFormat::Elf
            || !matches!(ctx.file.architecture(), Architecture::Sparc | Architecture::Sparc32Plus)
            || ctx.arch.get_description() != "sparc:BE:32:default:default"
        {
            return out;
        }
        let (imports, reloc_imports) = super::loaded_import_addrs(ctx.file, ctx.bytes, ctx.image);
        let names = super::loaded_function_names(ctx.file, ctx.bytes, &reloc_imports, false);
        let types = ctx.arch.types();
        let (_, word_size) = ctx.arch.data_org();
        seed_named_prototypes(&mut out, &names, SIGNATURES, types, word_size, L);
        seed_resolved_prototypes(&mut out, &imports, SIGNATURES, types, word_size, L);
        out
    }
}
