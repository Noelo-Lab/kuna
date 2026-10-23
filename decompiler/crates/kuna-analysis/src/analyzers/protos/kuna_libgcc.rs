//! Compiler-runtime contracts whose integer widths come from machine modes,
//! not the host C data model. Applied under the existing `libproto` option.
//!
//! `__divdi3` is `di_int (di_int, di_int)`, with `di_int = int64_t`:
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/divdi3.c
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/int_types.h

use std::collections::HashSet;

use object::{Architecture, BinaryFormat, Object};

use super::{seed_named_prototypes, seed_resolved_prototypes, Sig, Ty, L};
use crate::pass::{AnalysisCtx, AnalysisOutput};

const SIGNATURES: &[(&str, Sig)] = &[
    ("__divdi3", Sig { ret: Ty::Int64, params: &[Ty::Int64, Ty::Int64], vararg: -1 }),
];

/// Seed the evidenced SPARC32 ELF runtime ABI without retyping other targets.
pub(super) fn seed(
    ctx: &AnalysisCtx,
    out: &mut AnalysisOutput,
    names: &HashSet<String>,
    imports: &[(String, u64)],
) {
    if ctx.file.format() != BinaryFormat::Elf
        || !matches!(ctx.file.architecture(), Architecture::Sparc | Architecture::Sparc32Plus)
        || ctx.arch.get_description() != "sparc:BE:32:default:default"
    {
        return;
    }
    let types = ctx.arch.types();
    let (_, word_size) = ctx.arch.data_org();
    seed_named_prototypes(out, names, SIGNATURES, types, word_size, L);
    seed_resolved_prototypes(out, imports, SIGNATURES, types, word_size, L);
}
