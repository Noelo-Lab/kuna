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
//!    A match one byte into a REX-prefixed `48 FF 25` tail jump is not a thunk
//!    and is dropped ([`super::kuna_rexthunk`]).
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

use std::collections::{HashMap, HashSet};

use super::format::{ImportSym, ImportSymKind};

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
            Ok(pe) => collect::<ImageNtHeaders64>(file, &pe, bytes, 8),
            Err(_) => Vec::new(),
        },
        Ok(FileKind::Pe32) => match PeFile32::parse(bytes) {
            Ok(pe) => collect::<ImageNtHeaders32>(file, &pe, bytes, 4),
            Err(_) => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// The Import Address Table slot ranges of a PE, as `[slot, slot + ptr)` byte
/// ranges in ascending order (empty for a non-PE / no-import / unparsable image).
///
/// The address half of [`resolve_pe_imports`]'s IAT arm, without the names: the
/// `peimportcall` pass paints `Varnode::externref` over exactly these ranges so a
/// `call dword ptr [slot]` resolves through `ActionDeindirect` to the import
/// FunctionSymbol registered at the slot VA.  See
/// [`crate::loader::kuna_peimportcall`].
pub(crate) fn resolve_pe_import_slots(bytes: &[u8]) -> Vec<(u64, u64)> {
    match FileKind::parse(bytes) {
        Ok(FileKind::Pe64) => match PeFile64::parse(bytes) {
            Ok(pe) => slot_ranges::<ImageNtHeaders64>(&pe, 8),
            Err(_) => Vec::new(),
        },
        Ok(FileKind::Pe32) => match PeFile32::parse(bytes) {
            Ok(pe) => slot_ranges::<ImageNtHeaders32>(&pe, 4),
            Err(_) => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// [`resolve_pe_import_slots`] over a typed `PeFile`: reuse the INT/IAT lockstep
/// walk purely for its slot-VA side table, then widen each slot to its pointer.
fn slot_ranges<Pe: ImageNtHeaders>(pe: &PeFile<Pe>, ptr: u64) -> Vec<(u64, u64)> {
    let mut slot_to_name: std::collections::HashMap<u64, Vec<u8>> = std::collections::HashMap::new();
    let mut discard: Vec<ImportSym> = Vec::new();
    if let Ok(Some(it)) = pe.import_table() {
        walk_import_table::<Pe>(
            &it,
            pe.relative_address_base(),
            ptr,
            &mut discard,
            &mut slot_to_name,
        );
    }
    let mut slots: Vec<u64> = slot_to_name.into_keys().collect();
    slots.sort_unstable();
    slots.into_iter().map(|s| (s, s.wrapping_add(ptr))).collect()
}

/// Walk the Import Directory of a typed `PeFile`, pairing the INT (names) and IAT
/// (slot addresses) in lockstep, then decode the `FF 25` thunk veneers and append
/// the exports.  `ptr` is the IAT slot width (8 for PE32+, 4 for PE32).
fn collect<Pe: ImageNtHeaders>(
    neutral: &object::File,
    pe: &PeFile<Pe>,
    bytes: &[u8],
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
    let starts = super::kuna_rexthunk::declared_starts(neutral, pe, bytes);
    decode_iat_thunks(neutral, &starts, &slot_to_name, &mut out);

    // PE exports (additive funcsyms): name + address, registered directly so a
    // call to an exported function of this image resolves.  `relative_address_base`
    // is already folded into `Export::address` by `object`.
    if let Ok(exports) = neutral.exports() {
        for e in exports {
            let name = e.name();
            if !name.is_empty() {
                out.push(ImportSym {
                    addr: e.address(),
                    name: name.to_vec(),
                    kind: ImportSymKind::Export,
                });
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
            out.push(ImportSym {
                addr: slot_va,
                name,
                kind: ImportSymKind::Import,
            });
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
    starts: &[u64],
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

    let mut code: Vec<(u64, &[u8])> = Vec::new();
    let mut image: Vec<(u64, &[u8])> = Vec::new();
    for sec in file.sections() {
        let Ok(data) = sec.data() else { continue };
        let characteristics = match sec.flags() {
            object::SectionFlags::Coff { characteristics } => characteristics,
            _ => 0,
        };
        // Only executable sections carry thunk veneers.
        let exec = characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0 || sec.kind() == object::SectionKind::Text;
        if exec {
            code.push((sec.address(), data));
        }
        if exec || characteristics & object::pe::IMAGE_SCN_MEM_DISCARDABLE == 0 {
            image.push((sec.address(), data));
        }
    }

    let reject_rex = kuna_decomp::kuna_rexthunk::rexthunk_enabled();
    decode_thunks(&code, &image, starts, is_64, reject_rex, slot_to_name, out);
}

/// Decode the `FF 25` thunks of every `(vma, bytes)` code section into `out`. With
/// `reject_rex` on x86-64, a REX tail the image never references is dropped
/// ([`super::kuna_rexthunk`]); `image` is every section but discardable data and
/// `starts` the addresses the image declares to begin code.
fn decode_thunks(
    code: &[(u64, &[u8])],
    image: &[(u64, &[u8])],
    starts: &[u64],
    is_64: bool,
    reject_rex: bool,
    slot_to_name: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<ImportSym>,
) {
    let first = out.len();
    let mut rex_tails: HashSet<u64> = HashSet::new();
    let track = reject_rex && is_64;
    for &(vma, data) in code {
        decode_ff25(vma, data, is_64, slot_to_name, out, track.then_some(&mut rex_tails));
    }
    if rex_tails.is_empty() {
        return;
    }
    let referenced = super::kuna_rexthunk::referenced_cached(code, image, starts, &rex_tails);
    let mut thunks = out.split_off(first);
    thunks.retain(|t| !rex_tails.contains(&t.addr) || referenced.contains(&t.addr));
    out.extend(thunks);
}

/// Scan a code blob for `FF 25 <disp32>` (`jmp [mem]`) and, for each whose target
/// IAT slot is in `slot_to_name`, record the thunk entry address against the
/// import name.  `is_64` selects RIP-relative (x64) vs absolute (x86) addressing.
/// With `rex_tails`, each recorded entry that is a REX tail
/// ([`super::kuna_rexthunk::is_rex_tail`]) is also added to that set.
/// Factored out from [`decode_iat_thunks`] so the arithmetic is unit-testable
/// without an `object::File` (mirrors `elf_plt`'s synthetic decoder tests).
fn decode_ff25(
    vma: u64,
    data: &[u8],
    is_64: bool,
    slot_to_name: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<ImportSym>,
    mut rex_tails: Option<&mut HashSet<u64>>,
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
                if let Some(tails) = rex_tails.as_deref_mut() {
                    if super::kuna_rexthunk::is_rex_tail(vma, data, i, slot_to_name) {
                        tails.insert(insn_addr);
                    }
                }
                out.push(ImportSym {
                    addr: insn_addr,
                    name: name.clone(),
                    kind: ImportSymKind::Import,
                });
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
        decode_ff25(vma, &data, /*is_64*/ true, &map, &mut out, None);

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
        decode_ff25(vma, &data, /*is_64*/ false, &map, &mut out, None);

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
        decode_ff25(vma, &data, true, &map, &mut out, None);
        assert!(out.is_empty());
    }

    fn rel32(op: &[u8], at: u64, target: u64) -> Vec<u8> {
        let mut b = op.to_vec();
        let end = at + op.len() as u64 + 4;
        b.extend_from_slice(&(target.wrapping_sub(end) as i32).to_le_bytes());
        b
    }

    fn thunk_addrs(code: &[(u64, &[u8])], is_64: bool, reject_rex: bool, map: &HashMap<u64, Vec<u8>>) -> Vec<u64> {
        let mut out: Vec<ImportSym> = Vec::new();
        decode_thunks(code, code, &[], is_64, reject_rex, map, &mut out);
        out.iter().map(|s| s.addr).collect()
    }

    /// A REX.W tail jump `48 FF 25` (`lea rcx,[list]; jmp [slot]`) next to a bare
    /// linker thunk through the same slot: only the bare thunk is named. With the
    /// rejection off the entry one byte into the tail jump comes back.
    #[test]
    fn rex_tail_jump_is_not_a_thunk() {
        let vma = 0x140001000u64;
        let slot = 0x140002050u64;
        let mut data = vec![0x48u8, 0x8D, 0x0D, 0, 0, 0, 0];
        let tail = data.len() + 1;
        data.push(0x48);
        data.extend(rel32(&[0xFF, 0x25], vma + tail as u64, slot));
        data.extend_from_slice(&[0xCC, 0xCC]);
        let thunk = data.len();
        data.extend(rel32(&[0xFF, 0x25], vma + thunk as u64, slot));
        let map = HashMap::from([(slot, b"InitializeSListHead".to_vec())]);

        let code = [(vma, &data[..])];
        assert_eq!(thunk_addrs(&code, true, true, &map), vec![vma + thunk as u64]);
        assert_eq!(thunk_addrs(&code, true, false, &map), vec![vma + tail as u64, vma + thunk as u64]);
    }

    /// A genuine thunk right after a function ending in `jmp qword ptr [rax+0x48]`
    /// (`48 FF 60 48`) is preceded by a REX-valued byte. It keeps its name while a
    /// call, a jump or a RIP-relative `lea` anywhere in the code targets it; only a
    /// thunk nothing references is dropped.
    #[test]
    fn called_thunk_after_a_rex_valued_byte_keeps_its_name() {
        let vma = 0x140001000u64;
        let slot = 0x140003000u64;
        let thunk = vma + 4;
        let mut text = vec![0x48u8, 0xFF, 0x60, 0x48];
        text.extend(rel32(&[0xFF, 0x25], thunk, slot));
        let map = HashMap::from([(slot, b"ExitProcess".to_vec())]);
        let caller_at = 0x140005000u64;

        for op in [&[0xE8u8][..], &[0xE9], &[0x0F, 0x85], &[0x48, 0x8D, 0x0D]] {
            let caller = rel32(op, caller_at, thunk);
            let code = [(vma, &text[..]), (caller_at, &caller[..])];
            assert_eq!(thunk_addrs(&code, true, true, &map), vec![thunk], "referenced by {op:x?}");
        }
        let unrelated = rel32(&[0xE8], caller_at, thunk + 1);
        let code = [(vma, &text[..]), (caller_at, &unrelated[..])];
        assert!(thunk_addrs(&code, true, true, &map).is_empty());
    }

    /// PE32 never rejects: `40`-`4F` are `inc`/`dec` in 32-bit code.
    #[test]
    fn x86_thunk_after_inc_dec_byte_is_kept() {
        let vma = 0x401000u64;
        let slot = 0x403000u64;
        let mut data = vec![0x48u8, 0xFF, 0x25];
        data.extend_from_slice(&(slot as u32).to_le_bytes());
        let map = HashMap::from([(slot, b"printf".to_vec())]);
        assert_eq!(thunk_addrs(&[(vma, &data[..])], false, true, &map), vec![vma + 1]);
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
