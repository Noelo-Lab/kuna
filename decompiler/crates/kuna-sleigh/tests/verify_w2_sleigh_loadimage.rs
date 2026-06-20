//! Verifier adversarial tests for item `w2-sleigh-loadimage` (round 1).
//!
//! Every expectation below was pinned against a C++ oracle: two standalone
//! harnesses transcribing verbatim (a) the hex-content istream scan of
//! `LoadImageXml::open` (loadimage_xml.cc:111-135) and (b) the full `pad()`
//! body (loadimage_xml.cc:154-201) including its erase-while-iterating and
//! insert-while-iterating idioms over `std::map` (reproduced in
//! docs/rust-port/reviews/w2-sleigh-loadimage.md).  Targets: the signed-char
//! `istream::get` semantics on garbage/whitespace/high-bit content, the
//! wrapping `offset + size - 1` end arithmetic that lets an empty chunk at
//! offset 0 erase every later same-space chunk, the exact-end erase tie, the
//! cross-space non-clamping of the 512-byte pad, the wrapOffset overlap
//! behavior of a chunk extending past the end of its space, zero-length
//! loadFill requests, and the int4 truncation chain in `adjustVma`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaError;
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_base::xml::xml_tree;
use kuna_sleigh::loadimage::{LoadImage, LoadImageFunc, RawLoadImage};
use kuna_sleigh::loadimage_xml::{register_loadimage_xml_ids, LoadImageXml};

/// const(0) + processor spaces named `names`, little endian, `addr_size`-byte
/// addresses, wordsize 1.
fn manager(names: &[&str], addr_size: u32) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    for (i, nm) in names.iter().enumerate() {
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            nm,
            false,
            addr_size,
            1,
            1 + i as i32,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    }
    m.set_default_code_space(1).unwrap();
    m
}

fn registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_loadimage_xml_ids(&mut registry);
    registry
}

fn image_from_str(xml: &str, m: &AddrSpaceManager, registry: &IdRegistry) -> LoadImageXml {
    let doc = xml_tree(xml.as_bytes()).unwrap();
    let mut img = LoadImageXml::new("verify", Rc::clone(doc.get_root())).unwrap();
    img.open(m, registry).unwrap();
    img
}

/// A single-chunk image whose <bytechunk> text is exactly `content`.
fn single_chunk(content: &str, offset: u64, m: &AddrSpaceManager, reg: &IdRegistry) -> LoadImageXml {
    image_from_str(
        &format!(
            "<binaryimage arch=\"test\"><bytechunk space=\"ram\" offset=\"{offset:#x}\">{content}</bytechunk></binaryimage>"
        ),
        m,
        reg,
    )
}

/// Assert the chunk at `offset` parsed to exactly `expect`, by pinning the
/// bytes, the final byte of the 512-byte pad, and the first unmapped byte.
fn assert_chunk_bytes(img: &mut LoadImageXml, ram: &Rc<AddrSpace>, offset: u64, expect: &[u8]) {
    if !expect.is_empty() {
        let got = img.load(expect.len() as i32, &Address::new(Rc::clone(ram), offset)).unwrap();
        assert_eq!(got, expect, "chunk bytes at {offset:#x}");
    }
    let pad_end = offset + expect.len() as u64 + 512;
    let got = img.load(1, &Address::new(Rc::clone(ram), pad_end - 1)).unwrap();
    assert_eq!(got, vec![0], "last pad byte at {:#x}", pad_end - 1);
    let err = img.load(1, &Address::new(Rc::clone(ram), pad_end)).unwrap_err();
    assert!(
        matches!(err, KunaError::DataUnavail { .. }),
        "first unmapped byte at {pad_end:#x}"
    );
}

/// The hex-content scan keeps the C++ istream/signed-char semantics: the
/// stream chars assign into a (signed) char, whitespace is skipped only
/// *between* pairs, conversion of non-hex chars produces the same garbage,
/// and any non-positive char (EOF, NUL, high-bit byte) ends the scan.
/// C++ oracle (verbatim scan loop over istringstream):
///   "0 a"        -> f0          (' ' as second digit: 32-'0' = -16)
///   "G@!~"       -> 09 37       (garbage conversions, int math, uint1 trunc)
///   "ab\ncd e f" -> ab cd d0    (ws between pairs only; "e " pair = 0xd0)
///   "0:"         -> 03          (':' takes the upper-case-hex branch)
///   "abc"        -> ab          (dangling nibble dropped at EOF)
///   "ab\xc3\xa9cd" -> ab        (high-bit byte reads negative, ends scan)
///   "\n  \t\r"   -> (empty)
#[test]
fn verify_w2loadimage_hex_scan_istream_char_semantics() {
    let m = manager(&["ram"], 4);
    let reg = registry();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

    let cases: &[(&str, &[u8])] = &[
        ("0 a", &[0xf0]),
        ("G@!~", &[0x09, 0x37]),
        ("ab\ncd e f", &[0xab, 0xcd, 0xd0]),
        ("0:", &[0x03]),
        ("abc", &[0xab]),
        ("ab\u{e9}cd", &[0xab]), // 'é' = 0xc3 0xa9 in UTF-8
        ("\n  \t\r", &[]),
    ];
    for (content, expect) in cases {
        let mut img = single_chunk(content, 0x1000, &m, &reg);
        assert_chunk_bytes(&mut img, &ram, 0x1000, expect);
    }
}

/// pad()'s redundant-chunk erase computes `end = offset + size - 1` in
/// wrapping uintb arithmetic: an empty chunk at offset 0 has end == uintb
/// max, so EVERY later same-space chunk is "redundant" and erased; pad()
/// then appends 512 zeros into the empty chunk itself.
/// C++ oracle (caseA): chunks {0x0 size 0, 0x100 size 8, 0x9000 size 4}
/// collapse to exactly {0x0 size 0x200}.
#[test]
fn verify_w2loadimage_pad_empty_chunk_at_zero_wrapping_erase() {
    let m = manager(&["ram"], 4);
    let reg = registry();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let mut img = image_from_str(
        "<binaryimage arch=\"test\">\
         <bytechunk space=\"ram\" offset=\"0x0\">\n</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x100\">eeeeeeeeeeeeeeee</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x9000\">dddddddd</bytechunk>\
         </binaryimage>",
        &m,
        &reg,
    );

    // The 0xee chunk was erased: its range now reads as the pad's zeros
    let got = img.load(8, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
    assert_eq!(got, vec![0u8; 8]);
    // Pad covers exactly [0x0, 0x200)
    let got = img.load(1, &Address::new(Rc::clone(&ram), 0x1ff)).unwrap();
    assert_eq!(got, vec![0]);
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0x200)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
    // The 0xdd chunk was erased too (nothing mapped at 0x9000)
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0x9000)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
}

/// Erase ties and chains: a chunk whose end EQUALS its predecessor's end is
/// erased (end1 >= end2), consecutive erasures keep comparing against the
/// same survivor, and an empty chunk strictly inside a gap survives (its
/// end is offset-1, *below* the predecessor end) then self-pads 512.
/// C++ oracle (caseC): {0x100/8, 0x102/2, 0x104/4, 0x10c/2} ->
///   {0x100 size 8, 0x108 size 4 (pad), 0x10c size 2, 0x10e size 0x200}.
/// C++ oracle (caseB): {0x100/4, 0x200/0} ->
///   {0x100 size 4, 0x104 size 0xfc (pad), 0x200 size 0x200}.
#[test]
fn verify_w2loadimage_pad_exact_end_tie_chain_and_empty_gap_chunk() {
    let m = manager(&["ram"], 4);
    let reg = registry();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

    // caseC: 0x102 (contained) and 0x104 (exact-end tie) erased; 0x10c kept
    let mut img = image_from_str(
        "<binaryimage arch=\"test\">\
         <bytechunk space=\"ram\" offset=\"0x100\">1111111111111111</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x102\">2222</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x104\">33333333</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x10c\">4444</bytechunk>\
         </binaryimage>",
        &m,
        &reg,
    );
    let got = img.load(0x10, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
    let mut expect = vec![0x11u8; 8]; // erased 0x22/0x33 never show
    expect.extend([0, 0, 0, 0]); // 4-byte pad clamped to next chunk
    expect.extend([0x44, 0x44]);
    expect.extend([0, 0]); // head of the trailing 512 pad
    assert_eq!(got, expect);
    // trailing pad covers exactly [0x10e, 0x30e)
    let got = img.load(1, &Address::new(Rc::clone(&ram), 0x30d)).unwrap();
    assert_eq!(got, vec![0]);
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0x30e)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));

    // caseB: empty chunk in the gap survives and self-pads
    let mut img = image_from_str(
        "<binaryimage arch=\"test\">\
         <bytechunk space=\"ram\" offset=\"0x100\">aabbccdd</bytechunk>\
         <bytechunk space=\"ram\" offset=\"0x200\">\n</bytechunk>\
         </binaryimage>",
        &m,
        &reg,
    );
    let got = img.load(0x300, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
    let mut expect = vec![0xaa, 0xbb, 0xcc, 0xdd];
    expect.extend(vec![0u8; 0x2fc]); // 0xfc clamped pad + 0x200 self-pad
    assert_eq!(got, expect);
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0x400)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
}

/// The pad clamp to the next chunk applies only when the next chunk is in
/// the SAME space (pointer identity in C++): a numerically-near chunk in a
/// different space neither clamps the 512-byte pad nor is erased.
/// C++ oracle (caseD): {spc1 0x100/4, spc2 0x110/4} ->
///   {spc1 0x100/4, spc1 0x104/0x200, spc2 0x110/4, spc2 0x114/0x200}.
#[test]
fn verify_w2loadimage_pad_cross_space_no_clamp() {
    let m = manager(&["ram1", "ram2"], 4);
    let reg = registry();
    let ram1 = Rc::clone(m.get_space_by_name("ram1").unwrap());
    let ram2 = Rc::clone(m.get_space_by_name("ram2").unwrap());
    let mut img = image_from_str(
        "<binaryimage arch=\"test\">\
         <bytechunk space=\"ram1\" offset=\"0x100\">55555555</bytechunk>\
         <bytechunk space=\"ram2\" offset=\"0x110\">66666666</bytechunk>\
         </binaryimage>",
        &m,
        &reg,
    );

    // ram1's pad runs the full 512 bytes straight through offset 0x110:
    // reading ram1:0x110 yields the pad zero, not ram2's 0x66
    let got = img.load(1, &Address::new(Rc::clone(&ram1), 0x110)).unwrap();
    assert_eq!(got, vec![0]);
    let got = img.load(1, &Address::new(Rc::clone(&ram1), 0x303)).unwrap();
    assert_eq!(got, vec![0]);
    let err = img.load(1, &Address::new(Rc::clone(&ram1), 0x304)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));

    // ram2's chunk survived (not erased by the cross-space comparison)
    let got = img.load(4, &Address::new(Rc::clone(&ram2), 0x110)).unwrap();
    assert_eq!(got, vec![0x66; 4]);
    // and below it ram2 is unmapped (the begin()-stays iterator path)
    let err = img.load(1, &Address::new(Rc::clone(&ram2), 0x10f)).unwrap_err();
    match &err {
        KunaError::DataUnavail { explain } => {
            assert_eq!(explain, "Bytes at 0x0000010f are not mapped");
        }
        other => panic!("expected DataUnavail, got {other:?}"),
    }
}

/// A chunk whose data extends past the end of its space: endaddr wraps below
/// the chunk key, so pad() skips it (no pad chunk at all), and loadFill's
/// wrapOffset-based overlap lets a read at offset 0 land INSIDE the chunk
/// (dist = 0 - 0xfff0 wraps to 0x10 < 0x18).
/// C++ oracle (caseF): {spc3(2-byte) 0xfff0 size 0x18} unchanged by pad().
#[test]
fn verify_w2loadimage_space_end_wrap_chunk_no_pad_and_wrapped_overlap() {
    let m = manager(&["ram"], 2);
    let reg = registry();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let mut img = single_chunk(
        "000102030405060708090a0b0c0d0e0f1011121314151617",
        0xfff0,
        &m,
        &reg,
    );

    // The whole 0x18-byte chunk reads back from its key address
    let got = img.load(0x18, &Address::new(Rc::clone(&ram), 0xfff0)).unwrap();
    let expect: Vec<u8> = (0u8..0x18).collect();
    assert_eq!(got, expect);

    // Read at offset 0: overlap distance wrapOffset(0 - 0xfff0) = 0x10
    // falls inside the chunk -> bytes 0x10..0x18 of the chunk data
    let got = img.load(8, &Address::new(Rc::clone(&ram), 0x0)).unwrap();
    assert_eq!(got, (0x10u8..0x18).collect::<Vec<u8>>());

    // Just past the wrapped image: dist 0x18 >= chunk size -> unmapped,
    // proving NO pad chunk was inserted after the wrapping chunk
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0x8)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
    // And just below the chunk: dist wraps to 0xffff -> unmapped
    let err = img.load(1, &Address::new(Rc::clone(&ram), 0xffef)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
}

/// Zero-length loadFill requests never throw in C++ (the while(size>0) loop
/// is skipped and the final size>0 test fails), even at unmapped addresses
/// — for both LoadImageXml and RawLoadImage.  adjustVma truncates
/// addressToByte's uintb result into an int4 (high bits of a 2^32-multiple
/// adjustment vanish) and negative adjustments wrap through wrapOffset.
#[test]
fn verify_w2loadimage_zero_length_reads_and_adjust_vma_int4_truncation() {
    let m = manager(&["ram"], 4);
    let reg = registry();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let mut img = image_from_str(
        "<binaryimage arch=\"test\">\
         <bytechunk space=\"ram\" offset=\"0x100\">a1b2c3d4</bytechunk>\
         <symbol space=\"ram\" offset=\"0x102\" name=\"sym\"/>\
         </binaryimage>",
        &m,
        &reg,
    );

    // Zero-length read at an unmapped address succeeds (empty result)
    let got = img.load(0, &Address::new(Rc::clone(&ram), 0xdead_0000)).unwrap();
    assert_eq!(got, Vec::<u8>::new());

    // adjust = 0x1_0000_0010: int4 off = (int4)addressToByte(...) = 0x10
    img.adjust_vma(0x1_0000_0010);
    let got = img.load(4, &Address::new(Rc::clone(&ram), 0x110)).unwrap();
    assert_eq!(got, vec![0xa1, 0xb2, 0xc3, 0xd4]);
    let err = img.load(4, &Address::new(Rc::clone(&ram), 0x100)).unwrap_err();
    assert!(matches!(err, KunaError::DataUnavail { .. }));
    let mut rec = LoadImageFunc::default();
    img.open_symbols();
    assert!(img.get_next_symbol(&mut rec));
    assert_eq!(rec.address.get_offset(), 0x112);

    // Negative adjustment: 0x110 - 0x20 = 0xf0
    img.adjust_vma(-0x20);
    let got = img.load(4, &Address::new(Rc::clone(&ram), 0xf0)).unwrap();
    assert_eq!(got, vec![0xa1, 0xb2, 0xc3, 0xd4]);

    // Wrap below zero: 0xf0 - 0x200 wraps (mod 2^32) to 0xffff_fef0
    img.adjust_vma(-0x200);
    let got = img.load(4, &Address::new(Rc::clone(&ram), 0xffff_fef0)).unwrap();
    assert_eq!(got, vec![0xa1, 0xb2, 0xc3, 0xd4]);

    // RawLoadImage: zero-length read far past EOF also succeeds
    let mut path = std::env::temp_dir();
    path.push(format!("kuna_verify_loadimage_{}", std::process::id()));
    std::fs::write(&path, [1u8, 2, 3, 4]).unwrap();
    let mut raw = RawLoadImage::new(path.to_str().unwrap());
    raw.attach_to_space(Rc::clone(&ram));
    raw.open().unwrap();
    let got = raw.load(0, &Address::new(Rc::clone(&ram), 0x9999_9999)).unwrap();
    assert_eq!(got, Vec::<u8>::new());
    std::fs::remove_file(&path).unwrap();
}
