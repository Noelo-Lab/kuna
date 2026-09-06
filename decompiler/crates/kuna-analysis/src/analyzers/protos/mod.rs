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

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

pub mod kuna_libcsigs;

/// Port of `ApplyDataArchiveAnalyzer`: seed built-in libc prototypes onto matching
/// FunctionSymbols so call arguments get typed.
pub struct LibProtoPass;

/// A primitive type slot in a built-in libc signature.
///
/// Every variant is **width-stable**: it is either `void`, exactly 4 bytes, or
/// exactly pointer-width on every ILP32/LP64 target. A C type that is neither
/// (`off_t`, `time_t`, `long long`, `char`/`short` parameters) has no spelling
/// here on purpose — see [`kuna_libcsigs`].
#[derive(Clone, Copy)]
enum Ty {
    /// `void` (return only).
    Void,
    /// `int` (4-byte signed).
    Int,
    /// `unsigned int` / `mode_t` / `uid_t` / `wint_t` (4-byte unsigned).
    UInt,
    /// `size_t` (pointer-width unsigned).
    Size,
    /// `ssize_t` / `long` / `ptrdiff_t` (pointer-width signed).
    Long,
    /// `char *`.
    CharPtr,
    /// `char **`.
    CharPtrPtr,
    /// `int *`.
    IntPtr,
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
    // locale.h — `char *setlocale(int category, const char *locale)`.  Without
    // this prototype the call's result is an untyped `undefined8`, so a wrapper
    // whose last act is `return setlocale(cat, NULL);` (e.g. gnulib's
    // `setlocale_null_androidfix`, a tail call at -O2) loses both the recovered
    // return value and the `char *` type.  See docs/features/setlocale-rettype/.
    ("setlocale", Sig { ret: Ty::CharPtr, params: &[Ty::Int, Ty::CharPtr], vararg: -1 }),
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
        Ty::UInt => types.get_base(4, type_metatype::TYPE_UINT),
        Ty::Size => types.get_base(ptr, type_metatype::TYPE_UINT),
        Ty::Long => types.get_base(ptr, type_metatype::TYPE_INT),
        Ty::CharPtr => {
            let c = types.get_type_char(types.get_size_of_char())?;
            types.get_type_pointer(ptr, c, word_size)
        }
        Ty::CharPtrPtr => {
            let c = types.get_type_char(types.get_size_of_char())?;
            let cp = types.get_type_pointer(ptr, c, word_size)?;
            types.get_type_pointer(ptr, cp, word_size)
        }
        Ty::IntPtr => {
            let i = types.get_base(4, type_metatype::TYPE_INT)?;
            types.get_type_pointer(ptr, i, word_size)
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
        input_storage: Vec::new(),
    })
}

/// Collect the set of FUNC symbol names present in the object — the names the
/// prototype table is matched against. Two format-neutral sources, unioned:
///
/// 1. defined/declared FUNC symbols (`.symtab` + `.dynsym` on ELF; the COFF
///    symtab on PE/COFF; `LC_SYMTAB` on Mach-O), `@VERSION` stripped;
/// 2. the §3 import resolver (`resolve_imports`): PE IAT/INT, Mach-O `__stubs`.
///    This is the source that matters on a **stripped** PE (no symtab `puts`)
///    and on Mach-O (the import `printf` is named by the `__stubs` walk, not a
///    `SymbolKind::Text` entry) — `ApplyDataArchiveAnalyzer` matches archive
///    entries to the program's *functions*, which on these formats include the
///    resolved imports.
///
/// libc/msvcrt names are unmangled, so demangling is a no-op here.
fn present_function_names(file: &object::File, bytes: &[u8]) -> std::collections::HashSet<String> {
    let mut present = std::collections::HashSet::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        if let Ok(n) = sym.name() {
            if let Ok(n) = String::from_utf8(crate::loader::elf_plt::strip_version(n.as_bytes()))
            {
                present.insert(n);
            }
        }
    }
    // The resolved imports (PE IAT, Mach-O __stubs). On ELF this overlaps the
    // `.dynsym` set already collected (`elf_plt` names the PLT stub by the same
    // `.dynstr` name), so the union is a no-op there — ELF behavior unchanged.
    for imp in crate::loader::format::resolve_imports(file, bytes) {
        if let Ok(n) = String::from_utf8(imp.name) {
            present.insert(n);
        }
    }
    present
}

impl AnalysisPass for LibProtoPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "libproto"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        // Format-agnostic (PR-10): the libc/msvcrt name match reads neutral data
        // (`present_function_names` unions the FUNC symbols with the §3 import
        // resolver's names), so it fires on ELF/PE/COFF/Mach-O alike — no format
        // branch. On a PE a `printf` import then types its first arg `char *`, so
        // `printf("%d\n", …)` renders the literal instead of `printf(0x…, …)`.
        let mut out = AnalysisOutput::default();
        let present = present_function_names(ctx.file, ctx.bytes);
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
    fn setlocale_signature_is_char_ptr_int_char_ptr() {
        // `char *setlocale(int category, const char *locale)`.  Curating this
        // entry is the fix for the `-O2` setlocale wrapper (gnulib
        // `setlocale_null_androidfix`): without it the call's result is an
        // untyped `undefined8`, so the wrapper's signature comes out `void`
        // instead of `char *` and the return value is lost.  Pin the shape so a
        // future edit cannot silently demote the return type back to `int`/`void`.
        let entry = LIBC.iter().find(|(n, _)| *n == "setlocale");
        let (_, sig) = entry.expect("table must know setlocale");
        assert!(matches!(sig.ret, Ty::CharPtr), "setlocale returns char *");
        assert_eq!(sig.params.len(), 2, "setlocale takes (int, const char *)");
        assert!(matches!(sig.params[0], Ty::Int), "category is int");
        assert!(matches!(sig.params[1], Ty::CharPtr), "locale is const char *");
        assert_eq!(sig.vararg, -1, "setlocale is not variadic");
    }

    #[test]
    fn fauxware_seeds_puts_printf_strcmp() {
        // The fixture's imports (puts/printf/strcmp/read/open) are present; the pass
        // must emit prototypes for the libc names it knows that are present, and
        // none for names absent from the table or the binary.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let present = present_function_names(&file, &bytes);
        for want in ["puts", "printf", "strcmp"] {
            assert!(present.contains(want), "fauxware should import {want}");
            assert!(LIBC.iter().any(|(n, _)| n == &want), "table should know {want}");
        }
    }

    #[test]
    fn pe_present_names_include_imports_for_proto_typing() {
        // PR-10: on a PE the libc imports must be in `present_function_names` (so
        // their prototypes seed and the call args type `char *`). In the linked
        // MinGW PE `puts`/`printf` are in the COFF symtab; the resolver also names
        // them via the IAT — either way they are present, so `printf("%d\n", …)`
        // can render the literal.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pe_imports.exe");
        let bytes = std::fs::read(path).expect("read pe_imports.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse pe_imports.exe");
        assert_eq!(file.format(), object::BinaryFormat::Pe, "fixture is a PE");
        let present = present_function_names(&file, &bytes);
        for want in ["puts", "printf"] {
            assert!(present.contains(want), "PE present-names must include {want}: {present:?}");
            assert!(LIBC.iter().any(|(n, _)| n == &want), "table should know {want}");
        }
    }

    #[test]
    fn stripped_pe_present_names_from_resolver_only() {
        // The IAT-resolver half: in a *stripped* PE there is no COFF symtab `puts`,
        // so the import names come purely from `resolve_imports`. They must still
        // be present so the prototype seeds (the stripped-binary proof).
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pe_imports_stripped.exe");
        let bytes = std::fs::read(path).expect("read pe_imports_stripped.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped PE");
        let present = present_function_names(&file, &bytes);
        assert!(present.contains("puts"), "stripped PE must name `puts` via the IAT: {present:?}");
    }

    #[test]
    fn macho_present_names_include_stub_import() {
        // Mach-O: the `printf` import is named by the `__stubs` indirect-symbol
        // walk (not a `SymbolKind::Text` entry), so the resolver-union is what
        // makes it present for prototype typing.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_imports");
        let bytes = std::fs::read(path).expect("read macho_imports");
        let file = object::File::parse(bytes.as_slice()).expect("parse macho_imports");
        assert_eq!(file.format(), object::BinaryFormat::MachO, "fixture is Mach-O");
        let present = present_function_names(&file, &bytes);
        assert!(present.contains("printf"), "Mach-O must name `printf` via __stubs: {present:?}");
    }
}
