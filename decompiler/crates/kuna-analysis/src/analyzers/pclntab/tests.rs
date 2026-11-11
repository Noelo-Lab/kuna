//! Hermetic unit tests for the Go pclntab parser.
//!
//! These build hand-constructed pclntab byte layouts (no `go` toolchain, no
//! fixture) and exercise the parser primitives end to end: header probe (magic /
//! endianness / validation), per-version header parsing, and the functab→`_func`
//! →`funcnametab` name decode for go1.18, go1.16, and go1.2. The real-Go e2e lives
//! in `kuna-console/tests/verify_go_pclntab.rs` (runtime `go build`, skips if
//! absent); this file is the merge-blocking gate that needs no `go`.

use super::*;

// --- small byte-builder helpers ----------------------------------------------

fn push_u32_le(v: &mut Vec<u8>, x: u32) {
    v.extend_from_slice(&x.to_le_bytes());
}
fn push_u64_le(v: &mut Vec<u8>, x: u64) {
    v.extend_from_slice(&x.to_le_bytes());
}
fn push_u32_be(v: &mut Vec<u8>, x: u32) {
    v.extend_from_slice(&x.to_be_bytes());
}
fn push_u64_be(v: &mut Vec<u8>, x: u64) {
    v.extend_from_slice(&x.to_be_bytes());
}

/// Build a NUL-terminated funcnametab blob and return `(blob, offsets)` where
/// `offsets[i]` is the byte offset of `names[i]` within the blob. Offset 0 is a
/// leading NUL (Go reserves nameoff 0 as the empty name), so real names start at 1.
fn build_nametab(names: &[&str]) -> (Vec<u8>, Vec<u32>) {
    let mut blob = vec![0u8]; // reserved empty-name at offset 0
    let mut offs = Vec::new();
    for n in names {
        offs.push(blob.len() as u32);
        blob.extend_from_slice(n.as_bytes());
        blob.push(0);
    }
    (blob, offs)
}

// --- header probe (magic / endian / validation) ------------------------------

#[test]
fn probe_recognizes_all_four_magics_little_endian() {
    for (magic, want) in [
        (MAGIC_GO_1_2, GoVer::Go12),
        (MAGIC_GO_1_16, GoVer::Go116),
        (MAGIC_GO_1_18, GoVer::Go118),
        (MAGIC_GO_1_20, GoVer::Go118),
    ] {
        let mut h = Vec::new();
        push_u32_le(&mut h, magic);
        h.extend_from_slice(&[0, 0, 1, 8]); // pad,pad,minLC=1,ptrSize=8
        let (ver, end, min_lc, ptr) = probe_header(&h).expect("valid header");
        assert_eq!(ver, want, "magic {magic:#x}");
        assert_eq!(end, End::Little);
        assert_eq!(min_lc, 1);
        assert_eq!(ptr, 8);
    }
}

#[test]
fn probe_detects_big_endian_via_magic() {
    let mut h = Vec::new();
    push_u32_be(&mut h, MAGIC_GO_1_18);
    h.extend_from_slice(&[0, 0, 4, 8]); // minLC=4, ptrSize=8
    let (ver, end, min_lc, ptr) = probe_header(&h).expect("BE header valid");
    assert_eq!(ver, GoVer::Go118);
    assert_eq!(end, End::Big);
    assert_eq!(min_lc, 4);
    assert_eq!(ptr, 8);
}

#[test]
fn probe_rejects_malformed_headers() {
    // unknown magic
    let mut h = Vec::new();
    push_u32_le(&mut h, 0xdead_beef);
    h.extend_from_slice(&[0, 0, 1, 8]);
    assert!(probe_header(&h).is_none(), "unknown magic rejected");

    // nonzero pad byte
    let mut h = Vec::new();
    push_u32_le(&mut h, MAGIC_GO_1_18);
    h.extend_from_slice(&[1, 0, 1, 8]);
    assert!(probe_header(&h).is_none(), "nonzero pad1 rejected");

    // bad minLC (3 is not in {1,2,4})
    let mut h = Vec::new();
    push_u32_le(&mut h, MAGIC_GO_1_18);
    h.extend_from_slice(&[0, 0, 3, 8]);
    assert!(probe_header(&h).is_none(), "bad minLC rejected");

    // bad ptrSize (16 not in {4,8})
    let mut h = Vec::new();
    push_u32_le(&mut h, MAGIC_GO_1_18);
    h.extend_from_slice(&[0, 0, 1, 16]);
    assert!(probe_header(&h).is_none(), "bad ptrSize rejected");

    // too short
    assert!(probe_header(&[0xf0, 0xff, 0xff]).is_none(), "short header rejected");
}

// --- go1.18 (64-bit LE) full decode ------------------------------------------

/// Build a complete, self-consistent go1.18 (64-bit LE) pclntab as a single blob
/// rooted at `header_vma`, with `text_start` as the entry base. Returns the blob.
/// Layout (all offsets relative to header start):
///   header (0x48 bytes), then funcnametab, then functab, then the `_func`s.
fn build_go118_image(_header_vma: u64, text_start: u64, funcs: &[(u32, &str)]) -> Vec<u8> {
    build_go118_image_end(text_start, funcs, None)
}

/// [`build_go118_image`] with the closing `ftab[nfunc]` entry (`etext - text`)
/// when `etext_off` is given.
fn build_go118_image_end(text_start: u64, funcs: &[(u32, &str)], etext_off: Option<u32>) -> Vec<u8> {
    let names: Vec<&str> = funcs.iter().map(|&(_, n)| n).collect();
    let (nametab, name_offs) = build_nametab(&names);

    // Header is 0x48 bytes (8 fixed + nfunc + nfiles + textStart + 5 offsets).
    const HDR_LEN: u64 = 0x48;
    let funcname_off = HDR_LEN;
    let functab_off = funcname_off + nametab.len() as u64;
    // functab: nfunc entries of (entryoff:u32, funcoff:u32) = 8 bytes each.
    let nfunc = funcs.len() as u64;
    let funcs_blob_off = functab_off + (nfunc + etext_off.is_some() as u64) * 8;

    // Lay out each `_func`: entryoff(u32) nameoff(i32) then padding to a few
    // fields (we only need the first 8 bytes). Give each 16 bytes of room.
    let func_stride = 16u64;

    let mut blob = Vec::new();
    // --- header ---
    push_u32_le(&mut blob, MAGIC_GO_1_18);
    blob.extend_from_slice(&[0, 0, 1, 8]); // pad,pad,minLC=1,ptrSize=8
    push_u64_le(&mut blob, nfunc); // nfunc
    push_u64_le(&mut blob, 0); // nfiles
    push_u64_le(&mut blob, text_start); // textStart
    push_u64_le(&mut blob, funcname_off); // funcnameOffset (rel header)
    push_u64_le(&mut blob, 0); // cuOffset
    push_u64_le(&mut blob, 0); // filetabOffset
    push_u64_le(&mut blob, 0); // pctabOffset
    push_u64_le(&mut blob, functab_off); // pclnOffset (rel header) -> functab base
    assert_eq!(blob.len() as u64, HDR_LEN);

    // --- funcnametab ---
    blob.extend_from_slice(&nametab);
    assert_eq!(blob.len() as u64, functab_off);

    // --- functab: (entryoff, funcoff) ---
    // funcoff is relative to the pclntab/funcdata base (= the functab base), NOT
    // the blob start, mirroring Go's `pclntable.array + funcoff`.
    for (i, &(entryoff, _)) in funcs.iter().enumerate() {
        push_u32_le(&mut blob, entryoff);
        let funcoff = (funcs_blob_off - functab_off) + i as u64 * func_stride;
        push_u32_le(&mut blob, funcoff as u32);
    }
    if let Some(end) = etext_off {
        push_u32_le(&mut blob, end);
        push_u32_le(&mut blob, 0);
    }
    assert_eq!(blob.len() as u64, funcs_blob_off);

    // --- the `_func` structs ---
    for (i, &(entryoff, _)) in funcs.iter().enumerate() {
        let start = blob.len();
        push_u32_le(&mut blob, entryoff); // _func.entryoff
        push_u32_le(&mut blob, name_offs[i]); // _func.nameoff
        // pad the rest of the stride
        blob.resize(start + func_stride as usize, 0);
    }
    blob
}

#[test]
fn decodes_go118_64bit_le() {
    let header_vma = 0x500000;
    let text_start = 0x401000;
    let funcs = [(0x10u32, "main.main"), (0x80u32, "main.compute"), (0x200u32, "runtime.gopanic")];
    let blob = build_go118_image(header_vma, text_start, &funcs);

    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).expect("header parses");
    assert_eq!(h.ver, GoVer::Go118);
    assert_eq!(h.ptr_size, 8);
    assert_eq!(h.text_start, text_start);
    assert_eq!(h.nfunc, 3);

    let got = decode_functions(&img, &h);
    assert_eq!(got.len(), 3, "all three functions decoded");
    // entry PC = text_start + entryoff; name from funcnametab.
    assert_eq!(got[0], GoFunc { addr: text_start + 0x10, name: "main.main".into() });
    assert_eq!(got[1], GoFunc { addr: text_start + 0x80, name: "main.compute".into() });
    assert_eq!(got[2], GoFunc { addr: text_start + 0x200, name: "runtime.gopanic".into() });
}

#[test]
fn go118_skips_inline_sentinel_entry() {
    let header_vma = 0x500000;
    let text_start = 0x401000;
    // Second entry is an inline sentinel (entryoff == 0xffffffff): must be skipped.
    let funcs = [(0x10u32, "main.main"), (u32::MAX, "inline.sentinel"), (0x200u32, "main.other")];
    let blob = build_go118_image(header_vma, text_start, &funcs);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).unwrap();
    let got = decode_functions(&img, &h);
    let names: Vec<&str> = got.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["main.main", "main.other"], "inline sentinel dropped");
}

#[test]
fn go118_ignores_buildid_sentinel_name() {
    let header_vma = 0x500000;
    let text_start = 0x401000;
    let funcs = [(0x10u32, "go:buildid"), (0x80u32, "main.main")];
    let blob = build_go118_image(header_vma, text_start, &funcs);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).unwrap();
    let got = decode_functions(&img, &h);
    assert_eq!(got.len(), 1, "go:buildid dropped");
    assert_eq!(got[0].name, "main.main");
}

// --- go1.16 (64-bit LE) decode (absolute entries, no textStart) --------------

fn build_go116_image(_header_vma: u64, funcs: &[(u64, &str)]) -> Vec<u8> {
    let names: Vec<&str> = funcs.iter().map(|&(_, n)| n).collect();
    let (nametab, name_offs) = build_nametab(&names);

    // 1.16 header: 8 fixed + nfunc + nfiles + 5 offsets (NO textStart) = 0x40.
    const HDR_LEN: u64 = 0x40;
    let funcname_off = HDR_LEN;
    let functab_off = funcname_off + nametab.len() as u64;
    let nfunc = funcs.len() as u64;
    // functab entry = (entry: u64, funcoff: u64) = 16 bytes.
    let funcs_blob_off = functab_off + nfunc * 16;
    let func_stride = 16u64; // _func: entry(u64) @0, nameoff(i32) @8

    let mut blob = Vec::new();
    push_u32_le(&mut blob, MAGIC_GO_1_16);
    blob.extend_from_slice(&[0, 0, 1, 8]);
    push_u64_le(&mut blob, nfunc);
    push_u64_le(&mut blob, 0); // nfiles
    push_u64_le(&mut blob, funcname_off);
    push_u64_le(&mut blob, 0); // cuOffset
    push_u64_le(&mut blob, 0); // filetabOffset
    push_u64_le(&mut blob, 0); // pctabOffset
    push_u64_le(&mut blob, functab_off); // pclnOffset
    assert_eq!(blob.len() as u64, HDR_LEN);

    blob.extend_from_slice(&nametab);
    // functab: (entry absolute, funcoff). funcoff is relative to the functab base.
    for (i, &(entry, _)) in funcs.iter().enumerate() {
        push_u64_le(&mut blob, entry);
        let funcoff = (funcs_blob_off - functab_off) + i as u64 * func_stride;
        push_u64_le(&mut blob, funcoff);
    }
    // the _func structs: entry(u64) @0, nameoff(i32) @8
    for (i, &(entry, _)) in funcs.iter().enumerate() {
        let start = blob.len();
        push_u64_le(&mut blob, entry); // _func.entry
        push_u32_le(&mut blob, name_offs[i]); // _func.nameoff
        blob.resize(start + func_stride as usize, 0);
    }
    blob
}

#[test]
fn decodes_go116_64bit_le_absolute_entries() {
    let header_vma = 0x600000;
    let funcs = [(0x401000u64, "main.main"), (0x401200u64, "fmt.Println")];
    let blob = build_go116_image(header_vma, &funcs);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).expect("1.16 header parses");
    assert_eq!(h.ver, GoVer::Go116);
    assert_eq!(h.text_start, 0, "1.16 header has no textStart");
    let got = decode_functions(&img, &h);
    assert_eq!(got.len(), 2);
    // entries are absolute (no text_start add).
    assert_eq!(got[0], GoFunc { addr: 0x401000, name: "main.main".into() });
    assert_eq!(got[1], GoFunc { addr: 0x401200, name: "fmt.Println".into() });
}

// --- go1.2 legacy (64-bit LE) decode -----------------------------------------

fn build_go12_image(_header_vma: u64, funcs: &[(u64, &str)]) -> Vec<u8> {
    // go1.2 layout: 8-byte header, nfunc(u64) @8, then functab (entry,funcoff)
    // pairs @16. Names live in the pclntab blob itself (funcname base = blob base);
    // nameoff in each _func indexes the whole blob. _func: entry(u64) @0,
    // nameoff(i32) @8.
    let names: Vec<&str> = funcs.iter().map(|&(_, n)| n).collect();
    let (nametab, raw_name_offs) = build_nametab(&names);

    let nfunc = funcs.len() as u64;
    let functab_off = 16u64;
    let funcs_blob_off = functab_off + nfunc * 16;
    let func_stride = 16u64;
    // The nametab is placed AFTER the _func structs; nameoff indexes the whole
    // blob (relative to blob base = header_vma).
    let nametab_off = funcs_blob_off + nfunc * func_stride;

    let mut blob = Vec::new();
    push_u32_le(&mut blob, MAGIC_GO_1_2);
    blob.extend_from_slice(&[0, 0, 1, 8]);
    push_u64_le(&mut blob, nfunc); // nfunc @8
    // functab (entry, funcoff)
    for (i, &(entry, _)) in funcs.iter().enumerate() {
        push_u64_le(&mut blob, entry);
        let funcoff = funcs_blob_off + i as u64 * func_stride;
        push_u64_le(&mut blob, funcoff);
    }
    // _func structs
    for (i, &(entry, _)) in funcs.iter().enumerate() {
        let start = blob.len();
        push_u64_le(&mut blob, entry); // _func.entry
        // nameoff = offset of the name within the whole blob.
        push_u32_le(&mut blob, nametab_off as u32 + raw_name_offs[i]);
        blob.resize(start + func_stride as usize, 0);
    }
    assert_eq!(blob.len() as u64, nametab_off);
    blob.extend_from_slice(&nametab);
    blob
}

#[test]
fn decodes_go12_legacy_64bit_le() {
    let header_vma = 0x700000;
    let funcs = [(0x401000u64, "main.main"), (0x401100u64, "runtime.main")];
    let blob = build_go12_image(header_vma, &funcs);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).expect("go1.2 header parses");
    assert_eq!(h.ver, GoVer::Go12);
    assert_eq!(h.nfunc, 2);
    // For go1.2 the funcname base IS the pclntab blob base.
    assert_eq!(h.funcname_vma, header_vma);
    let got = decode_functions(&img, &h);
    assert_eq!(got.len(), 2);
    assert_eq!(got[0], GoFunc { addr: 0x401000, name: "main.main".into() });
    assert_eq!(got[1], GoFunc { addr: 0x401100, name: "runtime.main".into() });
}

// --- endianness ---------------------------------------------------------------

fn build_go118_image_be(_header_vma: u64, text_start: u64, funcs: &[(u32, &str)]) -> Vec<u8> {
    let names: Vec<&str> = funcs.iter().map(|&(_, n)| n).collect();
    let (nametab, name_offs) = build_nametab(&names);
    const HDR_LEN: u64 = 0x48;
    let funcname_off = HDR_LEN;
    let functab_off = funcname_off + nametab.len() as u64;
    let nfunc = funcs.len() as u64;
    let funcs_blob_off = functab_off + nfunc * 8;
    let func_stride = 16u64;

    let mut blob = Vec::new();
    push_u32_be(&mut blob, MAGIC_GO_1_18);
    blob.extend_from_slice(&[0, 0, 1, 8]);
    push_u64_be(&mut blob, nfunc);
    push_u64_be(&mut blob, 0);
    push_u64_be(&mut blob, text_start);
    push_u64_be(&mut blob, funcname_off);
    push_u64_be(&mut blob, 0);
    push_u64_be(&mut blob, 0);
    push_u64_be(&mut blob, 0);
    push_u64_be(&mut blob, functab_off);
    blob.extend_from_slice(&nametab);
    for (i, &(entryoff, _)) in funcs.iter().enumerate() {
        push_u32_be(&mut blob, entryoff);
        // funcoff relative to the functab base.
        push_u32_be(&mut blob, ((funcs_blob_off - functab_off) + i as u64 * func_stride) as u32);
    }
    for (i, &(entryoff, _)) in funcs.iter().enumerate() {
        let start = blob.len();
        push_u32_be(&mut blob, entryoff);
        push_u32_be(&mut blob, name_offs[i]);
        blob.resize(start + func_stride as usize, 0);
    }
    blob
}

#[test]
fn decodes_go118_big_endian() {
    let header_vma = 0x500000;
    let text_start = 0x10000;
    let funcs = [(0x10u32, "main.main")];
    let blob = build_go118_image_be(header_vma, text_start, &funcs);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).expect("BE header parses");
    assert_eq!(h.end, End::Big);
    let got = decode_functions(&img, &h);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0], GoFunc { addr: text_start + 0x10, name: "main.main".into() });
}

// --- defensiveness / never-fail ----------------------------------------------

#[test]
fn unknown_magic_yields_empty_via_locate_miss() {
    // A header-shaped buffer with a bad magic: parse_header returns None.
    let header_vma = 0x500000;
    let mut blob = Vec::new();
    push_u32_le(&mut blob, 0x1234_5678);
    blob.extend_from_slice(&[0, 0, 1, 8]);
    blob.resize(0x48, 0);
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    assert!(parse_header(&img, header_vma).is_none(), "unknown magic -> None");
}

#[test]
fn truncated_table_does_not_panic() {
    // A valid header that claims 100 funcs but the functab bytes are absent: the
    // decode must stop cleanly (never panic / never read OOB).
    let header_vma = 0x500000;
    let mut blob = Vec::new();
    push_u32_le(&mut blob, MAGIC_GO_1_18);
    blob.extend_from_slice(&[0, 0, 1, 8]);
    push_u64_le(&mut blob, 100); // nfunc = 100 (a lie)
    push_u64_le(&mut blob, 0); // nfiles
    push_u64_le(&mut blob, 0x1000); // textStart
    push_u64_le(&mut blob, 0x48); // funcnameOffset
    push_u64_le(&mut blob, 0);
    push_u64_le(&mut blob, 0);
    push_u64_le(&mut blob, 0);
    push_u64_le(&mut blob, 0x100); // pclnOffset -> functab base (past the blob)
    // No funcnametab/functab bytes follow.
    let img = ImageBytes::from_spans(vec![(header_vma, blob.as_slice())]);
    let h = parse_header(&img, header_vma).expect("header itself is valid");
    let got = decode_functions(&img, &h); // must not panic
    assert!(got.is_empty(), "no decodable entries -> empty");
}

#[test]
fn image_read_cstr_bounds() {
    let data: &[u8] = b"\0main.main\0extra";
    let img = ImageBytes::from_spans(vec![(0x1000, data)]);
    // offset 1 -> "main.main"
    assert_eq!(img.read_cstr(0x1001, 64).as_deref(), Some("main.main"));
    // offset 0 -> empty string (leading NUL)
    assert_eq!(img.read_cstr(0x1000, 64).as_deref(), Some(""));
    // unmapped address -> None
    assert_eq!(img.read_cstr(0x9999, 64), None);
}
// --- Mach-O container --------------------------------------------------------
//
// The pass returned before parsing on any non-ELF view, and `locate_header` only
// knew the ELF section spellings. This builds a real arm64 Mach-O (no toolchain, no
// fixture) whose `__DATA_CONST,__gopclntab` holds the same hand-built table the
// ELF-shaped tests use, and drives recovery through `object::File` as the pass does.

fn push_name16(v: &mut Vec<u8>, s: &str) {
    let mut name = [0u8; 16];
    name[..s.len()].copy_from_slice(s.as_bytes());
    v.extend_from_slice(&name);
}

/// One `LC_SEGMENT_64` holding exactly one section, `(addr, size)` at `offset`.
fn push_segment64(
    v: &mut Vec<u8>,
    seg: &str,
    sect: &str,
    addr: u64,
    size: u64,
    offset: u64,
    sect_flags: u32,
) {
    push_u32_le(v, 0x19); // LC_SEGMENT_64
    push_u32_le(v, 72 + 80); // cmdsize
    push_name16(v, seg);
    push_u64_le(v, addr);
    push_u64_le(v, size);
    push_u64_le(v, offset);
    push_u64_le(v, size);
    push_u32_le(v, 7);
    push_u32_le(v, 5); // initprot r-x
    push_u32_le(v, 1); // nsects
    push_u32_le(v, 0); // flags
    push_name16(v, sect);
    push_name16(v, seg);
    push_u64_le(v, addr);
    push_u64_le(v, size);
    push_u32_le(v, offset as u32);
    push_u32_le(v, 4); // align 2^4
    v.resize(v.len() + 8, 0); // reloff, nreloc
    push_u32_le(v, sect_flags);
    v.resize(v.len() + 12, 0); // reserved1..3
}

/// A minimal `MH_EXECUTE` arm64 Mach-O with one single-section segment per
/// `(segment, section, vma, bytes, flags)`.
fn build_macho(secs: &[(&str, &str, u64, &[u8], u32)]) -> Vec<u8> {
    let mut off = 0x1000u64;
    let mut v = Vec::new();
    push_u32_le(&mut v, 0xfeed_facf); // MH_MAGIC_64
    push_u32_le(&mut v, 0x0100_000c); // CPU_TYPE_ARM64
    push_u32_le(&mut v, 0); // cpusubtype
    push_u32_le(&mut v, 2); // MH_EXECUTE
    push_u32_le(&mut v, secs.len() as u32); // ncmds
    push_u32_le(&mut v, (72 + 80) * secs.len() as u32); // sizeofcmds
    v.resize(32, 0); // flags, reserved
    let mut offs = Vec::new();
    for &(seg, sect, vma, bytes, flags) in secs {
        push_segment64(&mut v, seg, sect, vma, bytes.len() as u64, off, flags);
        offs.push(off);
        off += (bytes.len() as u64 + 0xf) & !0xf;
    }
    for (&(_, _, _, bytes, _), &o) in secs.iter().zip(&offs) {
        v.resize(o as usize, 0);
        v.extend_from_slice(bytes);
    }
    v
}

/// [`build_macho`] with executable `__TEXT,__text` at `text_vma` and
/// `__DATA_CONST,__gopclntab` holding `blob` at `pcln_vma`.
fn build_macho_with_pclntab(text_vma: u64, text_size: u64, pcln_vma: u64, blob: &[u8]) -> Vec<u8> {
    let text = vec![0u8; text_size as usize];
    build_macho(&[
        ("__TEXT", "__text", text_vma, &text, 0x8000_0400),
        ("__DATA_CONST", "__gopclntab", pcln_vma, blob, 0),
    ])
}

#[test]
fn macho_gopclntab_section_is_found_and_decoded() {
    let text_vma = 0x1_0000_1000u64;
    let funcs = [(0x10u32, "main.main"), (0x80u32, "main.compute"), (0x200u32, "runtime.gopanic")];
    let blob = build_go118_image(0, text_vma, &funcs);
    let bytes = build_macho_with_pclntab(text_vma, 0x400, 0x1_0001_0000, &blob);
    let file = object::File::parse(&bytes[..]).expect("hand-built Mach-O must parse");
    assert_eq!(file.format(), object::BinaryFormat::MachO);
    assert_eq!(
        recover_go_functions(&file),
        vec![
            GoFunc { addr: text_vma + 0x10, name: "main.main".into() },
            GoFunc { addr: text_vma + 0x80, name: "main.compute".into() },
            GoFunc { addr: text_vma + 0x200, name: "runtime.gopanic".into() },
        ]
    );
}

#[test]
fn zero_text_start_resolves_against_the_text_section() {
    // Newer Go writes `textStart = 0` in the header; the entries must still land in
    // `__text` (Go's `moduledata.text` is the section base).
    let text_vma = 0x1_0000_1000u64;
    let funcs = [(0x0u32, "main.main"), (0x80u32, "main.compute")];
    let blob = build_go118_image(0, 0, &funcs);
    let bytes = build_macho_with_pclntab(text_vma, 0x400, 0x1_0001_0000, &blob);
    let file = object::File::parse(&bytes[..]).expect("hand-built Mach-O must parse");
    assert_eq!(
        recover_go_functions(&file),
        vec![
            GoFunc { addr: text_vma, name: "main.main".into() },
            GoFunc { addr: text_vma + 0x80, name: "main.compute".into() },
        ]
    );
}

#[test]
fn exec_end_bounds_only_interior_addresses() {
    let ranges = [(0x1000u64, 0x10u64)];
    assert_eq!(exec_end(&ranges, 0x1000), Some(0x1010));
    assert_eq!(exec_end(&ranges, 0x1004), Some(0x1010));
    assert_eq!(exec_end(&ranges, 0x1010), None);
    assert_eq!(exec_end(&ranges, 0x0fff), None);
    assert_eq!(exec_end(&[], 0x1000), None);
}

#[test]
fn zero_text_start_resolves_against_moduledata_text() {
    // An external link places host code ahead of Go's: `moduledata.text` (here
    // `__text + 0x400`), not the section base, is what `entryoff` is relative to.
    let text_vma = 0x1_0000_1000u64;
    let go_text = text_vma + 0x400;
    let pcln_vma = 0x1_0001_0000u64;
    let funcs = [(0x0u32, "main.main"), (0x80u32, "main.compute")];
    let blob = build_go118_image_end(0, &funcs, Some(0x100));
    let mut md = Vec::new();
    for w in 0..24u64 {
        push_u64_le(
            &mut md,
            match w {
                0 => pcln_vma,
                17 => funcs.len() as u64 + 1,
                20 | 22 => go_text,
                21 | 23 => go_text + 0x100,
                _ => 0,
            },
        );
    }
    let text = vec![0u8; 0x800];
    let bytes = build_macho(&[
        ("__TEXT", "__text", text_vma, &text, 0x8000_0400),
        ("__DATA_CONST", "__gopclntab", pcln_vma, &blob, 0),
        ("__DATA", "__go_module", 0x1_0002_0000, &md, 0),
    ]);
    let file = object::File::parse(&bytes[..]).expect("hand-built Mach-O must parse");
    assert_eq!(
        recover_go_table(&file),
        (
            vec![
                GoFunc { addr: go_text, name: "main.main".into() },
                GoFunc { addr: go_text + 0x80, name: "main.compute".into() },
            ],
            Some(go_text + 0x100)
        )
    );
}

#[test]
fn macho_lrodata_gopcln_section_is_found_and_decoded() {
    let text_vma = 0x1_0000_11c0u64;
    let funcs = [(0x10u32, "main.main"), (0x140u32, "main.helper")];
    let blob = build_go118_image(0, 0, &funcs);
    let text = vec![0u8; 0x400];
    let bytes = build_macho(&[
        ("__TEXT", "__text", text_vma, &text, 0x8000_0400),
        ("__TEXT", "__lrodata_gopcln", 0x1_03eb_2ea0, &blob, 0),
    ]);
    let file = object::File::parse(&bytes[..]).expect("hand-built Mach-O must parse");
    assert_eq!(
        recover_go_functions(&file),
        vec![
            GoFunc { addr: text_vma + 0x10, name: "main.main".into() },
            GoFunc { addr: text_vma + 0x140, name: "main.helper".into() },
        ]
    );
}

// --- ELF relro container -----------------------------------------------------
//
// An internally linked go1.25-or-older ELF PIE keeps the table in
// `.data.rel.ro.gopclntab`, the relro spelling of `.gopclntab`, which no scanned
// section covers: only the name finds it.

/// A minimal x86-64 `ET_DYN` ELF: executable `.text` at `text_vma`, then one
/// writable PROGBITS section `name` holding `blob` at `vma`.
fn build_elf(text_vma: u64, text_size: usize, name: &str, vma: u64, blob: &[u8]) -> Vec<u8> {
    use object::elf::*;
    use object::write::elf::{FileHeader, SectionHeader, Writer};
    let mut buf = Vec::new();
    let mut w = Writer::new(object::Endianness::Little, true, &mut buf);
    w.reserve_file_header();
    w.reserve_null_section_index();
    let text_name = w.add_section_name(b".text");
    let blob_name = w.add_section_name(name.as_bytes());
    w.reserve_section_index();
    w.reserve_section_index();
    w.reserve_shstrtab_section_index();
    let text_off = w.reserve(text_size, 16);
    let blob_off = w.reserve(blob.len(), 16);
    w.reserve_shstrtab();
    w.reserve_section_headers();
    w.write_file_header(&FileHeader {
        os_abi: ELFOSABI_NONE,
        abi_version: 0,
        e_type: ET_DYN,
        e_machine: EM_X86_64,
        e_entry: text_vma,
        e_flags: 0,
    })
    .expect("file header");
    w.write_align(16);
    w.write(&vec![0xc3; text_size]);
    w.write_align(16);
    w.write(blob);
    w.write_shstrtab();
    w.write_null_section_header();
    let progbits = |name, flags: u32, sh_addr, sh_offset: usize, sh_size: usize| SectionHeader {
        name: Some(name),
        sh_type: SHT_PROGBITS,
        sh_flags: (SHF_ALLOC | flags) as u64,
        sh_addr,
        sh_offset: sh_offset as u64,
        sh_size: sh_size as u64,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 16,
        sh_entsize: 0,
    };
    w.write_section_header(&progbits(text_name, SHF_EXECINSTR, text_vma, text_off, text_size));
    w.write_section_header(&progbits(blob_name, SHF_WRITE, vma, blob_off, blob.len()));
    w.write_shstrtab_section_header();
    buf
}

#[test]
fn elf_relro_gopclntab_section_is_found_and_decoded() {
    let text_vma = 0x1000u64;
    let funcs = [(0x10u32, "main.main"), (0x80u32, "main.compute"), (0x200u32, "runtime.gopanic")];
    let blob = build_go118_image(0, text_vma, &funcs);
    let bytes = build_elf(text_vma, 0x400, ".data.rel.ro.gopclntab", 0x13_5100, &blob);
    let file = object::File::parse(&bytes[..]).expect("hand-built ELF must parse");
    assert_eq!(file.format(), object::BinaryFormat::Elf);
    assert_eq!(
        recover_go_functions(&file),
        vec![
            GoFunc { addr: text_vma + 0x10, name: "main.main".into() },
            GoFunc { addr: text_vma + 0x80, name: "main.compute".into() },
            GoFunc { addr: text_vma + 0x200, name: "runtime.gopanic".into() },
        ]
    );
}

// A Go PE has no pclntab section (the Windows linker keeps the table inside
// `.rdata`) and no build-info section (the blob heads `.data`).

const PE_BASE: u64 = 0x1_4000_0000;

/// A minimal x86-64 PE with `.text`, `.rdata`, `.data` in that order.
fn build_pe(text: &[u8], rdata: &[u8], data: &[u8]) -> Vec<u8> {
    use object::pe::*;
    use object::write::pe::{NtHeaders, Writer};
    let mut buf = Vec::new();
    let mut w = Writer::new(true, 0x1000, 0x200, &mut buf);
    w.reserve_dos_header_and_stub();
    w.reserve_nt_headers(16);
    w.reserve_section_headers(3);
    let t = w.reserve_text_section(text.len() as u32);
    let r = w.reserve_rdata_section(rdata.len() as u32);
    let d = w.reserve_data_section(data.len() as u32, data.len() as u32);
    w.write_dos_header_and_stub().expect("dos header");
    w.write_nt_headers(NtHeaders {
        machine: IMAGE_FILE_MACHINE_AMD64,
        time_date_stamp: 0,
        characteristics: IMAGE_FILE_EXECUTABLE_IMAGE | IMAGE_FILE_LARGE_ADDRESS_AWARE,
        major_linker_version: 3,
        minor_linker_version: 0,
        address_of_entry_point: t.virtual_address,
        image_base: PE_BASE,
        major_operating_system_version: 6,
        minor_operating_system_version: 1,
        major_image_version: 1,
        minor_image_version: 0,
        major_subsystem_version: 6,
        minor_subsystem_version: 1,
        subsystem: IMAGE_SUBSYSTEM_WINDOWS_CUI,
        dll_characteristics: 0,
        size_of_stack_reserve: 0x20_0000,
        size_of_stack_commit: 0x1000,
        size_of_heap_reserve: 0x10_0000,
        size_of_heap_commit: 0x1000,
    });
    w.write_section_headers();
    w.write_section(t.file_offset, text);
    w.write_section(r.file_offset, rdata);
    w.write_section(d.file_offset, data);
    buf
}

/// The `(.text, .rdata, .data)` VMAs [`build_pe`] assigns to sections of these
/// sizes, so contents can point at each other before the image is written.
fn pe_vmas(text: usize, rdata: usize, data: usize) -> (u64, u64, u64) {
    let bytes = build_pe(&vec![0; text], &vec![0; rdata], &vec![0; data]);
    let file = object::File::parse(&bytes[..]).expect("hand-built PE must parse");
    let at = |n: &str| file.section_by_name(n).expect("section").address();
    (at(".text"), at(".rdata"), at(".data"))
}

/// A go1.18+ inline build-info blob (`cmd/link`'s `buildinfo`), 16-byte padded.
fn buildinfo(version: &str) -> Vec<u8> {
    let mut b = b"\xff Go buildinf:".to_vec();
    b.extend_from_slice(&[8, 2]);
    b.resize(32, 0);
    b.push(version.len() as u8);
    b.extend_from_slice(version.as_bytes());
    b.push(0);
    b.resize(b.len().next_multiple_of(16), 0);
    b
}

/// `.rdata` = `lead` bytes of other rodata, then the pclntab.
fn rdata_with_table(lead: &[u8], table: &[u8]) -> Vec<u8> {
    let mut r = lead.to_vec();
    r.resize(r.len().next_multiple_of(8), 0);
    r.extend_from_slice(table);
    r
}

const PE_FUNCS: [(u32, &str); 3] = [(0x10, "main.main"), (0x80, "main.compute"), (0x200, "runtime.gopanic")];

/// The head of a go1.16+ `moduledata`: the `pcHeader` pointer, then the
/// `funcnametab` slice (the test tables put it right after their 0x48-byte
/// header), padded to the 24 words the runtime record spans.
fn moduledata(header_vma: u64) -> Vec<u8> {
    let mut m = Vec::new();
    push_u64_le(&mut m, header_vma);
    push_u64_le(&mut m, header_vma + 0x48);
    m.resize(24 * 8, 0);
    m
}

/// A Go PE (go1.18..go1.25 shape: `textStart` set in the header) with a
/// magic-shaped 8-byte decoy ahead of the table in `.rdata`, and the
/// `moduledata` naming that table in `.data`.
fn go_pe(blob_in_data: bool, table_in_rdata: bool) -> (Vec<u8>, u64) {
    let decoy = [0xf1, 0xff, 0xff, 0xff, 0, 0, 1, 8, 0xff, 0xff, 0xff, 0xff];
    let probe = build_go118_image(0, 0, &PE_FUNCS);
    let rlen = rdata_with_table(&decoy, &probe).len();
    let (text_vma, rdata_vma, _) = pe_vmas(0x400, rlen, 0x100);
    let table = build_go118_image(0, text_vma, &PE_FUNCS);
    let mut data = if blob_in_data { buildinfo("go1.21.3") } else { Vec::new() };
    let rdata = if table_in_rdata {
        data.extend(moduledata(rdata_vma + (rlen - table.len()) as u64));
        rdata_with_table(&decoy, &table)
    } else {
        data = rdata_with_table(&data, &table);
        vec![0; rlen]
    };
    data.resize(data.len().max(0x100), 0);
    (build_pe(&[0xc3; 0x400], &rdata, &data), text_vma)
}

#[test]
fn pe_rdata_pclntab_is_found_past_a_decoy_and_decoded() {
    let (bytes, text_vma) = go_pe(true, true);
    let file = object::File::parse(&bytes[..]).expect("hand-built PE must parse");
    assert_eq!(
        recover_go_functions(&file),
        PE_FUNCS.iter().map(|&(o, n)| GoFunc { addr: text_vma + o as u64, name: n.into() }).collect::<Vec<_>>()
    );
}

#[test]
fn pe_own_table_wins_over_an_embedded_go_image() {
    // A `//go:embed`ded Go program sits in the same `.rdata`, ahead of the
    // program's own table and just as plausible; only the own table has a
    // `moduledata` pointing at it.
    const FOREIGN: [(u32, &str); 2] = [(0x20, "payload.main"), (0x40, "payload.run")];
    let probe = rdata_with_table(&build_go118_image(0, 0, &FOREIGN), &build_go118_image(0, 0, &PE_FUNCS));
    let (text_vma, rdata_vma, _) = pe_vmas(0x400, probe.len(), 0x100);
    let own = build_go118_image(0, text_vma, &PE_FUNCS);
    let rdata = rdata_with_table(&build_go118_image(0, text_vma, &FOREIGN), &own);
    let mut data = buildinfo("go1.21.3");
    data.extend(moduledata(rdata_vma + (rdata.len() - own.len()) as u64));
    data.resize(0x100, 0);
    let bytes = build_pe(&[0xc3; 0x400], &rdata, &data);
    let file = object::File::parse(&bytes[..]).expect("hand-built PE must parse");
    assert_eq!(
        recover_go_functions(&file),
        PE_FUNCS.iter().map(|&(o, n)| GoFunc { addr: text_vma + o as u64, name: n.into() }).collect::<Vec<_>>()
    );
    assert_eq!(crate::sourcelang::detect_compiler(&file, &bytes), crate::sourcelang::Compiler::Go);
}

#[test]
fn pe_zero_text_start_resolves_against_moduledata_in_data() {
    // go1.26+: the header's `textStart` is 0 and the runtime reads
    // `moduledata.text`, which the Windows linker keeps in `.data`.
    let probe = build_go118_image_end(0, &PE_FUNCS, Some(0x300));
    let (text_vma, rdata_vma, _) = pe_vmas(0x400, probe.len(), 0x100 + 24 * 8);
    let mut data = buildinfo("go1.26.1");
    for w in 0..24u64 {
        push_u64_le(
            &mut data,
            match w {
                0 => rdata_vma,
                17 => PE_FUNCS.len() as u64 + 1,
                20 | 22 => text_vma,
                21 | 23 => text_vma + 0x300,
                _ => 0,
            },
        );
    }
    data.resize(0x100 + 24 * 8, 0);
    let bytes = build_pe(&[0xc3; 0x400], &probe, &data);
    let file = object::File::parse(&bytes[..]).expect("hand-built PE must parse");
    let (funcs, end) = recover_go_table(&file);
    assert_eq!(funcs.iter().map(|f| f.addr).collect::<Vec<_>>(), vec![text_vma + 0x10, text_vma + 0x80, text_vma + 0x200]);
    assert_eq!(end, Some(text_vma + 0x300));
}

#[test]
fn pe_is_go_only_with_its_own_buildinfo_and_readonly_table() {
    let detect = |bytes: &[u8]| {
        let file = object::File::parse(bytes).expect("hand-built PE must parse");
        crate::sourcelang::detect_compiler(&file, bytes)
    };
    assert_eq!(detect(&go_pe(true, true).0), crate::sourcelang::Compiler::Go);
    // A table with no build info, build info with no table, and a C program
    // carrying a whole Go image in its writable data are not Go programs.
    assert!(!detect(&go_pe(false, true).0).is_golang());
    assert!(!detect(&build_pe(&[0xc3; 0x400], &[0; 0x100], &buildinfo("go1.21.3"))).is_golang());
    assert!(!detect(&go_pe(true, false).0).is_golang());
    // The same Go image carried twice, a const copy in `.rdata` and a writable
    // one in `.data`: blob and table are both there, but the `.data` copy's
    // `moduledata` names the address the table had in its own image.
    let (text_vma, rdata_vma, _) = pe_vmas(0x400, 0x200, 0x200);
    let rdata = rdata_with_table(&[], &build_go118_image(0, text_vma, &PE_FUNCS));
    let mut data = buildinfo("go1.21.3");
    data.extend(moduledata(rdata_vma + 0x7000));
    data.resize(0x200, 0);
    assert!(!detect(&build_pe(&[0xc3; 0x400], &rdata, &data)).is_golang());
    // The bare magic string, as a C `char[]` would carry it.
    let (text, rdata) = (vec![0xc3; 0x400], go_pe(true, true));
    let file = object::File::parse(&rdata.0[..]).expect("parse");
    let rd = file.section_by_name(".rdata").unwrap().data().unwrap().to_vec();
    let mut bare = b"\xff Go buildinf:\0".to_vec();
    bare.resize(0x100, 0);
    assert!(!detect(&build_pe(&text, &rd, &bare)).is_golang());
}
