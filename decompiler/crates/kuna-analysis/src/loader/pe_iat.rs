//! PE IAT/INT import-name resolution — the kuna analog of Ghidra's
//! `PeLoader.processImports` / `ImportDataDirectory` markup
//! (`Ghidra/Features/PE/.../format/pe/ImportDataDirectory.java`,
//! `.../opinion/PeLoader.java`).
//!
//! ## Why this lives in kuna at all
//!
//! kuna ports Ghidra's C++ *decompiler*; resolving a Windows import slot
//! (`0x14000d33c`) to its imported library name (`puts`) is done by Ghidra's
//! Java *loader* layer, which the port never carried over.  Without it a CALL
//! that lands on an import has no symbol, so `FlowInfo::queryCall` (kuna
//! `decompile_drive.rs::query_call`) finds nothing and the printer falls back to
//! `sub_<addr>`.  This module reconstructs the slot→name mapping straight from
//! the PE Import Directory, so the loader can register the imports as ordinary
//! `FunctionSymbol`s in the same stream as the `.symtab`/`.dynsym` defined
//! functions — exactly the ELF `elf_plt` model, with the IAT slot playing the
//! role of the ELF GOT slot.
//!
//! ## What a PE call to an import looks like (and what we name)
//!
//! A PE references an import through an **Import Address Table (IAT) slot** — a
//! pointer the loader fills in at load time with the resolved function address.
//! Three call shapes reach it (all covered here):
//!
//! 1. **Direct indirect call** — `call [rip+disp]` (x64) / `call [abs32]` (x86)
//!    straight to the IAT slot.  The decompiler constant-folds the `[slot]` load
//!    (the IAT lives in a read-only `.idata` page), so — exactly like ELF's GOT
//!    — **the name attaches to the IAT slot VA**.  This is the *primary*,
//!    arch-independent path (no per-arch stub decode).
//! 2. **Thunk veneer** — MinGW often routes a *direct* `call thunk` to a
//!    one-instruction veneer `jmp [rip+disp]` (`FF 25 <disp32>`) that jumps
//!    through the IAT slot.  Naming only the slot leaves the `call thunk`
//!    rendering `sub_<thunk>`; so we *also* decode the `FF 25` veneers over the
//!    executable sections and name the **thunk entry** with the slot's import
//!    name (the decode is identical to the ELF x86-64 PLT `decode_x86_64`).
//! 3. **Named thunk / wrapper** — a non-stripped MinGW PE additionally carries a
//!    COFF symbol *on* the thunk (`puts`), which the format-neutral
//!    `file.symbols()` funcsym source already registers; this module is purely
//!    additive to that.
//!
//! ## Algorithm (matches Ghidra's `ImportDataDirectory` loop)
//!
//! For each import descriptor, walk the INT (Import Name Table — the names) and
//! the IAT (the slot addresses) **in lockstep**: the `i`-th INT entry's name
//! belongs to the IAT slot at `image_base + first_thunk_rva + i*ptr`.  Emit one
//! [`ImportSym`] per slot.  An import-by-ordinal (no name) synthesizes
//! `<DLL>_Ordinal_<n>` (named beats unresolved).
//!
//! Everything degrades gracefully: a non-PE input, a missing import directory, or
//! an unparsable descriptor yields fewer (or zero) entries.  This module never
//! panics and never errors — it only ever produces *more* names.

use object::pe::{ImageNtHeaders32, ImageNtHeaders64};
use object::read::pe::{ImageNtHeaders, ImportTable, PeFile, PeFile32, PeFile64};
use object::read::{Object, ObjectSection};
use object::{FileKind, LittleEndian as LE};

use super::format::ImportSym;

/// Resolve PE import slots (and the thunk veneers that jump through them) to
/// their imported function names.
///
/// Returns one [`ImportSym`] per IAT slot (`addr` = the slot VA the decompiler
/// constant-folds) plus one per decodable `FF 25` thunk veneer (`addr` = the
/// thunk entry a direct `call` targets), plus the PE exports.  Empty when the
/// input is not a PE, has no import directory, or the layout is unparsable.
pub(crate) fn resolve_pe_imports(file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
    // Branch on the concrete PE width: the typed parsers (`PeFile32`/`PeFile64`)
    // expose the Import Directory + thunk widths the neutral `object::File` view
    // does not.  A `FileKind` we do not recognize (or a parse failure) yields an
    // empty result — never an error.
    match FileKind::parse(bytes) {
        Ok(FileKind::Pe64) => match PeFile64::parse(bytes) {
            Ok(pe) => collect::<ImageNtHeaders64>(file, &pe, 8),
            Err(_) => Vec::new(),
        },
        Ok(FileKind::Pe32) => match PeFile32::parse(bytes) {
            Ok(pe) => collect::<ImageNtHeaders32>(file, &pe, 4),
            Err(_) => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// Walk the Import Directory of a typed `PeFile`, pairing the INT (names) and IAT
/// (slot addresses) in lockstep, then decode the `FF 25` thunk veneers and append
/// the exports.  `ptr` is the IAT slot width (8 for PE32+, 4 for PE32).
fn collect<Pe: ImageNtHeaders>(
    neutral: &object::File,
    pe: &PeFile<Pe>,
    ptr: u64,
) -> Vec<ImportSym> {
    let mut out: Vec<ImportSym> = Vec::new();

    let image_base = pe.relative_address_base(); // = optional-header ImageBase

    // The IAT slot VA → name map, also used to name the `FF 25` thunk veneers.
    let mut slot_to_name: std::collections::HashMap<u64, Vec<u8>> = std::collections::HashMap::new();

    if let Ok(Some(it)) = pe.import_table() {
        walk_import_table::<Pe>(&it, image_base, ptr, &mut out, &mut slot_to_name);
    }

    // Thunk veneers: a direct `call thunk` to a `jmp [rip+disp]` (`FF 25 …`)
    // veneer that jumps through an IAT slot we just named.  Decode the veneers
    // over the executable sections (x86/x64 only — `FF 25` is an x86 opcode; on
    // other arches the call shape differs and this loop simply matches nothing).
    decode_iat_thunks(neutral, &slot_to_name, &mut out);

    // PE exports (additive funcsyms): name + address, registered directly so a
    // call to an exported function of this image resolves.  `relative_address_base`
    // is already folded into `Export::address` by `object`.
    if let Ok(exports) = neutral.exports() {
        for e in exports {
            let name = e.name();
            if !name.is_empty() {
                out.push(ImportSym { addr: e.address(), name: name.to_vec() });
            }
        }
    }

    out
}

/// Walk one import table, emitting an [`ImportSym`] per IAT slot and recording the
/// slot-VA → name map (for the thunk-veneer decode).
fn walk_import_table<Pe: ImageNtHeaders>(
    it: &ImportTable<'_>,
    image_base: u64,
    ptr: u64,
    out: &mut Vec<ImportSym>,
    slot_to_name: &mut std::collections::HashMap<u64, Vec<u8>>,
) {
    let mut descs = match it.descriptors() {
        Ok(d) => d,
        Err(_) => return,
    };
    while let Ok(Some(desc)) = descs.next() {
        // The DLL name (for ordinal synthesis); best-effort.
        let dll = it.name(desc.name.get(LE)).unwrap_or(b"");

        // The INT (`original_first_thunk`) carries the *names*; the IAT
        // (`first_thunk`) is the slot table.  A bound import may have a zero INT,
        // in which case the IAT itself still holds the (pre-binding) name
        // thunks, so fall back to it — exactly Ghidra's `getOriginalFirstThunk()`
        // / `getFirstThunk()` preference.
        let int_rva = {
            let oft = desc.original_first_thunk.get(LE);
            if oft != 0 { oft } else { desc.first_thunk.get(LE) }
        };
        let iat_rva = desc.first_thunk.get(LE) as u64;

        let mut names = match it.thunks(int_rva) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut i = 0u64;
        loop {
            let thunk = match names.next::<Pe>() {
                Ok(Some(t)) => t,
                _ => break, // end of this descriptor's thunk list (or a read error)
            };
            let slot_va = image_base.wrapping_add(iat_rva).wrapping_add(i.wrapping_mul(ptr));
            let name: Vec<u8> = match it.import::<Pe>(thunk) {
                Ok(object::read::pe::Import::Name(_hint, n)) => n.to_vec(),
                Ok(object::read::pe::Import::Ordinal(ord)) => synth_ordinal_name(dll, ord),
                Err(_) => {
                    i += 1;
                    continue;
                }
            };
            slot_to_name.insert(slot_va, name.clone());
            out.push(ImportSym { addr: slot_va, name });
            i += 1;
        }
    }
}

/// Synthesize a name for an import-by-ordinal: `<DLL-stem>_Ordinal_<n>` (the
/// extension is dropped, so `ws2_32.dll` → `ws2_32_Ordinal_115`).  A named symbol
/// — even a synthetic one — beats an unresolved `sub_<addr>` call (design §3.5).
fn synth_ordinal_name(dll: &[u8], ord: u16) -> Vec<u8> {
    let dll = String::from_utf8_lossy(dll);
    let stem = dll.rsplit_once('.').map(|(s, _)| s).unwrap_or(&dll);
    format!("{stem}_Ordinal_{ord}").into_bytes()
}

/// Decode the x86/x64 `FF 25 <disp32>` import thunk veneers over every executable
/// section, naming each thunk **entry** with the import its target IAT slot holds.
///
/// MinGW emits a one-instruction veneer `jmp QWORD PTR [rip+disp]` (x64) /
/// `jmp DWORD PTR [abs32]` (x86) per import and routes *direct* `call thunk`s to
/// it; naming only the IAT slot would leave such a `call` rendering `sub_<thunk>`.
/// The decode is the same `FF 25` recovery as the ELF x86-64 PLT
/// (`elf_plt::decode_x86_64`): the byte after the 6-byte `FF 25 <disp32>` is the
/// next-instruction address, and the slot is `next + sign(disp32)` (x64,
/// RIP-relative) or the absolute `disp32` (x86).
fn decode_iat_thunks(
    file: &object::File,
    slot_to_name: &std::collections::HashMap<u64, Vec<u8>>,
    out: &mut Vec<ImportSym>,
) {
    if slot_to_name.is_empty() {
        return;
    }
    let is_64 = file.is_64();
    let is_x86 = matches!(
        file.architecture(),
        object::Architecture::X86_64 | object::Architecture::X86_64_X32 | object::Architecture::I386
    );
    if !is_x86 {
        return; // `FF 25` is x86-only; other arches use a different call shape.
    }

    for sec in file.sections() {
        // Only executable sections carry thunk veneers.
        let exec = matches!(
            sec.flags(),
            object::SectionFlags::Coff { characteristics }
                if characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
        ) || sec.kind() == object::SectionKind::Text;
        if !exec {
            continue;
        }
        let vma = sec.address();
        let data = match sec.data() {
            Ok(d) => d,
            Err(_) => continue,
        };
        decode_ff25(vma, data, is_64, slot_to_name, out);
    }
}

/// Scan a code blob for `FF 25 <disp32>` (`jmp [mem]`) and, for each whose target
/// IAT slot is in `slot_to_name`, record the thunk entry address against the
/// import name.  `is_64` selects RIP-relative (x64) vs absolute (x86) addressing.
/// Factored out from [`decode_iat_thunks`] so the arithmetic is unit-testable
/// without an `object::File` (mirrors `elf_plt`'s synthetic decoder tests).
fn decode_ff25(
    vma: u64,
    data: &[u8],
    is_64: bool,
    slot_to_name: &std::collections::HashMap<u64, Vec<u8>>,
    out: &mut Vec<ImportSym>,
) {
    let mut i = 0usize;
    while i + 6 <= data.len() {
        if data[i] == 0xFF && data[i + 1] == 0x25 {
            let disp = i32::from_le_bytes([data[i + 2], data[i + 3], data[i + 4], data[i + 5]]);
            let insn_addr = vma + i as u64;
            let slot = if is_64 {
                // RIP-relative: rip = insn_addr + 6 (next insn), slot = rip + disp32.
                insn_addr.wrapping_add(6).wrapping_add(disp as i64 as u64)
            } else {
                // x86: the disp32 IS the absolute IAT slot address.
                disp as u32 as u64
            };
            if let Some(name) = slot_to_name.get(&slot) {
                // The thunk entry is the `FF` byte (the `call` target).  MinGW
                // veneers have no CET `endbr` prefix, so no back-up is needed.
                out.push(ImportSym { addr: insn_addr, name: name.clone() });
            }
            i += 6;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    //! Synthetic decoder/ordinal tests mirroring `elf_plt.rs`'s per-arch tests.
    //! The IAT *walk* itself is covered end-to-end by the real-PE console gate
    //! (`verify_pe_imports`); here we unit-test the two pieces of arithmetic that
    //! are awkward to reach through a linked binary: the `FF 25` thunk decode
    //! (both widths) and the ordinal-name synthesis.
    use super::*;
    use std::collections::HashMap;

    /// x64 `FF 25 <disp32>` (`jmp [rip+disp]`) resolves to `rip+disp` where
    /// `rip` is the byte after the 6-byte instruction; the thunk entry is named.
    #[test]
    fn ff25_x64_thunk_decode() {
        // A puts veneer like MinGW's `0x140007240: ff 25 f6 60 00 00`
        // (`jmp [rip+0x60f6]` # 0x14000d33c).
        let vma = 0x140007240u64;
        let disp: i32 = 0x60f6;
        let mut data = vec![0xFFu8, 0x25];
        data.extend_from_slice(&disp.to_le_bytes());
        data.extend_from_slice(&[0x90, 0x90]); // trailing nops

        // slot = (vma + 6) + 0x60f6
        let slot = vma + 6 + disp as u64;
        assert_eq!(slot, 0x14000d33c);

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(slot, b"puts".to_vec());
        let mut out: Vec<ImportSym> = Vec::new();
        decode_ff25(vma, &data, /*is_64*/ true, &map, &mut out);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, vma); // the thunk entry (the call target)
        assert_eq!(out[0].name, b"puts".to_vec());
    }

    /// x86 `FF 25 <abs32>` (`jmp [abs32]`) resolves to the absolute `disp32` IAT
    /// slot (no RIP-relative offset).
    #[test]
    fn ff25_x86_thunk_decode() {
        let vma = 0x401050u64;
        let slot: u32 = 0x40a004;
        let mut data = vec![0xFFu8, 0x25];
        data.extend_from_slice(&slot.to_le_bytes());

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(slot as u64, b"printf".to_vec());
        let mut out: Vec<ImportSym> = Vec::new();
        decode_ff25(vma, &data, /*is_64*/ false, &map, &mut out);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, vma);
        assert_eq!(out[0].name, b"printf".to_vec());
    }

    /// A `FF 25` whose target slot is NOT a known import is left alone (no
    /// spurious name) — the same self-correcting cross-check the ELF decoder uses.
    #[test]
    fn ff25_unknown_slot_is_ignored() {
        let vma = 0x140007240u64;
        let mut data = vec![0xFFu8, 0x25];
        data.extend_from_slice(&0x10i32.to_le_bytes());
        let map: HashMap<u64, Vec<u8>> = HashMap::new(); // empty: nothing known
        let mut out: Vec<ImportSym> = Vec::new();
        decode_ff25(vma, &data, true, &map, &mut out);
        assert!(out.is_empty());
    }

    /// Ordinal-name synthesis drops the DLL extension: `ws2_32.dll` + 115 →
    /// `ws2_32_Ordinal_115`.
    #[test]
    fn ordinal_name_synthesis() {
        assert_eq!(synth_ordinal_name(b"ws2_32.dll", 115), b"ws2_32_Ordinal_115".to_vec());
        assert_eq!(synth_ordinal_name(b"KERNEL32.dll", 7), b"KERNEL32_Ordinal_7".to_vec());
        // No extension: the whole string is the stem.
        assert_eq!(synth_ordinal_name(b"mylib", 3), b"mylib_Ordinal_3".to_vec());
    }
}
