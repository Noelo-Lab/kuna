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
//!    UnwindInfoAddress}`, 12 bytes, all RVAs) marks a function: `BeginAddress`
//!    is a function start. On x64 the compiler emits one per non-leaf function,
//!    so `.pdata` recovers nearly every function in a stripped binary — the
//!    richest PE source (97 records in the vendored `pe_imports_stripped.exe`).
//!    Ghidra's `PdataDirectory` / `RuntimeFunction.createFunctions`.
//!
//!    Two things make a record something other than a function start. A record
//!    whose `UNWIND_INFO` sets `UNW_FLAG_CHAININFO` describes a separated code
//!    chunk of another function — MSVC emits one per shrink-wrapped prologue or
//!    out-of-line cold block — and its `BeginAddress` is a point *inside* the
//!    primary, so `pdatachained` (default-on) skips it. And ARM, ARM64, ARM64EC
//!    and ARM64X images use an 8-byte `{BeginAddress, UnwindData}` record whose
//!    `BeginAddress` carries the Thumb bit, so the table is walked at the stride
//!    the machine dictates rather than at a fixed 12, and a machine with neither
//!    shape is not walked at all (`PdataForm`).
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

use object::pe::{
    ImageNtHeaders32, ImageNtHeaders64, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM,
    IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_ARM64EC, IMAGE_FILE_MACHINE_ARM64X,
    IMAGE_FILE_MACHINE_ARMNT, IMAGE_FILE_MACHINE_I386,
};
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
/// records and push each `BeginAddress` (converted RVA → VMA), minus the records
/// the image itself declares to be interior chunks.
///
/// The record shape is machine-dependent ([`PdataForm`]). On the x64 form a
/// record whose `UNWIND_INFO` carries `UNW_FLAG_CHAININFO` describes a separated
/// code chunk of another function rather than a function of its own, so its
/// `BeginAddress` is skipped (`pdatachained`, default-on) exactly as Ghidra's
/// `ImageRuntimeFunctionEntries_X86.markup` skips it.
fn pdata_begins<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    image_base: u64,
    out: &mut Vec<u64>,
) {
    let form = match PdataForm::for_machine(pe.nt_headers().file_header().machine.get(LE)) {
        Some(f) => f,
        None => return,
    };
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
    let stride = form.record_size();
    let skip_chained = matches!(form, PdataForm::X86)
        && kuna_decomp::kuna_pdatachained::pdatachained_enabled();
    let n = data.len() / stride;
    for i in 0..n {
        let o = i * stride;
        let begin_rva = read_u32(data, o);
        if begin_rva == 0 {
            continue;
        }
        match form {
            PdataForm::X86 => {
                if skip_chained && unwind_is_chained(pe, bytes, read_u32(data, o + 8)) {
                    continue;
                }
                out.push(image_base.wrapping_add(begin_rva as u64));
            }
            // The ARM form's low bit is a Thumb marker, not part of the address
            // (`ImageRuntimeFunctionEntries_ARM.java:70`). The second dword is an
            // `.xdata` RVA only when its low two bits are clear and packed unwind
            // data otherwise, so it is never dereferenced here — Ghidra does not
            // decode ARM chained fragments either.
            PdataForm::Arm => {
                out.push(image_base.wrapping_add((begin_rva & !1) as u64));
            }
        }
    }
}

/// The `.pdata` record shape a PE `FileHeader.Machine` selects. x86/x64 images
/// use the 12-byte `{BeginAddress, EndAddress, UnwindInfoAddress}` record; ARM,
/// ARM64, ARM64EC and ARM64X use the 8-byte `{BeginAddress, UnwindData}` record.
/// Every other machine — IA64, MIPS, SH, PowerPC — has its own layout (a MIPS
/// record is 20 bytes), so `None` says the table cannot be read at all and the
/// directory is left alone rather than misparsed. Ghidra dispatches the same
/// three ways, logging "Exception Data unsupported architecture" and leaving
/// `functionEntries` null (`ExceptionDataDirectory.java:59-68`).
#[derive(Clone, Copy)]
enum PdataForm {
    X86,
    Arm,
}

impl PdataForm {
    fn for_machine(machine: u16) -> Option<Self> {
        match machine {
            IMAGE_FILE_MACHINE_I386 | IMAGE_FILE_MACHINE_AMD64 => Some(PdataForm::X86),
            IMAGE_FILE_MACHINE_ARM
            | IMAGE_FILE_MACHINE_ARMNT
            | IMAGE_FILE_MACHINE_ARM64
            | IMAGE_FILE_MACHINE_ARM64EC
            | IMAGE_FILE_MACHINE_ARM64X => Some(PdataForm::Arm),
            _ => None,
        }
    }

    fn record_size(self) -> usize {
        match self {
            PdataForm::X86 => 12,
            PdataForm::Arm => 8,
        }
    }
}

/// Whether the `UNWIND_INFO` at `unwind_rva` carries `UNW_FLAG_CHAININFO`, i.e.
/// the record pointing at it is a separated chunk of another function rather
/// than a function start. The flags are the high five bits of the first byte
/// (`PEx64UnwindInfo.java:173`).
///
/// Total, like the rest of the module: a null RVA, an RVA no section covers, or
/// an empty slice reads as "not chained", so the filter can only ever subtract a
/// record it has positively identified.
fn unwind_is_chained<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    unwind_rva: u32,
) -> bool {
    const UNW_FLAG_CHAININFO: u8 = 0x4;
    if unwind_rva == 0 {
        return false;
    }
    match pe.section_table().pe_data_at(bytes, unwind_rva) {
        Some(d) if !d.is_empty() => (d[0] >> 3) & UNW_FLAG_CHAININFO != 0,
        _ => false,
    }
}

/// Read the little-endian `u32` at `data[off..off + 4]`, or 0 if short.
fn read_u32(data: &[u8], off: usize) -> u32 {
    match data.get(off..off + 4) {
        Some(b) => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        None => 0,
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

    /// A chained (`UNW_FLAG_CHAININFO`) `RUNTIME_FUNCTION` names a separated
    /// chunk of the record before it, so its `BeginAddress` is not a function
    /// start (GH-403). The two ordinary records still are.
    #[test]
    fn pdata_chained_record_is_not_a_function_start() {
        std::env::remove_var(kuna_decomp::kuna_pdatachained::PDATACHAINED_ENV);
        let bytes = fixture("pe_chainedunwind_x86_64.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse chained-unwind PE");
        let cands = pe_entry_candidates(&file, bytes.as_slice());
        assert!(cands.contains(&0x140001000), "primary missing from {cands:#x?}");
        assert!(cands.contains(&0x140001040), "entry point missing from {cands:#x?}");
        assert!(
            !cands.contains(&0x140001020),
            "chained chunk 0x140001020 claimed as a function in {cands:#x?}"
        );
    }

    /// ARM64 `.pdata` holds 8-byte `{BeginAddress, UnwindData}` records. Read at
    /// the x64 stride of 12 the probe's 32-byte directory yields two reads, one
    /// of them landing on `record[1].UnwindData`; at the right stride all four
    /// function starts come back.
    #[test]
    fn pdata_arm64_uses_the_eight_byte_record() {
        let bytes = fixture("pe_pdata_arm64.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse arm64 PE");
        let cands = pe_entry_candidates(&file, bytes.as_slice());
        for vma in [0x140001000u64, 0x140001010, 0x140001020, 0x140001030] {
            assert!(cands.contains(&vma), "{vma:#x} missing from {cands:#x?}");
        }
    }

    /// The record shape follows `FileHeader.Machine`, as Ghidra's
    /// `ExceptionDataDirectory` does, and a machine that is neither an x86 nor
    /// an ARM variant has no readable shape at all.
    #[test]
    fn pdata_form_dispatches_on_machine() {
        assert_eq!(
            PdataForm::for_machine(IMAGE_FILE_MACHINE_AMD64).map(PdataForm::record_size),
            Some(12)
        );
        assert_eq!(
            PdataForm::for_machine(IMAGE_FILE_MACHINE_I386).map(PdataForm::record_size),
            Some(12)
        );
        for arm in [
            IMAGE_FILE_MACHINE_ARM,
            IMAGE_FILE_MACHINE_ARMNT,
            IMAGE_FILE_MACHINE_ARM64,
            IMAGE_FILE_MACHINE_ARM64EC,
            IMAGE_FILE_MACHINE_ARM64X,
        ] {
            assert_eq!(
                PdataForm::for_machine(arm).map(PdataForm::record_size),
                Some(8),
                "machine {arm:#x}"
            );
        }
        // IA64, MIPS R4000, SH-4, PowerPC: each has its own record layout (a
        // MIPS one is 20 bytes), so none of them is read.
        for other in [0x0200u16, 0x0166, 0x01a6, 0x01f0, 0] {
            assert!(PdataForm::for_machine(other).is_none(), "machine {other:#x}");
        }
    }

    /// An unsupported machine parses no exception directory at all rather than
    /// reading it at the x64 stride and dereferencing dword[2] as an
    /// `UNWIND_INFO` RVA. Same image, same `.pdata`, machine word retyped to
    /// MIPS R4000: only the entry point (a different oracle) survives.
    #[test]
    fn pdata_unsupported_machine_is_not_parsed() {
        let mut bytes = fixture("pe_pdata_arm64.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse arm64 PE");
        let before = pe_entry_candidates(&file, bytes.as_slice());
        assert!(before.contains(&0x140001000), "fixture lost its .pdata: {before:#x?}");

        let nt = u32::from_le_bytes([bytes[0x3c], bytes[0x3d], bytes[0x3e], bytes[0x3f]]) as usize;
        bytes[nt + 4..nt + 6].copy_from_slice(&0x0166u16.to_le_bytes());
        let file = object::File::parse(bytes.as_slice()).expect("parse retyped PE");
        let after = pe_entry_candidates(&file, bytes.as_slice());
        for vma in [0x140001000u64, 0x140001010, 0x140001020] {
            assert!(!after.contains(&vma), "{vma:#x} still parsed in {after:#x?}");
        }
        // 0x140001030 is `AddressOfEntryPoint`, which oracle 1 still supplies.
        assert_eq!(after, vec![0x140001030], "only the entry point survives: {after:#x?}");
    }
}
