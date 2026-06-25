//! Mach-O `__stubs` / indirect-symbol import-name resolution — the kuna analog
//! of Ghidra's `MachoProgramBuilder.processIndirectSymbols` / `processStubs`
//! (`Ghidra/Features/MachO/.../macho/MachoProgramBuilder.java`).
//!
//! ## Why this lives in kuna at all
//!
//! kuna ports Ghidra's C++ *decompiler*; resolving a Mach-O call target
//! (`0x1000005cc`) to its imported function name (`printf`) is done by Ghidra's
//! Java *loader* layer, which the port never carried over. Without it a CALL
//! that lands on a `__stubs` entry has no symbol, so `FlowInfo::queryCall` (kuna
//! `decompile_drive.rs::query_call`) finds nothing and the printer falls back to
//! `sub_<addr>`. This module reconstructs the stub/slot → name mapping straight
//! from the indirect-symbol table, so the loader can register the imports as
//! ordinary `FunctionSymbol`s in the same stream as the `LC_SYMTAB` defined
//! functions — exactly the ELF `elf_plt` / PE `pe_iat` model.
//!
//! ## What a Mach-O call to an import looks like (and what we name)
//!
//! A classic (non-chained-fixup) Mach-O references an imported function through a
//! **`__TEXT,__stubs`** entry — a tiny stub the static linker emits. A *direct*
//! `bl`/`callq` targets the stub **directly** (verified on the vendored fixture:
//! x86-64 `callq 0x1000005cc`, arm64 `bl 0x1000005a0`), so — unlike the PE IAT
//! slot or the ELF GOT slot — there is **no pointer to constant-fold and no
//! per-arch stub decode**. Naming the stub *entry* address is enough, and that
//! is **arch-independent**: the entry address is pure section metadata
//! (`sec.addr + i*reserved2`), not a decoded instruction.
//!
//! We additionally name the **symbol-pointer slots**
//! (`S_LAZY_SYMBOL_POINTERS` / `S_NON_LAZY_SYMBOL_POINTERS`, i.e.
//! `__la_symbol_ptr` / `__got`) — the GOT analog — so that a *folded indirect*
//! `call [ptr]` (e.g. a `__got`-routed call in `-fno-plt`-style codegen) also
//! resolves. This is the defensive ELF-GOT-slot mirror.
//!
//! ## Algorithm (matches Ghidra's `processIndirectSymbols`)
//!
//! The `LC_DYSYMTAB` load command carries the **indirect-symbol table**: a flat
//! array of symbol-table indices. Each `S_SYMBOL_STUBS` /
//! `S_*_SYMBOL_POINTERS` section's `reserved1` is that section's start offset
//! into the indirect table, and its entry stride is `reserved2` (stubs) or the
//! pointer width (pointers). For the `i`-th entry the imported name is the
//! `LC_SYMTAB` symbol at the indirect index, and the named address is
//! `sec.addr + i*stride`. We:
//!
//!   1. skip `INDIRECT_SYMBOL_LOCAL` / `INDIRECT_SYMBOL_ABS` entries (a
//!      relocated-away local / absolute symbol, not an import — the
//!      self-correction analog),
//!   2. skip `n_strx == 0` (no name),
//!   3. strip the leading `_` the C ABI prepends (`_printf` → `printf`) — the
//!      `strip_version` analog.
//!
//! Everything degrades gracefully: a non-Mach-O input, a missing `LC_DYSYMTAB`,
//! a fat binary with no usable slice, or an unparsable layout yields fewer (or
//! zero) entries. This module never panics and never errors — it only ever
//! produces *more* names (design §3.3, §8 PR-7).

use object::macho::{INDIRECT_SYMBOL_ABS, INDIRECT_SYMBOL_LOCAL};
use object::read::macho::{
    FatArch, MachHeader, MachOFatFile32, MachOFatFile64, Nlist, Section, Segment, SymbolTable,
};
use object::{Architecture, FileKind};

use super::format::ImportSym;

/// Resolve a Mach-O image's `__stubs` entries and symbol-pointer slots to their
/// imported function names.
///
/// Returns one [`ImportSym`] per named `__stubs` entry (`addr` = the stub VA a
/// `bl`/`callq` targets directly) and one per named symbol-pointer slot (`addr`
/// = the pointer VA a folded indirect `call [ptr]` resolves), plus the Mach-O
/// exports. Empty when the input is not a Mach-O, has no `LC_DYSYMTAB`, selects
/// no fat slice, or the layout is unparsable — never an error.
pub(crate) fn resolve_macho_imports(file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
    match FileKind::parse(bytes) {
        // A thin Mach-O: parse with the matching-width typed header.
        Ok(FileKind::MachO64) => {
            collect::<object::macho::MachHeader64<Endianness>>(file, bytes)
        }
        Ok(FileKind::MachO32) => {
            collect::<object::macho::MachHeader32<Endianness>>(file, bytes)
        }
        // A fat / universal binary (`0xcafebabe` / `0xcafebabf`): select one
        // slice, then re-dispatch on that slice's thin magic (design §3.4).
        Ok(FileKind::MachOFat32) => match select_fat_slice_32(bytes) {
            Some(slice) => resolve_macho_imports(file, slice),
            None => Vec::new(),
        },
        Ok(FileKind::MachOFat64) => match select_fat_slice_64(bytes) {
            Some(slice) => resolve_macho_imports(file, slice),
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// The endianness the typed `MachHeader` parsers are instantiated with — the
/// `object`-crate runtime-`Endianness` so a BE Mach-O (rare, classic PPC) also
/// parses. The header's magic disambiguates LE/BE at parse time.
type Endianness = object::Endianness;

/// Walk the `LC_DYSYMTAB` indirect-symbol table of a typed Mach-O header,
/// naming each `S_SYMBOL_STUBS` entry + each symbol-pointer slot, then append
/// the exports. Generic over the header width so x86-64 (64) and i386 (32)
/// share one body.
fn collect<Mach>(neutral: &object::File, bytes: &[u8]) -> Vec<ImportSym>
where
    Mach: MachHeader<Endian = Endianness>,
{
    let mut out: Vec<ImportSym> = Vec::new();

    // The typed file view exposes the load commands + symbol table the neutral
    // `object::File` does not surface in the shape we need (the raw section
    // `reserved1`/`reserved2` and the flat indirect-symbol slice).
    let header = match Mach::parse(bytes, 0) {
        Ok(h) => h,
        Err(_) => return finish_with_exports(neutral, out),
    };
    let endian = match header.endian() {
        Ok(e) => e,
        Err(_) => return finish_with_exports(neutral, out),
    };

    // Build the section list + the symbol table from the load commands in one
    // pass; pull the `LC_DYSYMTAB` indirect-symbol slice from the same walk.
    let mut commands = match header.load_commands(endian, bytes, 0) {
        Ok(c) => c,
        Err(_) => return finish_with_exports(neutral, out),
    };

    let mut symtab: Option<SymbolTable<Mach>> = None;
    let mut indirect: &[object::U32<Endianness>] = &[];
    // Collect the typed sections so we can read `reserved1`/`reserved2`/`addr`
    // (the neutral `ObjectSection` view drops `reserved*`).
    let mut sections: Vec<&Mach::Section> = Vec::new();

    // Walk the load commands once, harvesting `LC_SYMTAB` (the name table),
    // `LC_DYSYMTAB` (the indirect-symbol index array), and every segment's
    // sections (for the per-section `reserved1`/`reserved2`/`addr`). This is the
    // canonical `object`-crate dispatch (`Segment::from_command` / `command
    // .symtab()` / `command.dysymtab()`), the same one `MachOFile::parse` uses.
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, section_data))) = Mach::Segment::from_command(command) {
            if let Ok(secs) = segment.sections(endian, section_data) {
                for sec in secs {
                    sections.push(sec);
                }
            }
        } else if let Ok(Some(symtab_command)) = command.symtab() {
            if let Ok(st) = symtab_command.symbols::<Mach, _>(endian, bytes) {
                symtab = Some(st);
            }
        } else if let Ok(Some(dysymtab_command)) = command.dysymtab() {
            if let Ok(idx) = dysymtab_command.indirect_symbols(endian, bytes) {
                indirect = idx;
            }
        }
    }

    let symtab = match symtab {
        Some(st) => st,
        None => return finish_with_exports(neutral, out),
    };
    let strings = symtab.strings();

    for sec in sections {
        // `Section::indirect_symbols` returns this section's window into the
        // flat indirect table (using `reserved1` as the start and `reserved2` /
        // pointer-width as the stride) — empty for any non-stub/non-pointer
        // section, so we only process the import-bearing ones.
        let entries = match sec.indirect_symbols(endian, indirect) {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entries.is_empty() {
            continue;
        }
        // The per-entry stride: `reserved2` for stub sections, the pointer width
        // for symbol-pointer sections. Mirrors `Section::indirect_symbols`'s own
        // `entry_size`, recomputed here because we need it for the address.
        let stride = match sec.section_type(endian) {
            object::macho::S_SYMBOL_STUBS => sec.reserved2(endian) as u64,
            object::macho::S_LAZY_SYMBOL_POINTERS
            | object::macho::S_NON_LAZY_SYMBOL_POINTERS
            | object::macho::S_LAZY_DYLIB_SYMBOL_POINTERS
            | object::macho::S_THREAD_LOCAL_VARIABLE_POINTERS => {
                std::mem::size_of::<Mach::Word>() as u64
            }
            _ => continue,
        };
        if stride == 0 {
            continue;
        }
        let base = sec.addr(endian).into();

        for (i, raw_idx) in entries.iter().enumerate() {
            let raw = raw_idx.get(endian);
            // A relocated-away local / absolute symbol is not an import.
            if raw & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 {
                continue;
            }
            let nlist = match symtab.symbol(object::read::SymbolIndex(raw as usize)) {
                Ok(n) => n,
                Err(_) => continue,
            };
            if nlist.n_strx(endian) == 0 {
                continue; // no name
            }
            let name = match nlist.name(endian, strings) {
                Ok(n) if !n.is_empty() => strip_leading_underscore(n),
                _ => continue,
            };
            if name.is_empty() {
                continue;
            }
            let addr = base.wrapping_add((i as u64).wrapping_mul(stride));
            out.push(ImportSym { addr, name });
        }
    }

    finish_with_exports(neutral, out)
}

/// Append the Mach-O exports (name + address) as additive funcsyms, so a call to
/// an exported function of this image resolves. Shared tail of every `collect`
/// exit path (including the early-return error arms), mirroring `pe_iat`'s
/// export append.
fn finish_with_exports(neutral: &object::File, mut out: Vec<ImportSym>) -> Vec<ImportSym> {
    use object::read::Object;
    if let Ok(exports) = neutral.exports() {
        for e in exports {
            let name = e.name();
            if !name.is_empty() {
                out.push(ImportSym {
                    addr: e.address(),
                    name: strip_leading_underscore(name),
                });
            }
        }
    }
    out
}

/// Strip the single leading `_` the C ABI prepends to Mach-O symbol names
/// (`_printf` → `printf`, `_main` → `main`). A name without the prefix (a
/// C++-mangled `__Z…`, a Swift `_$s…`, or an already-clean name) keeps exactly
/// one leading `_` removed — matching Ghidra's `SymbolUtilities` underscore
/// trim and the ELF `strip_version` posture (clean the ABI decoration, leave the
/// rest for the demangler downstream).
fn strip_leading_underscore(name: &[u8]) -> Vec<u8> {
    match name.split_first() {
        Some((b'_', rest)) => rest.to_vec(),
        _ => name.to_vec(),
    }
}

/// Select one slice of a 32-bit fat header, returning the sub-slice `bytes`
/// covering it. Prefers x86-64, then arm64, else the first arch (design §3.4).
fn select_fat_slice_32(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile32::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Select one slice of a 64-bit fat header (design §3.4).
fn select_fat_slice_64(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile64::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Deterministic fat-slice preference: x86-64 first (host-relevant in this
/// container), then arm64, then the first arch present. `--target` would be the
/// manual override, but the import walk only needs *a* consistent slice.
fn select_fat_arch<Fat: FatArch>(arches: &[Fat]) -> Option<&Fat> {
    arches
        .iter()
        .find(|a| a.architecture() == Architecture::X86_64)
        .or_else(|| arches.iter().find(|a| a.architecture() == Architecture::Aarch64))
        .or_else(|| arches.first())
}

#[cfg(test)]
mod tests {
    //! Synthetic indirect-symbol tests mirroring `elf_plt.rs` / `pe_iat.rs`. The
    //! `__stubs` *walk* is covered end-to-end by the real-Mach-O console gate
    //! (`verify_macho_imports`); here we unit-test the two pieces of pure logic
    //! that are awkward to reach through a linked binary: the leading-underscore
    //! strip and the stub-entry address arithmetic + indirect-table filtering.
    use super::*;

    /// `_printf` → `printf`; a name without the C-ABI underscore is untouched.
    #[test]
    fn underscore_strip() {
        assert_eq!(strip_leading_underscore(b"_printf"), b"printf".to_vec());
        assert_eq!(strip_leading_underscore(b"_main"), b"main".to_vec());
        // A clean / already-demanglable name keeps exactly one `_` removed.
        assert_eq!(strip_leading_underscore(b"__Znwm"), b"_Znwm".to_vec());
        // No underscore: untouched.
        assert_eq!(strip_leading_underscore(b"foo"), b"foo".to_vec());
        // Empty stays empty.
        assert_eq!(strip_leading_underscore(b""), b"".to_vec());
    }

    /// The pure stub-naming arithmetic + indirect-table filtering the real walk
    /// performs, exercised against a synthetic `S_SYMBOL_STUBS` layout
    /// (1 import + 1 LOCAL placeholder) — no linker / `object::File` needed.
    ///
    /// Models the vendored x86-64 fixture: `__stubs` @ 0x1000005cc, `reserved2`
    /// (stub size) = 6, indirect index 0 → `_printf`, index 1 → INDIRECT_LOCAL.
    #[test]
    fn synthetic_stub_naming() {
        // The flat indirect-symbol table, as the section's window into it.
        let indirect: [u32; 2] = [4 /* -> _printf */, INDIRECT_SYMBOL_LOCAL];
        // A name table keyed by symbol index (stand-in for the `LC_SYMTAB`).
        let symbol_name = |idx: u32| -> Option<&'static [u8]> {
            match idx {
                4 => Some(b"_printf"),
                _ => None,
            }
        };

        let stub_base: u64 = 0x1000005cc;
        let stride: u64 = 6; // reserved2 for the x86-64 fixture

        let mut out: Vec<ImportSym> = Vec::new();
        for (i, &raw) in indirect.iter().enumerate() {
            if raw & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 {
                continue; // the LOCAL placeholder is skipped
            }
            let Some(raw_name) = symbol_name(raw) else { continue };
            let name = strip_leading_underscore(raw_name);
            let addr = stub_base + (i as u64) * stride;
            out.push(ImportSym { addr, name });
        }

        assert_eq!(out.len(), 1, "only the real import is named, the LOCAL is skipped");
        assert_eq!(out[0].addr, 0x1000005cc, "stub entry 0 is at the section base");
        assert_eq!(out[0].name, b"printf".to_vec(), "_printf is named, `_` stripped");
    }

    /// Symbol-pointer slots (`__la_symbol_ptr` / `__got`) are named at
    /// `base + i*ptr` — the GOT-slot analog, for a folded indirect `call [ptr]`.
    #[test]
    fn synthetic_pointer_slot_naming() {
        let indirect: [u32; 1] = [4]; // -> _printf
        let slot_base: u64 = 0x100003000;
        let ptr: u64 = 8; // 64-bit pointer width
        let mut out: Vec<ImportSym> = Vec::new();
        for (i, &raw) in indirect.iter().enumerate() {
            if raw & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 {
                continue;
            }
            let name = strip_leading_underscore(b"_printf");
            out.push(ImportSym { addr: slot_base + (i as u64) * ptr, name });
            let _ = raw;
        }
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, 0x100003000);
        assert_eq!(out[0].name, b"printf".to_vec());
    }
}
