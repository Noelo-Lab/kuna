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
    let names: Vec<&str> = funcs.iter().map(|&(_, n)| n).collect();
    let (nametab, name_offs) = build_nametab(&names);

    // Header is 0x48 bytes (8 fixed + nfunc + nfiles + textStart + 5 offsets).
    const HDR_LEN: u64 = 0x48;
    let funcname_off = HDR_LEN;
    let functab_off = funcname_off + nametab.len() as u64;
    // functab: nfunc entries of (entryoff:u32, funcoff:u32) = 8 bytes each.
    let nfunc = funcs.len() as u64;
    let funcs_blob_off = functab_off + nfunc * 8;

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
