//! PE function-entry discovery — the kuna analog of the ELF entry oracles
//! (`super`'s oracles 1-3), adapted to the Windows PE container so a **stripped**
//! PE recovers its function starts without a supplied `--addr` (design §4.1 /
//! §5.3 / §8 PR-12).
//!
//! ## Why a stripped PE needs this
//!
//! A linked PE32+ carries no symbol table once stripped (`-s`): `file.symbols()`
//! is empty and `file.exports()` is empty for a non-DLL exe. Without an entry
//! oracle the only function the loader knows is whatever the user names with
//! `--addr`. Ghidra's `PeLoader` instead seeds functions from the PE's own
//! structural metadata, and the highest-value sources survive stripping:
//!
//! 1. **The entry point** (`AddressOfEntryPoint + ImageBase`) — `object`'s
//!    `file.entry()` already returns this as a VMA for PE (verified: the PE arm
//!    rebases, unlike Mach-O `LC_MAIN`). The `_start`/`mainCRTStartup`.
//! 2. **`.pdata` exception handlers** — the x64 SEH analog of ELF `.eh_frame`
//!    FDEs. Each `RUNTIME_FUNCTION` record (`{BeginAddress, EndAddress,
//!    UnwindData}`, 12 bytes, all RVAs) marks a function: `BeginAddress` is a
//!    function start. On x64 the compiler emits one per non-leaf function, so
//!    `.pdata` recovers nearly every function in a stripped binary — the richest
//!    PE source (97 records in the vendored `pe_imports_stripped.exe`). Ghidra's
//!    `PdataDirectory` / `RuntimeFunction.createFunctions`.
//! 3. **TLS callbacks** — the TLS directory's `AddressOfCallBacks` array is a
//!    NULL-terminated list of `PIMAGE_TLS_CALLBACK` function pointers (absolute
//!    VAs, not RVAs) the loader runs before the entry point. Ghidra's
//!    `TLSDataDirectory`. Each non-null pointer is a function start.
//! 4. **Exports** (`file.exports()`) — a DLL / export-bearing exe's named
//!    entry points. Already a funcsym source in `pe_iat::resolve_pe_imports`, but
//!    re-unioned here so an export with no `.pdata`/symbol is still discovered as
//!    a *start* (the address is what matters here; the name rides the funcsym).
//!
//! The arch-specific oracles `super` already owns (x86-64 prologue patterns,
//! oracle 5) are reused unchanged over the PE `.text` — they key off
//! `file.architecture()` and the executable-section bytes, not the container.
//! The `_start`→`main` libc-start idiom (oracle 4) is ELF-crt1-specific (the
//! `__libc_start_main` GOT call shape); the PE CRT (`__scrt_common_main` /
//! `mainCRTStartup`) uses a different idiom, so it is **not** reused — `.pdata`
//! already recovers `main` directly, making the idiom redundant for PE.
//!
//! Every candidate is validated to land in an executable section and to not
//! duplicate an existing funcsym by `super::collect_entries` — this module only
//! produces the raw candidate VMAs.

use object::pe::{ImageNtHeaders32, ImageNtHeaders64};
use object::read::pe::{ImageNtHeaders, PeFile, PeFile32, PeFile64};
use object::read::Object;
use object::{FileKind, LittleEndian as LE};

/// Collect PE function-start candidate VMAs: the entry point, every `.pdata`
/// `RUNTIME_FUNCTION` `BeginAddress`, the TLS callbacks, and the exports.
///
/// Pure & total: a non-PE input, an unparsable layout, or a missing directory
/// yields fewer (or zero) candidates — never an error or panic. The caller
/// (`super::collect_entries`) does the exec-section / funcsym-dedup filtering.
pub(super) fn pe_entry_candidates(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();

    // Oracle 1: the entry point. `object` returns the rebased VMA for PE.
    let entry = file.entry();
    if entry != 0 {
        out.push(entry);
    }

    // Oracle 4 (exports): a named export is also a function start. Address only;
    // the name rides the funcsym stream (`pe_iat`).
    if let Ok(exports) = file.exports() {
        for e in exports {
            if e.address() != 0 {
                out.push(e.address());
            }
        }
    }

    // Oracles 2 + 3 (.pdata + TLS) need the typed PE parser for the data
    // directories; branch on the concrete PE width.
    match FileKind::parse(bytes) {
        Ok(FileKind::Pe64) => {
            if let Ok(pe) = PeFile64::parse(bytes) {
                collect_typed::<ImageNtHeaders64>(&pe, bytes, 8, &mut out);
            }
        }
        Ok(FileKind::Pe32) => {
            if let Ok(pe) = PeFile32::parse(bytes) {
                collect_typed::<ImageNtHeaders32>(&pe, bytes, 4, &mut out);
            }
        }
        _ => {}
    }

    out
}

/// Append the `.pdata` `BeginAddress`es and the TLS callbacks for a typed PE.
/// `ptr` is the pointer width (8 for PE32+, 4 for PE32) — the TLS callback array
/// holds absolute VAs of that width.
fn collect_typed<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    ptr: usize,
    out: &mut Vec<u64>,
) {
    let image_base = pe.relative_address_base();
    pdata_begins(pe, bytes, image_base, out);
    tls_callbacks(pe, bytes, ptr, out);
}

/// Walk the exception directory (`.pdata`) as an array of `RUNTIME_FUNCTION`
/// records and push each `BeginAddress` (converted RVA → VMA).
///
/// `RUNTIME_FUNCTION` (x64/ARM64) = `{ u32 BeginAddress; u32 EndAddress; u32
/// UnwindInfoAddress }` — 12 bytes, all RVAs. (32-bit x86 has no `.pdata`, so
/// this is naturally a no-op there.) Faithful to Ghidra's `PdataDirectory` /
/// `RuntimeFunction`.
fn pdata_begins<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    image_base: u64,
    out: &mut Vec<u64>,
) {
    const RUNTIME_FUNCTION_SIZE: usize = 12;
    let dir = match pe.data_directories().get(object::pe::IMAGE_DIRECTORY_ENTRY_EXCEPTION) {
        Some(d) => d,
        None => return,
    };
    if dir.size.get(LE) == 0 {
        return;
    }
    // The directory bytes live in the section containing its RVA (usually
    // `.pdata`). `data` resolves the RVA against the loaded section bytes.
    let data = match dir.data(bytes, &pe.section_table()) {
        Ok(d) => d,
        Err(_) => return,
    };
    let n = data.len() / RUNTIME_FUNCTION_SIZE;
    for i in 0..n {
        let o = i * RUNTIME_FUNCTION_SIZE;
        let begin_rva = u32::from_le_bytes([data[o], data[o + 1], data[o + 2], data[o + 3]]);
        if begin_rva != 0 {
            out.push(image_base.wrapping_add(begin_rva as u64));
        }
    }
}

/// Push every non-null TLS callback pointer from the TLS directory's
/// `AddressOfCallBacks` array (the loader runs these before the entry point —
/// each is a function start). The callbacks are stored as absolute VAs (not
/// RVAs), NULL-terminated. Faithful to Ghidra's `TLSDataDirectory`.
fn tls_callbacks<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    ptr: usize,
    out: &mut Vec<u64>,
) {
    let image_base = pe.relative_address_base();
    let dir = match pe.data_directories().get(object::pe::IMAGE_DIRECTORY_ENTRY_TLS) {
        Some(d) => d,
        None => return,
    };
    if dir.size.get(LE) == 0 {
        return;
    }
    // The TLS directory header. `AddressOfCallBacks` is the 4th VA-width word
    // (after StartAddressOfRawData / EndAddressOfRawData / AddressOfIndex), all
    // stored as absolute VAs.
    let dir_bytes = match dir.data(bytes, &pe.section_table()) {
        Ok(d) => d,
        Err(_) => return,
    };
    let cb_field_off = 3 * ptr; // past Start/End raw-data + AddressOfIndex
    let callbacks_va = match read_word(dir_bytes, cb_field_off, ptr) {
        Some(v) if v != 0 => v,
        _ => return,
    };
    // The callback array lives at `callbacks_va` (an absolute VA); resolve it to
    // its RVA, read NULL-terminated pointers from the loaded section bytes.
    let callbacks_rva = match callbacks_va.checked_sub(image_base) {
        Some(r) => r,
        None => return,
    };
    let arr = match pe.section_table().pe_data_at(bytes, callbacks_rva as u32) {
        Some(a) => a,
        None => return,
    };
    let mut o = 0usize;
    // Cap the walk so a corrupt (un-terminated) array can never loop unbounded.
    let max = 256usize;
    let mut count = 0usize;
    while o + ptr <= arr.len() && count < max {
        let p = match read_word(arr, o, ptr) {
            Some(v) => v,
            None => break,
        };
        if p == 0 {
            break; // NULL terminator
        }
        out.push(p); // absolute VA of the callback function
        o += ptr;
        count += 1;
    }
}

/// Read a `ptr`-width little-endian word at `data[off..]`, or `None` if short.
fn read_word(data: &[u8], off: usize, ptr: usize) -> Option<u64> {
    if off + ptr > data.len() {
        return None;
    }
    let mut v = 0u64;
    for i in 0..ptr {
        v |= (data[off + i] as u64) << (8 * i);
    }
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    /// The stripped PE has no symbols/exports, but `.pdata` + the entry point
    /// recover its functions. `main` (0x140001592) and the entry (0x1400014f0)
    /// are both `.pdata`-covered, so a stripped PE finds them with no `--addr`.
    #[test]
    fn pe_stripped_pdata_recovers_functions() {
        let bytes = fixture("pe_imports_stripped.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped PE");
        let cands = pe_entry_candidates(&file, bytes.as_slice());
        // The entry point (oracle 1).
        assert!(cands.contains(&0x1400014f0), "entry 0x1400014f0 missing from {cands:#x?}");
        // `.pdata`-covered functions (oracle 2), incl. `main`.
        assert!(cands.contains(&0x140001592), "main 0x140001592 missing (.pdata)");
        // The richest source: dozens of records in this binary.
        assert!(
            cands.len() >= 50,
            ".pdata should recover many functions, got {} candidates",
            cands.len()
        );
        // No spurious 0.
        assert!(!cands.contains(&0), "no zero candidate");
    }
}
