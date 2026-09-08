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
pub mod kuna_win32sigs;

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
    /// `unsigned int *` (`LPDWORD`, `PDWORD`).
    UIntPtr,
    /// `wchar_t *` (`LPWSTR` / `LPCWSTR`), at the compiler spec's `wchar_size`.
    WCharPtr,
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
    // sys/ptrace.h — `long ptrace(int request, pid_t pid, void *addr, void *data)`.
    // glibc DECLARES it variadic (`long ptrace(enum __ptrace_request, ...)`), so no
    // body analysis can derive the arity; the four fixed slots are glibc's own
    // (`sysdeps/unix/sysv/linux/ptrace.c` fetches `pid_t`, `void *`, `void *` after
    // the request with `va_arg`), which is also the call form ptrace(2) documents.
    ("ptrace", Sig { ret: Ty::Long, params: &[Ty::Int, Ty::Int, Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
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
        Ty::UIntPtr => {
            let u = types.get_base(4, type_metatype::TYPE_UINT)?;
            types.get_type_pointer(ptr, u, word_size)
        }
        Ty::WCharPtr => {
            let w = types.get_type_char(types.get_size_of_wchar())?;
            types.get_type_pointer(ptr, w, word_size)
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

/// (kuna `declaredlibcproto`) The built-in signature for a function name the
/// OPERATOR declared, or `None` when neither table knows the name.
///
/// The two load-time passes match a name the *image* carries: [`LibProtoPass`]
/// over the object's own FUNC symbols and imports, [`kuna_libcsigs::LibcSigsPass`]
/// over the imports alone. Neither can answer for a name that exists only because
/// a caller said so (`--define-function 0x8048968=ptrace` on a stripped, statically
/// linked image), which is precisely the reverse-engineering case: the symbol table
/// is gone, the operator has identified the callee, and the arity is still unknown.
///
/// Both tables are searched here, the imports-only restriction included. That
/// restriction exists because a *coincidental* spelling must not retype a function
/// the image defines itself — a judgement about evidence, and the evidence is
/// different when a human or an agent has named the entry outright.
pub fn declared_libc_prototype(
    name: &str,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> Option<PrototypePieces> {
    let sig = LIBC
        .iter()
        .chain(kuna_libcsigs::LIBC_EXT.iter())
        .find(|(n, _)| *n == name)
        .map(|(_, sig)| sig)?;
    build_pieces(name, sig, types, word_size).ok()
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
    fn ptrace_is_the_four_slot_documented_call_form() {
        // glibc DECLARES `long ptrace(enum __ptrace_request, ...)`, so a stripped
        // static image gives argument recovery nothing to work from. The four
        // fixed slots are the ones glibc's own wrapper fetches with `va_arg`.
        let (_, sig) = LIBC.iter().find(|(n, _)| *n == "ptrace").expect("table knows ptrace");
        assert!(matches!(sig.ret, Ty::Long), "ptrace returns long");
        assert_eq!(sig.params.len(), 4, "request, pid, addr, data");
        assert!(matches!(sig.params[0], Ty::Int));
        assert!(matches!(sig.params[1], Ty::Int));
        assert!(matches!(sig.params[2], Ty::VoidPtr));
        assert!(matches!(sig.params[3], Ty::VoidPtr));
        assert_eq!(sig.vararg, -1, "the fixed form is what a caller is typed against");
    }

    #[test]
    fn a_declared_name_is_answered_out_of_either_table() {
        // The declared-name lookup is what a stripped image has instead of a symbol
        // table, so it must reach BOTH tables: the imports-only restriction on the
        // measured extension is about a coincidental spelling, and an operator who
        // named the entry outright is not a coincidence.
        let types = kuna_decomp::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(4), 4, 4);
        let _ = types.cache_core_types();
        let base = declared_libc_prototype("ptrace", &types, 1).expect("base table name");
        assert_eq!(base.name, "ptrace");
        assert_eq!(base.intypes.len(), 4);
        let ext = declared_libc_prototype("read", &types, 1).expect("extension table name");
        assert_eq!(ext.intypes.len(), 3, "ssize_t read(int, void *, size_t)");
        assert!(
            declared_libc_prototype("sub_8049027", &types, 1).is_none(),
            "a name neither table knows is left alone"
        );
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
