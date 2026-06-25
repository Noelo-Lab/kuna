//! Mach-O function-entry discovery — the kuna analog of the ELF entry oracles
//! (`super`'s oracles 1-3), adapted to the Apple Mach-O container so a
//! **stripped** Mach-O recovers its function starts without a supplied `--addr`
//! (design §4.1 / §5.3 / §8 PR-13).
//!
//! ## Why a stripped Mach-O needs this — and why `LC_FUNCTION_STARTS` is special
//!
//! Mach-O carries a dedicated, compact **function-start table** in the
//! `LC_FUNCTION_STARTS` load command: a ULEB128 **delta-encoded** list of every
//! function's start address, the first delta relative to the `__TEXT` segment
//! base and each subsequent delta relative to the previous start. The static
//! linker emits it for *every* function, and — crucially — it **survives
//! stripping** (`strip -x` removes the symbol table but keeps the linkedit
//! function-starts blob). So it is the single richest, most reliable Mach-O
//! function-start source, the analog Ghidra reads in
//! `MachoProgramBuilder.markupFunctionStarts`.
//!
//! The unioned oracles:
//!
//! 1. **The entry point** — `LC_MAIN`'s `entryoff` (a file/segment offset, NOT a
//!    VMA: `object`'s `file.entry()` returns the raw `entryoff` here, unlike PE
//!    where it is already rebased — verified). The VMA is `__TEXT.vmaddr +
//!    entryoff`, computed in [`macho_entry_vma`]. (`LC_UNIXTHREAD` — the older
//!    thread-state entry — carries the entry PC directly; handled too.)
//! 2. **`LC_FUNCTION_STARTS`** — the delta-encoded function-start table
//!    ([`function_starts`]). The headline source.
//! 3. **`__DATA,__mod_init_func`** — the C++ static-initializer / constructor
//!    pointer array (the `.init_array` analog); each pointer is a function start.
//! 4. **Exports** (`file.exports()`) — the dynamic export trie's entries. Already
//!    a funcsym source in `macho_stubs::resolve_macho_imports`, re-unioned here so
//!    an export with no function-starts entry is still discovered as a *start*.
//!
//! `__TEXT,__eh_frame` / `__TEXT,__unwind_info` FDE/entry function-starts (the
//! design's §5.3 list) are a *strict subset* of what `LC_FUNCTION_STARTS`
//! already yields (the linker derives both from the same function set), so they
//! add nothing on a binary that carries `LC_FUNCTION_STARTS` — which every
//! `ld64`-linked image does. We therefore lean on `LC_FUNCTION_STARTS` as the
//! complete source rather than re-deriving starts from the CFI tables.
//!
//! The arch-specific oracles `super` already owns (x86-64 prologue patterns) are
//! reused over the Mach-O `__text`. The `_start`→`main` libc-start idiom is
//! ELF-crt1-specific and not reused (Mach-O's entry is `LC_MAIN`, recovered
//! directly).
//!
//! Pure & total: a non-Mach-O input, a fat binary with no usable slice, or a
//! missing load command yields fewer (or zero) candidates — never an error.
//! Fat/universal binaries select one slice (x86-64, then arm64, then first),
//! mirroring [`crate::s1_loader::macho_stubs`].

use object::macho::{MachHeader32, MachHeader64, SECTION_TYPE, S_MOD_INIT_FUNC_POINTERS};
use object::read::macho::{
    FatArch, LoadCommandVariant, MachHeader, MachOFatFile32, MachOFatFile64, Section, Segment,
};
use object::read::Object;
use object::{Architecture, Endianness, FileKind};

/// Collect Mach-O function-start candidate VMAs: the `LC_MAIN`/`LC_UNIXTHREAD`
/// entry, every `LC_FUNCTION_STARTS` start, the `__mod_init_func` initializers,
/// and the exports.
///
/// The caller (`super::collect_entries`) does the exec-section / funcsym-dedup
/// filtering; this only produces raw candidate VMAs.
pub(super) fn macho_entry_candidates(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();

    // Oracle 4 (exports): a named export is also a function start. Address only;
    // the name rides the funcsym stream (`macho_stubs`).
    if let Ok(exports) = file.exports() {
        for e in exports {
            if e.address() != 0 {
                out.push(e.address());
            }
        }
    }

    // Oracles 1-3 (entry / function-starts / mod_init_func) need the typed header
    // for the load commands + linkedit; dispatch on the concrete Mach-O kind,
    // peeling a fat slice if needed.
    collect_thin(bytes, &mut out);

    out
}

/// Dispatch on the concrete Mach-O file kind, peeling one fat slice if the input
/// is a universal binary, then walk the chosen thin image's load commands.
fn collect_thin(bytes: &[u8], out: &mut Vec<u64>) {
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => walk::<MachHeader64<Endianness>>(bytes, out),
        Ok(FileKind::MachO32) => walk::<MachHeader32<Endianness>>(bytes, out),
        Ok(FileKind::MachOFat32) => {
            if let Some(slice) = select_fat_slice_32(bytes) {
                collect_thin(slice, out);
            }
        }
        Ok(FileKind::MachOFat64) => {
            if let Some(slice) = select_fat_slice_64(bytes) {
                collect_thin(slice, out);
            }
        }
        _ => {}
    }
}

/// Walk a thin Mach-O's load commands once, harvesting the entry VMA, the
/// `LC_FUNCTION_STARTS` table, and the `__mod_init_func` pointer arrays. Generic
/// over the header width so 64- and 32-bit share one body.
fn walk<Mach>(bytes: &[u8], out: &mut Vec<u64>)
where
    Mach: MachHeader<Endian = Endianness>,
{
    let header = match Mach::parse(bytes, 0) {
        Ok(h) => h,
        Err(_) => return,
    };
    let endian = match header.endian() {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut commands = match header.load_commands(endian, bytes, 0) {
        Ok(c) => c,
        Err(_) => return,
    };

    // The `__TEXT` segment base — `LC_MAIN`'s entryoff and the first
    // function-starts delta are both relative to it.
    let mut text_base: Option<u64> = None;
    // (dataoff, datasize) of the `LC_FUNCTION_STARTS` linkedit blob.
    let mut func_starts: Option<(u64, u64)> = None;
    // Entry: `LC_MAIN` entryoff (added to text_base) or `LC_UNIXTHREAD` PC (VMA).
    let mut main_entryoff: Option<u64> = None;
    let mut mod_init: Vec<(u64, u64)> = Vec::new(); // (addr, size) of init-func sections

    while let Ok(Some(command)) = commands.next() {
        // Segments: find `__TEXT` (base) and any `__mod_init_func` section.
        if let Ok(Some((segment, section_data))) = Mach::Segment::from_command(command) {
            let seg_name = trim(segment.name());
            if seg_name == b"__TEXT" {
                text_base = Some(segment.vmaddr(endian).into());
            }
            if let Ok(secs) = segment.sections(endian, section_data) {
                for sec in secs {
                    let sec_type = sec.flags(endian) & SECTION_TYPE;
                    if sec_type == S_MOD_INIT_FUNC_POINTERS {
                        mod_init.push((sec.addr(endian).into(), sec.size(endian).into()));
                    }
                }
            }
            continue;
        }
        // The entry-point + function-starts load commands.
        match command.variant() {
            Ok(LoadCommandVariant::EntryPoint(e)) => {
                // `LC_MAIN`: entryoff is relative to the `__TEXT` base.
                main_entryoff = Some(e.entryoff.get(endian));
            }
            Ok(LoadCommandVariant::LinkeditData(d))
                if command.cmd() == object::macho::LC_FUNCTION_STARTS =>
            {
                func_starts = Some((d.dataoff.get(endian) as u64, d.datasize.get(endian) as u64));
            }
            _ => {}
        }
    }

    // Oracle 1: the entry point VMA (`__TEXT` base + LC_MAIN entryoff).
    if let (Some(base), Some(off)) = (text_base, main_entryoff) {
        let vma = base.wrapping_add(off);
        if vma != 0 {
            out.push(vma);
        }
    }

    // Oracle 2: the `LC_FUNCTION_STARTS` delta table.
    if let (Some(base), Some((off, size))) = (text_base, func_starts) {
        decode_function_starts(bytes, off, size, base, out);
    }

    // Oracle 3: `__mod_init_func` initializer pointer arrays.
    let is64 = std::mem::size_of::<Mach::Word>() == 8;
    let ptr = if is64 { 8usize } else { 4usize };
    for (addr, size) in mod_init {
        read_init_pointers(bytes, header, endian, addr, size, ptr, out);
    }
}

/// Decode the `LC_FUNCTION_STARTS` ULEB128 delta table at `bytes[off..off+size]`,
/// pushing each absolute function-start VMA. The first delta is relative to
/// `text_base`; each subsequent delta accumulates onto the running address. A
/// zero delta terminates the list (the trailing padding).
fn decode_function_starts(
    bytes: &[u8],
    off: u64,
    size: u64,
    text_base: u64,
    out: &mut Vec<u64>,
) {
    let start = off as usize;
    let end = match start.checked_add(size as usize) {
        Some(e) if e <= bytes.len() => e,
        _ => return,
    };
    let data = &bytes[start..end];
    let mut p = 0usize;
    let mut addr = text_base;
    while p < data.len() {
        let (delta, np) = match uleb128(data, p) {
            Some(v) => v,
            None => break,
        };
        p = np;
        if delta == 0 {
            break; // terminator / padding
        }
        addr = addr.wrapping_add(delta);
        out.push(addr);
    }
}

/// Read a `__mod_init_func` section's pointer array (each entry an absolute VMA
/// of a C++ static initializer) from the loaded section bytes.
fn read_init_pointers<Mach>(
    bytes: &[u8],
    header: &Mach,
    endian: Endianness,
    sec_addr: u64,
    sec_size: u64,
    ptr: usize,
    out: &mut Vec<u64>,
) where
    Mach: MachHeader<Endian = Endianness>,
{
    // Resolve the section VMA to its file bytes by re-walking the segments (the
    // section's file offset = its containing segment's fileoff + (addr - vmaddr)).
    let data = match section_file_bytes(bytes, header, endian, sec_addr, sec_size) {
        Some(d) => d,
        None => return,
    };
    let n = data.len() / ptr;
    for i in 0..n {
        let o = i * ptr;
        let p = if ptr == 8 {
            u64::from_le_bytes([
                data[o], data[o + 1], data[o + 2], data[o + 3],
                data[o + 4], data[o + 5], data[o + 6], data[o + 7],
            ])
        } else {
            u32::from_le_bytes([data[o], data[o + 1], data[o + 2], data[o + 3]]) as u64
        };
        // The pointer is an absolute VMA (rebased pointers are 0 in a static
        // image with base bias 0); skip nulls.
        if p != 0 {
            out.push(p);
        }
    }
}

/// Resolve a section's `[sec_addr, sec_addr+sec_size)` to its slice of `bytes`
/// by finding the containing segment (`fileoff + (sec_addr - vmaddr)`).
fn section_file_bytes<'a, Mach>(
    bytes: &'a [u8],
    header: &Mach,
    endian: Endianness,
    sec_addr: u64,
    sec_size: u64,
) -> Option<&'a [u8]>
where
    Mach: MachHeader<Endian = Endianness>,
{
    let mut commands = header.load_commands(endian, bytes, 0).ok()?;
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            let vmaddr: u64 = segment.vmaddr(endian).into();
            let vmsize: u64 = segment.vmsize(endian).into();
            if sec_addr >= vmaddr && sec_addr.wrapping_add(sec_size) <= vmaddr.wrapping_add(vmsize) {
                let fileoff: u64 = segment.fileoff(endian).into();
                let file_pos = fileoff.wrapping_add(sec_addr - vmaddr) as usize;
                let file_end = file_pos.checked_add(sec_size as usize)?;
                return bytes.get(file_pos..file_end);
            }
        }
    }
    None
}

/// Trim a NUL-padded fixed-size Mach-O name field to its content bytes.
fn trim(name: &[u8]) -> &[u8] {
    match name.iter().position(|&b| b == 0) {
        Some(i) => &name[..i],
        None => name,
    }
}

/// Read an unsigned LEB128 at `data[off..]`, returning `(value, next_off)`.
fn uleb128(data: &[u8], off: usize) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0u32;
    let mut p = off;
    loop {
        let b = *data.get(p)?;
        p += 1;
        result |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            break;
        }
    }
    Some((result, p))
}

/// Select one slice of a 32-bit fat header (x86-64, then arm64, then first).
fn select_fat_slice_32(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile32::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Select one slice of a 64-bit fat header.
fn select_fat_slice_64(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile64::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Deterministic fat-slice preference (mirrors `macho_stubs::select_fat_arch`):
/// x86-64, then arm64, then the first arch present.
fn select_fat_arch<Fat: FatArch>(arches: &[Fat]) -> Option<&Fat> {
    arches
        .iter()
        .find(|a| a.architecture() == Architecture::X86_64)
        .or_else(|| arches.iter().find(|a| a.architecture() == Architecture::Aarch64))
        .or_else(|| arches.first())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    /// `LC_FUNCTION_STARTS` recovers `_compute` (0x1000005a0) and `_main`
    /// (0x1000005b0), and `LC_MAIN` recovers the entry (also 0x1000005b0) on the
    /// x86-64 fixture — the headline Mach-O fact: function starts even without
    /// relying on the symbol table.
    #[test]
    fn macho_x64_function_starts() {
        let bytes = fixture("macho_imports");
        let file = object::File::parse(bytes.as_slice()).expect("parse macho_imports");
        assert_eq!(file.architecture(), Architecture::X86_64);
        let cands = macho_entry_candidates(&file, bytes.as_slice());
        assert!(cands.contains(&0x1000005a0), "_compute 0x1000005a0 missing (FUNCTION_STARTS)");
        assert!(cands.contains(&0x1000005b0), "_main/entry 0x1000005b0 missing");
        assert!(!cands.contains(&0), "no zero candidate");
    }

    /// The same source linked for arm64 recovers `_compute` (0x100000560) and
    /// `_main` (0x10000056c) — arch-independent (the table is delta-encoded
    /// metadata, no instruction decode).
    #[test]
    fn macho_arm64_function_starts() {
        let bytes = fixture("macho_imports_arm64");
        let file = object::File::parse(bytes.as_slice()).expect("parse macho_imports_arm64");
        assert_eq!(file.architecture(), Architecture::Aarch64);
        let cands = macho_entry_candidates(&file, bytes.as_slice());
        assert!(cands.contains(&0x100000560), "_compute 0x100000560 missing (FUNCTION_STARTS)");
        assert!(cands.contains(&0x10000056c), "_main/entry 0x10000056c missing");
    }

    /// ULEB128 delta decoding: the x86-64 fixture's table is two deltas
    /// (`0x5a0`, `0x10`) → `[0x1000005a0, 0x1000005b0]` off the `__TEXT` base.
    #[test]
    fn function_starts_delta_decode() {
        // 0x5a0 = uleb [0xa0,0x0b], 0x10 = [0x10], then 0 terminator.
        let mut data = Vec::new();
        // 0x5a0 ULEB: 0x5a0 = 0b101_1010_0000 -> bytes 0xa0,0x0b
        data.extend_from_slice(&[0xa0, 0x0b, 0x10, 0x00]);
        let mut out = Vec::new();
        decode_function_starts(&data, 0, data.len() as u64, 0x100000000, &mut out);
        assert_eq!(out, vec![0x1000005a0, 0x1000005b0]);
    }
}
