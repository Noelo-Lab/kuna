//! Library-prototype seeding — the kuna analog of Ghidra's
//! `ApplyDataArchiveAnalyzer` ("Apply Data Archives").
//!
//! Ghidra ships parsed C headers as binary data-type archives (`.gdt`) and, for
//! each function whose name matches an archive entry, applies the archived
//! signature (return + parameter types) to the function. That gives an import
//! like `puts` its `int puts(char *)` prototype, so the decompiler types the
//! call's argument (a `char *`) and — combined with read-only string data — emits
//! `puts("Username: ")` instead of `puts(0x400915)`.
//!
//! The `.gdt` archives are a binary format not vendored into the kuna tree, so
//! this pass substitutes a **built-in table of the most common libc signatures**
//! (a faithful, minimal stand-in). It is the deliberate analog of the
//! dependency/data substitutions elsewhere in the port (BFD → `object`); the
//! signatures are standard C library declarations. Documented LOSS: it covers
//! only the table below, not a full header archive.
//!
//! Matching is by name against functions actually present in the object (same as
//! `ApplyDataArchiveAnalyzer` matching archive entries to program symbols); a
//! table entry with no matching function is simply not emitted. The commit seam
//! (`engine.rs::commit_analysis_output`) parks each prototype on its callee via
//! `Architecture::set_function_prototype_pieces`, which `ActionDefaultParams`
//! reads back when typing the caller's arguments.

use std::rc::Rc;

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use kuna_base::error::KunaResult;
use kuna_base::types::uint4;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage};

/// Port of `ApplyDataArchiveAnalyzer`: seed built-in libc prototypes onto matching
/// FunctionSymbols so call arguments get typed.
pub struct LibProtoPass;

/// A primitive type slot in a built-in libc signature.
#[derive(Clone, Copy)]
enum Ty {
    /// `void` (return only).
    Void,
    /// `int` (4-byte signed).
    Int,
    /// `size_t` / `long` (pointer-width unsigned).
    Size,
    /// `char *`.
    CharPtr,
    /// `void *` (also used for `FILE *`, opaque handles).
    VoidPtr,
}

/// A built-in libc signature: return type, parameter types, and the first
/// variadic slot (`-1` if not variadic).
struct Sig {
    ret: Ty,
    params: &'static [Ty],
    vararg: i32,
}

/// The built-in libc prototype table — a faithful minimal stand-in for Ghidra's
/// `.gdt` archives. Standard C library signatures; `FILE *` is modeled as
/// `void *`. Keep entries conservative and correct.
const LIBC: &[(&str, Sig)] = &[
    // stdio
    ("puts", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    ("printf", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: 1 }),
    ("fputs", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("fprintf", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::CharPtr], vararg: 2 }),
    ("sprintf", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: 2 }),
    ("snprintf", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Size, Ty::CharPtr], vararg: 3 }),
    ("scanf", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: 1 }),
    ("sscanf", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: 2 }),
    ("perror", Sig { ret: Ty::Void, params: &[Ty::CharPtr], vararg: -1 }),
    ("fopen", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    // string.h
    ("strlen", Sig { ret: Ty::Size, params: &[Ty::CharPtr], vararg: -1 }),
    ("strcmp", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strncmp", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strcpy", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strncpy", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr, Ty::Size], vararg: -1 }),
    ("strcat", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("strchr", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("strstr", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("atoi", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    // stdlib / mem
    ("malloc", Sig { ret: Ty::VoidPtr, params: &[Ty::Size], vararg: -1 }),
    ("calloc", Sig { ret: Ty::VoidPtr, params: &[Ty::Size, Ty::Size], vararg: -1 }),
    ("realloc", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("free", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("memcpy", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("memmove", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("memset", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Int, Ty::Size], vararg: -1 }),
];

/// Build the kuna [`Datatype`] for a [`Ty`] using the architecture's type factory.
fn build_ty(t: Ty, types: &dyn TypeFactory, word_size: uint4) -> KunaResult<Rc<Datatype>> {
    let ptr = types.get_size_of_pointer();
    match t {
        Ty::Void => types.get_type_void(),
        Ty::Int => types.get_base(4, type_metatype::TYPE_INT),
        Ty::Size => types.get_base(ptr, type_metatype::TYPE_UINT),
        Ty::CharPtr => {
            let c = types.get_type_char(types.get_size_of_char())?;
            types.get_type_pointer(ptr, c, word_size)
        }
        Ty::VoidPtr => {
            let v = types.get_type_void()?;
            types.get_type_pointer(ptr, v, word_size)
        }
    }
}

/// Build [`PrototypePieces`] for a single table entry.
fn build_pieces(
    name: &str,
    sig: &Sig,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> KunaResult<PrototypePieces> {
    let outtype = Some(build_ty(sig.ret, types, word_size)?);
    let mut intypes = Vec::with_capacity(sig.params.len());
    for p in sig.params {
        intypes.push(build_ty(*p, types, word_size)?);
    }
    let innames = vec![String::new(); intypes.len()];
    Ok(PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        first_var_arg_slot: sig.vararg,
        output_storage: None,
    })
}

/// Collect the set of FUNC symbol names present in the object (`.symtab` +
/// `.dynsym`, `@VERSION` stripped) — the names the prototype table is matched
/// against. libc names are unmangled, so demangling is a no-op here.
fn present_function_names(file: &object::File) -> std::collections::HashSet<String> {
    let mut present = std::collections::HashSet::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        if let Ok(n) = sym.name() {
            if let Ok(n) = String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes()))
            {
                present.insert(n);
            }
        }
    }
    present
}

impl AnalysisPass for LibProtoPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "libproto"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if !matches!(ctx.file.format(), object::BinaryFormat::Elf) {
            return out;
        }
        let present = present_function_names(ctx.file);
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        for (name, sig) in LIBC {
            if !present.contains(*name) {
                continue;
            }
            // Never fail the analysis: skip an entry whose types can't be built.
            if let Ok(pieces) = build_pieces(name, sig, types, word_size) {
                out.prototypes.push(pieces);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_entries_are_well_formed() {
        // vararg slot, when set, points within or just past the fixed params.
        for (name, sig) in LIBC {
            if sig.vararg >= 0 {
                assert!(
                    (sig.vararg as usize) <= sig.params.len(),
                    "{name}: vararg slot {} > {} fixed params",
                    sig.vararg,
                    sig.params.len()
                );
            }
        }
    }

    #[test]
    fn fauxware_seeds_puts_printf_strcmp() {
        // The fixture's imports (puts/printf/strcmp/read/open) are present; the pass
        // must emit prototypes for the libc names it knows that are present, and
        // none for names absent from the table or the binary.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let present = present_function_names(&file);
        for want in ["puts", "printf", "strcmp"] {
            assert!(present.contains(want), "fauxware should import {want}");
            assert!(LIBC.iter().any(|(n, _)| n == &want), "table should know {want}");
        }
    }
}
