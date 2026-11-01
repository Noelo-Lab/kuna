//! `kuna read --json`'s `provenance` (GH #1401): the listed span cut where its
//! bytes stop being the file's and become the loader's zero fill, each range
//! naming the segment that holds it and the permissions that segment declares.
//! The bytes themselves do not change.
//!
//! Fixtures: `readprov_macho_arm64`, `readprov_elf_x86_64` and
//! `readprov_pe_x86_64.exe` (see `readprov.provenance`), `macho_fat`, and the
//! relocatable `entry_selectors_x86_64.o`. Integration tests require the built
//! processor specs under `specs/`.

use crate::common;

fn read_in(binary: &str, start: &str, bytes: &str, extra: &[&str]) -> (serde_json::Value, String) {
    read_path(&common::fixture(binary), start, bytes, extra)
}

fn read_path(path: &str, start: &str, bytes: &str, extra: &[&str]) -> (serde_json::Value, String) {
    let mut argv = vec!["read", &path, start, "--addr", "--bytes", bytes, "--json"];
    argv.extend_from_slice(extra);
    let (stdout, stderr, rc) = common::run_kuna(&argv);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_tiles_and_matches_the_file(&doc, path);
    (doc, stderr)
}

fn read(binary: &str, start: &str, bytes: &str) -> serde_json::Value {
    read_in(binary, start, bytes, &[]).0
}

/// The ranges tile the listed span exactly, a `file` range's bytes are the
/// file's at its offset, a `repaired` range's are not, and a `zero-fill`
/// range reads zero.
fn assert_tiles_and_matches_the_file(doc: &serde_json::Value, path: &str) {
    let file = std::fs::read(path).unwrap();
    let listed = hex_bytes(doc["hex"].as_str().unwrap());
    let start = doc["start"].as_u64().unwrap();
    let mut at = start;
    for r in doc["provenance"].as_array().unwrap() {
        let (lo, hi) = (r["start"].as_u64().unwrap(), r["end"].as_u64().unwrap());
        assert_eq!(lo, at, "{doc}");
        assert_eq!(r["size"].as_u64().unwrap(), hi - lo);
        let got = &listed[(lo - start) as usize..(hi - start) as usize];
        match r["source"].as_str().unwrap() {
            "file" => {
                let off = r["file_offset"].as_u64().unwrap() as usize;
                assert_eq!(got, &file[off..off + got.len()], "{r}");
            }
            "repaired" => {
                let off = r["file_offset"].as_u64().unwrap() as usize;
                assert!(got.iter().zip(&file[off..off + got.len()]).all(|(a, b)| a != b), "{r}");
            }
            "zero-fill" => assert!(got.iter().all(|&b| b == 0), "{r}"),
            other => assert_eq!(other, "unknown"),
        }
        at = hi;
    }
    assert_eq!(at, doc["end"].as_u64().unwrap(), "{doc}");
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
}

/// `(start, end, source, file_offset, segment name, permissions)`.
type Row = (u64, u64, String, Option<u64>, Option<String>, Option<String>);

fn ranges(doc: &serde_json::Value) -> Vec<Row> {
    let text = |v: &serde_json::Value| v.as_str().map(str::to_owned);
    doc["provenance"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["start"].as_u64().unwrap(),
                r["end"].as_u64().unwrap(),
                text(&r["source"]).unwrap(),
                r["file_offset"].as_u64(),
                text(&r["segment"]["name"]),
                text(&r["segment"]["permissions"]),
            )
        })
        .collect()
}

fn row(start: u64, end: u64, source: &str, offset: Option<u64>, name: Option<&str>, perms: &str) -> Row {
    (start, end, source.to_owned(), offset, name.map(str::to_owned), Some(perms.to_owned()))
}

fn notes(doc: &serde_json::Value) -> Vec<String> {
    doc["notes"].as_array().unwrap().iter().map(|n| n.as_str().unwrap().to_owned()).collect()
}

/// The issue's first read: `__PAGEZERO` holds nothing in the file and grants
/// no access, so its zeroes are zero fill in an unreadable segment, on stderr
/// as well as in `notes`.
#[test]
fn pagezero_is_zero_fill_with_no_access() {
    let (doc, stderr) = read_in("readprov_macho_arm64", "0x0", "16", &[]);
    assert_eq!(doc["hex"], "00000000000000000000000000000000");
    assert_eq!(doc["truncated"], false);
    assert_eq!(ranges(&doc), [row(0, 0x10, "zero-fill", None, Some("__PAGEZERO"), "---")]);
    let seg = &doc["provenance"][0]["segment"];
    assert_eq!((seg["start"].as_u64(), seg["size"].as_u64()), (Some(0), Some(0x1_0000_0000)));
    assert_eq!((seg["file_offset"].as_u64(), seg["file_size"].as_u64()), (Some(0), Some(0)));
    let notes = notes(&doc);
    assert!(notes.iter().any(|n| n.starts_with("zero fill, not file bytes: 0x0..0x10 in __PAGEZERO")), "{notes:?}");
    assert!(notes.iter().any(|n| n.starts_with("no read access: 0x0..0x10 in __PAGEZERO (permissions ---)")), "{notes:?}");
    assert!(stderr.contains("no read access"), "{stderr}");
}

/// The issue's second read: `__LINKEDIT` declares a page of memory over 0x2e0
/// file bytes, so a read across that end is part file, part zero fill.
#[test]
fn a_read_past_linkedits_file_content_is_cut_where_it_ends() {
    let doc = read("readprov_macho_arm64", "0x1000102d8", "16");
    assert_eq!(
        ranges(&doc),
        [
            row(0x1000102d8, 0x1000102e0, "file", Some(0x82d8), Some("__LINKEDIT"), "r--"),
            row(0x1000102e0, 0x1000102e8, "zero-fill", None, Some("__LINKEDIT"), "r--"),
        ]
    );
    let notes = notes(&doc);
    assert!(
        notes.iter().any(|n| n.contains("0x1000102e0..0x1000102e8 in __LINKEDIT (0x2e0 bytes in the file, 0x4000 in memory)")),
        "{notes:?}"
    );
    assert!(notes.iter().all(|n| !n.contains("no read access")), "{notes:?}");

    let tail = read("readprov_macho_arm64", "0x100013ff8", "16");
    assert_eq!(ranges(&tail), [row(0x100013ff8, 0x100014000, "zero-fill", None, Some("__LINKEDIT"), "r--")]);
}

/// A zerofill `__bss` past `__DATA`'s file content, and the run from
/// `__PAGEZERO` straight into `__TEXT`: each byte has the source of the segment
/// it is in.
#[test]
fn macho_ranges_follow_the_segments_they_cross() {
    let bss = read("readprov_macho_arm64", "0x100007ff8", "16");
    assert_eq!(
        ranges(&bss),
        [
            row(0x100007ff8, 0x100008000, "file", Some(0x7ff8), Some("__DATA"), "rw-"),
            row(0x100008000, 0x100008008, "zero-fill", None, Some("__DATA"), "rw-"),
        ]
    );
    let across = read("readprov_macho_arm64", "0xfffffff8", "16");
    assert_eq!(
        ranges(&across),
        [
            row(0xfffffff8, 0x100000000, "zero-fill", None, Some("__PAGEZERO"), "---"),
            row(0x100000000, 0x100000008, "file", Some(0), Some("__TEXT"), "r-x"),
        ]
    );
    let code = read("readprov_macho_arm64", "0x1000003d8", "16");
    assert_eq!(ranges(&code), [row(0x1000003d8, 0x1000003e8, "file", Some(0x3d8), Some("__TEXT"), "r-x")]);
    assert!(notes(&code).is_empty(), "{:?}", notes(&code));
}

/// The same fields on an ELF: `p_memsz` past `p_filesz` is zero fill, and a
/// `PT_LOAD` with no flags is unreadable however file-backed its bytes are.
#[test]
fn elf_bss_is_zero_fill_and_a_flagless_load_is_unreadable() {
    let bss = read("readprov_elf_x86_64", "0x402000", "16");
    assert_eq!(
        ranges(&bss),
        [
            row(0x402000, 0x402008, "file", Some(0x2000), None, "rw-"),
            row(0x402008, 0x402010, "zero-fill", None, None, "rw-"),
        ]
    );
    assert_eq!(bss["provenance"][1]["segment"]["size"], 0x48);
    assert_eq!(bss["provenance"][1]["segment"]["file_size"], 8);
    assert!(notes(&bss).iter().any(|n| n.contains("0x402008..0x402010 in the PT_LOAD at 0x402000")), "{:?}", notes(&bss));

    let sealed = read("readprov_elf_x86_64", "0x401000", "8");
    assert_eq!(sealed["hex"], "efcdab8967452301");
    assert_eq!(ranges(&sealed), [row(0x401000, 0x401008, "file", Some(0x1000), None, "---")]);
    let notes = notes(&sealed);
    assert!(notes.iter().any(|n| n.starts_with("no read access: 0x401000..0x401008 in the PT_LOAD at 0x401000")), "{notes:?}");
    assert!(notes.iter().all(|n| !n.contains("zero fill")), "{notes:?}");

    let text = read("readprov_elf_x86_64", "0x4000e8", "7");
    assert_eq!(ranges(&text), [row(0x4000e8, 0x4000ef, "file", Some(0xe8), None, "r-x")]);
}

/// And on a PE: `VirtualSize` past the raw data is zero fill, a section with
/// no `MEM_READ` is unreadable, and the header page reads from offset 0.
#[test]
fn pe_virtual_size_is_zero_fill_and_a_section_without_mem_read_is_unreadable() {
    let data = read("readprov_pe_x86_64.exe", "0x1400031f8", "16");
    assert_eq!(
        ranges(&data),
        [
            row(0x1400031f8, 0x140003200, "file", Some(0x9f8), Some(".data"), "rw-"),
            row(0x140003200, 0x140003208, "zero-fill", None, Some(".data"), "rw-"),
        ]
    );
    let sealed = read("readprov_pe_x86_64.exe", "0x140006000", "8");
    assert_eq!(ranges(&sealed), [row(0x140006000, 0x140006008, "file", Some(0xa00), Some(".sealed"), "---")]);
    assert!(notes(&sealed).iter().any(|n| n.starts_with("no read access: 0x140006000..0x140006008 in .sealed")));
    let header = read("readprov_pe_x86_64.exe", "0x140000000", "2");
    assert_eq!(header["hex"], "4d5a");
    assert_eq!(ranges(&header), [row(0x140000000, 0x140000002, "file", Some(0), None, "r--")]);
}

/// A header field the loader rewrote so the image parses is cut out of the
/// file bytes around it: an ELF section table that points past the end of the
/// file, and a PE whose DOS magic is not `MZ`.
#[test]
fn a_repaired_header_field_is_not_reported_as_file_bytes() {
    let mut elf = std::fs::read(common::fixture("readprov_elf_x86_64")).unwrap();
    elf[0x28..0x30].copy_from_slice(&0x4141_4141_4141_4141u64.to_le_bytes());
    let path = common::scratch_file("readprov-shdr", "elf");
    std::fs::write(&path, &elf).unwrap();
    let (doc, _) = read_path(path.to_str().unwrap(), "0x400020", "32", &[]);
    let at = |off: u64| 0x400000 + off;
    let piece = |lo: u64, hi: u64, source: &str| row(at(lo), at(hi), source, Some(lo), None, "r-x");
    assert_eq!(
        ranges(&doc),
        [
            piece(0x20, 0x28, "file"),
            piece(0x28, 0x30, "repaired"),
            piece(0x30, 0x3c, "file"),
            piece(0x3c, 0x3d, "repaired"),
            piece(0x3d, 0x3e, "file"),
            piece(0x3e, 0x3f, "repaired"),
            piece(0x3f, 0x40, "file"),
        ]
    );
    let notes = notes(&doc);
    assert!(
        notes.iter().any(|n| n.starts_with("repaired, not file bytes: 0x400028..0x400030 in the PT_LOAD at 0x400000 (file offset 0x28)")),
        "{notes:?}"
    );

    let mut pe = std::fs::read(common::fixture("readprov_pe_x86_64.exe")).unwrap();
    pe[..2].copy_from_slice(b"XX");
    let path = common::scratch_file("readprov-dosmagic", "exe");
    std::fs::write(&path, &pe).unwrap();
    let (doc, _) = read_path(path.to_str().unwrap(), "0x140000000", "4", &[]);
    assert_eq!(doc["hex"], "4d5a7800");
    assert_eq!(
        ranges(&doc),
        [
            row(0x140000000, 0x140000002, "repaired", Some(0), None, "r--"),
            row(0x140000002, 0x140000004, "file", Some(2), None, "r--"),
        ]
    );
}

/// A fat Mach-O's offsets are the file's, not the slice's.
#[test]
fn a_fat_slices_offsets_count_from_the_start_of_the_file() {
    let x86 = read("macho_fat", "0x100000000", "4");
    assert_eq!(x86["hex"], "cffaedfe");
    assert_eq!(x86["provenance"][0]["file_offset"], 0x4000);
    assert_eq!(x86["provenance"][0]["segment"]["file_offset"], 0x4000);
    let (arm, _) = read_in("macho_fat", "0x100000000", "4", &["--slice", "arm64"]);
    assert_eq!(arm["provenance"][0]["file_offset"], 0xc000);
}

/// A relocatable object's bytes are kuna's own layout of its sections, which
/// no segment of the file describes: one `unknown` range, and no note.
#[test]
fn a_relocatable_objects_layout_is_unknown() {
    let doc = read("entry_selectors_x86_64.o", "0x400000", "8");
    assert_eq!(doc["provenance"].as_array().unwrap().len(), 1);
    assert_eq!(doc["provenance"][0]["source"], "unknown");
    assert_eq!(doc["provenance"][0]["segment"], serde_json::Value::Null);
    assert!(notes(&doc).iter().all(|n| !n.contains("zero fill") && !n.contains("no read access")));
}

/// The text view is unchanged: the notes go to stderr, the hexdump to stdout.
#[test]
fn the_text_view_keeps_its_rows_and_says_it_on_stderr() {
    let path = common::fixture("readprov_macho_arm64");
    let (stdout, stderr, rc) = common::run_kuna(&["read", &path, "0x0", "--addr", "--bytes", "16"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains("0x0           00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  |................|"), "{stdout}");
    assert!(!stdout.contains("zero fill"), "{stdout}");
    assert!(stderr.contains("note: zero fill, not file bytes: 0x0..0x10 in __PAGEZERO"), "{stderr}");
}
