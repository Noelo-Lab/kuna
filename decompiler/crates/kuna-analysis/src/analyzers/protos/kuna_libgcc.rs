//! (kuna `libgccsigs`) Compiler-runtime contracts whose integer widths come from
//! machine modes, not the host C data model. Off by default: a caller that
//! returns arithmetic on the locked eight-byte quotient still loses a word.
//!
//! `__divdi3` is `di_int (di_int, di_int)`, with `di_int = int64_t`:
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/divdi3.c
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/int_types.h
//!
//! On any ELF, a `__stack_chk_fail_local` the image defines takes the
//! `void (void)` row `libcsigs` gives the import: GCC's `-fPIC` stack protector
//! links the hidden helper into each PIE and shared object, so there it is no
//! import. Keyed by address, and only where no other function name holds that
//! address: static glibc aliases it to `__stack_chk_fail`.

use std::collections::BTreeSet;

use object::{Architecture, BinaryFormat, Object, ObjectSymbol, SymbolKind};

use super::{seed_named_prototypes, seed_resolved_prototypes, Sig, Ty, L};
use crate::loadimage_object::ObjectLoadImage;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

const STACK_CHK_FAIL_LOCAL: &str = "__stack_chk_fail_local";

const SIGNATURES: &[(&str, Sig)] = &[
    ("__divdi3", Sig { ret: Ty::Int64, params: &[Ty::Int64, Ty::Int64], vararg: -1 }),
];

pub struct LibgccSigsPass;

/// The addresses of each `__stack_chk_fail_local` the image defines that no
/// other function name holds.
fn defined_stack_chk_fail_local(ctx: &AnalysisCtx, image: &ObjectLoadImage) -> Vec<(String, u64)> {
    let defines = ctx.file.symbols().chain(ctx.file.dynamic_symbols()).any(|sym| {
        sym.kind() == SymbolKind::Text
            && !sym.is_undefined()
            && sym.name_bytes().is_ok_and(|name| name == STACK_CHK_FAIL_LOCAL.as_bytes())
    });
    if !defines {
        return Vec::new();
    }
    let symbols = image.func_symbols();
    let held: BTreeSet<u64> = symbols
        .iter()
        .chain(&image.func_symbol_aliases())
        .filter(|(_, name)| name != STACK_CHK_FAIL_LOCAL)
        .map(|(addr, _)| *addr)
        .collect();
    symbols
        .into_iter()
        .filter(|(addr, name)| name == STACK_CHK_FAIL_LOCAL && !held.contains(addr))
        .map(|(addr, name)| (name, addr))
        .collect()
}

impl AnalysisPass for LibgccSigsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "libgccsigs"
    }

    /// Seed a defined ELF `__stack_chk_fail_local`, and the evidenced SPARC32 ELF
    /// runtime ABI without retyping other targets.
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != BinaryFormat::Elf {
            return out;
        }
        let image = ctx.image.expect("load-time analysis image");
        let types = ctx.arch.types();
        let (_, word_size) = ctx.arch.data_org();
        let helpers = defined_stack_chk_fail_local(ctx, image);
        seed_resolved_prototypes(&mut out, &helpers, super::kuna_libcsigs::LIBC_EXT, types, word_size, L);
        if !matches!(ctx.file.architecture(), Architecture::Sparc | Architecture::Sparc32Plus)
            || ctx.arch.get_description() != "sparc:BE:32:default:default"
        {
            return out;
        }
        let (imports, reloc_imports) = super::loaded_import_addrs(ctx.file, ctx.bytes, image);
        let names = super::loaded_function_names(ctx.file, ctx.bytes, &reloc_imports, false);
        seed_named_prototypes(&mut out, &names, SIGNATURES, types, word_size, L);
        seed_resolved_prototypes(&mut out, &imports, SIGNATURES, types, word_size, L);
        out
    }
}
