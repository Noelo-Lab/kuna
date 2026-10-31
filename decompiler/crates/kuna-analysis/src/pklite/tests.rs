use super::synth::{self, Layout, Program};
use super::*;

fn packed(name: &str) -> (Program, Layout, Vec<u8>) {
    let (_, layout, relocs) = synth::layouts().into_iter().find(|(n, ..)| *n == name).unwrap();
    let mut p = synth::program(0x1288, 12_000, relocs, &[]);
    p.overlay = vec![0x02, 0x70, 0xe2, 0xaa, 0x28, 0x10, 0x27];
    let bytes = synth::pack(&p, &layout);
    (p, layout, bytes)
}

/// Every stub family, compression mode and relocation-table format the walk
/// knows, packed by the synthetic encoder: the unpack is the original file to
/// the byte, header copy and all, or the header deark's rules rebuild.
#[test]
fn every_layout_round_trips_to_the_original_program() {
    for (seed, (name, layout, relocs)) in synth::layouts().into_iter().enumerate() {
        let stored: &[usize] = if layout.mode.large { &[5000, 9100] } else { &[] };
        let mut p = synth::program(seed as u32 * 7919 + 1, 12_000, relocs, stored);
        p.overlay = b"overlay".to_vec();
        let image = synth::pack(&p, &layout);
        let out = unpack(&image).unwrap_or_else(|e| panic!("{name}: {e}"));
        let want = synth::expected(&p, &layout);
        assert_eq!(out.bytes.len(), want.len(), "{name}");
        assert!(
            out.bytes == want,
            "{name}: first difference at {:#x}",
            out.bytes.iter().zip(&want).position(|(a, b)| a != b).unwrap_or(0)
        );

        assert_eq!(out.container, Container::Exe, "{name}");
        assert_eq!(out.large, layout.mode.large, "{name}");
        assert_eq!(out.v120, layout.mode.v120, "{name}");
        assert_eq!(out.extra != Extra::None, layout.mode.extra != 0, "{name}");
        assert_eq!(out.scrambled, layout.scramble != synth::Scramble::None, "{name}");
        assert_eq!(out.relocations, p.relocs.len(), "{name}");
        assert_eq!(out.load_module, p.code.len(), "{name}");
        assert_eq!(out.overlay, p.overlay.len(), "{name}");
        assert_eq!((out.cs, out.ip, out.ss, out.sp), (p.cs, p.ip, p.ss, p.sp), "{name}");
        match out.header {
            HeaderSource::Original(at) => {
                assert_eq!(layout.mode.extra, 0, "{name}");
                assert_eq!(at, 0x50, "{name}");
                assert_eq!((out.min_alloc, out.max_alloc), (p.min_alloc, p.max_alloc), "{name}");
            }
            HeaderSource::Reconstructed => assert_ne!(layout.mode.extra, 0, "{name}"),
            HeaderSource::None => panic!("{name}: an EXE has a header"),
        }
        let expect_warnings = usize::from(layout.psp_sig) + usize::from(!stored.is_empty());
        assert_eq!(out.warnings.len(), expect_warnings, "{name}: {:?}", out.warnings);
    }
}

#[test]
fn the_dazzle_shaped_layout_reports_what_it_found() {
    let (p, _, image) = packed("v1.00 large");
    assert!(detect(&image));
    let out = unpack(&image).unwrap();
    assert_eq!(out.reported_version, "1.00/l");
    assert_eq!(out.compressed_offset, 0x70 + 0x200);
    assert_eq!(out.relocations, 58);
    assert_eq!(out.header, HeaderSource::Original(0x50));
    // MZ + the 32-byte copy, then the table at the copy's own 0x22.
    assert_eq!(u16::from_le_bytes([out.bytes[0x18], out.bytes[0x19]]), 0x22);
    assert_eq!(&out.bytes[0x22..0x26], &[0x7d, 0x01, 0x58, 0x1b]);
    assert_eq!(out.bytes[out.bytes.len() - 7..], p.overlay[..]);
}

#[test]
fn a_com_file_round_trips() {
    let code = synth::program(5, 9_000, Vec::new(), &[]).code;
    let image = synth::pack_com(&code, 0x010e);
    assert!(detect(&image));
    let out = unpack(&image).unwrap();
    assert_eq!(out.container, Container::Com);
    assert_eq!(out.bytes, code);
    assert_eq!(out.reported_version, "1.14/s");
    assert_eq!(out.detected_version.as_deref(), Some("1.00-1.03"));
    assert_eq!(out.header, HeaderSource::None);
}

#[test]
fn files_without_a_stub_are_not_packed() {
    let p = synth::program(9, 4_000, vec![(0x10, 0)], &[]);
    let plain = p.image();
    assert!(!detect(&plain));
    assert_eq!(unpack(&plain).err(), Some(PkliteError::NotPacked));
    assert_eq!(unpack(b"\x7fELF\x02\x01\x01").err(), Some(PkliteError::NotPacked));
    assert_eq!(unpack(b"").err(), Some(PkliteError::NotPacked));
    assert_eq!(unpack(b"MZ").err(), Some(PkliteError::NotPacked));
    // A COM-shaped start without the rest of the stub or the copyright text
    // is just a program that happens to begin `mov ax; mov dx; cmp ax, sp`.
    let mut com = vec![0u8; 600];
    com[..10].copy_from_slice(b"\xb8\x00\x10\xba\x00\x00\x3b\xc4\x73\x67");
    assert_eq!(unpack(&com).err(), Some(PkliteError::NotPacked));
}

/// A Windows program whose DOS stub is a PKLITE file is the Windows program,
/// so it goes to the other unpackers and gets no PKLITE hint.
#[test]
fn a_pklite_dos_stub_of_a_new_style_executable_is_not_packed() {
    let (_, _, mut image) = packed("v1.00 large");
    assert!(detect(&image));
    while image.len() % 16 != 0 {
        image.push(0);
    }
    let lfanew = image.len() as u32;
    image[0x3c..0x40].copy_from_slice(&lfanew.to_le_bytes());
    image.extend_from_slice(b"PE\0\0");
    image.resize(image.len() + 0x100, 0);
    assert!(!detect(&image));
    assert_eq!(unpack(&image).err(), Some(PkliteError::NotPacked));
    let at = lfanew as usize;
    image[at..at + 4].copy_from_slice(b"NE\x05\x0a");
    assert_eq!(unpack(&image).err(), Some(PkliteError::NotPacked));
}

#[test]
fn a_stub_variant_nothing_matches_is_refused_by_name() {
    let (_, _, image) = packed("v1.00 large");
    let entry = 0x70;
    let mut bad_copier = image.clone();
    for b in &mut bad_copier[entry + 16..entry + 91] {
        if *b == 0xb9 {
            *b = 0xbb;
        }
    }
    match unpack(&bad_copier) {
        Err(PkliteError::Unsupported(m)) => assert!(m.contains("copier"), "{m}"),
        other => panic!("{other:?}"),
    }
    let mut bad_decompressor = image.clone();
    bad_decompressor[entry + 64] = 0x90;
    match unpack(&bad_decompressor) {
        Err(PkliteError::Unsupported(m)) => assert!(m.contains("decompressor"), "{m}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_match_before_the_start_of_output_is_corrupt() {
    let (_, _, mut image) = packed("v1.00 small");
    // match bit, length "00" (3), distance-high "1" (0), low byte 5.
    image[0x270..0x273].copy_from_slice(&[0x09, 0x00, 0x05]);
    match unpack(&image) {
        Err(PkliteError::Corrupt(m)) => assert!(m.contains("distance 5"), "{m}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_large_mode_escape_in_a_small_stream_is_corrupt() {
    let (_, _, mut image) = packed("v1.00 small");
    // match bit, the escape length code 0x3003 ("011"), then the byte 0xfd.
    image[0x270..0x273].copy_from_slice(&[0b1101, 0x00, 0xfd]);
    match unpack(&image) {
        Err(PkliteError::Corrupt(m)) => assert!(m.contains("0xfd"), "{m}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_truncated_file_is_corrupt() {
    let (_, _, image) = packed("v1.00 large");
    let cut = &image[..image.len() - 200];
    assert!(matches!(unpack(cut), Err(PkliteError::Corrupt(_))));
}

#[test]
fn a_relocation_table_that_overruns_the_footer_is_corrupt() {
    let (_, _, mut image) = packed("v1.00 large");
    // Claim one more relocation in the last group than the bytes hold.
    let (_, _, clean) = packed("v1.00 large");
    let out = unpack(&clean).unwrap();
    image[out.compressed_end] = 0xff;
    match unpack(&image) {
        Err(PkliteError::Corrupt(m)) => assert!(m.contains("relocation"), "{m}"),
        other => panic!("{other:?}"),
    }
}

/// The kept header copy is a claim, not a fact: one that names other
/// registers than the footer is set aside and the header rebuilt.
#[test]
fn a_header_copy_that_disagrees_with_the_footer_is_not_used() {
    let (p, _, mut image) = packed("v1.00 large");
    image[0x50 + 0x14 - 2] ^= 0x01;
    let out = unpack(&image).unwrap();
    assert_eq!(out.header, HeaderSource::Reconstructed);
    assert_eq!((out.cs, out.ip), (p.cs, p.ip));
    assert!(out.warnings.iter().any(|w| w.contains("footer")), "{:?}", out.warnings);
}

#[test]
fn the_version_word_reads_like_pklite_prints_it() {
    assert_eq!(version_string(0x2100), "1.00/l");
    assert_eq!(version_string(0x310f), "1.15/l/e");
    assert_eq!(version_string(0x010c), "1.12/s");
    assert_eq!(version_string(0x6201), "2.01/h/l");
}

/// Writes every synthetic file and its expected unpack to
/// `$KUNA_PKLITE_DUMP`, for checking the generator against deark
/// (`deark -m pklite`); not part of the suite.
#[test]
#[ignore]
fn dump_synthetic_files_for_an_external_check() {
    let dir =
        std::path::PathBuf::from(std::env::var("KUNA_PKLITE_DUMP").expect("KUNA_PKLITE_DUMP"));
    for (seed, (name, layout, relocs)) in synth::layouts().into_iter().enumerate() {
        let stored: &[usize] = if layout.mode.large { &[5000, 9100] } else { &[] };
        let mut p = synth::program(seed as u32 * 7919 + 1, 12_000, relocs, stored);
        p.overlay = b"overlay".to_vec();
        let stem: String =
            name.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
        std::fs::write(dir.join(format!("{stem}.exe")), synth::pack(&p, &layout)).unwrap();
        std::fs::write(dir.join(format!("{stem}.want")), synth::expected(&p, &layout)).unwrap();
    }
    let code = synth::program(5, 9_000, Vec::new(), &[]).code;
    std::fs::write(dir.join("com.com"), synth::pack_com(&code, 0x010e)).unwrap();
    std::fs::write(dir.join("com.want"), code).unwrap();
}

/// The round trip above means something only if the generator reaches every
/// token kind: literals, both two-byte match forms, short and escaped long
/// lengths, most distance high-bit codes, the no-op and uncompressed escapes.
/// Nothing may match behind an uncompressed area, so these come late.
#[test]
fn the_generator_exercises_every_token_kind() {
    let p = synth::program(77, 30_000, Vec::new(), &[24_000, 28_000]);
    let m = |large, v120| synth::Mode { large, v120, extra: 0, offset_xor: 0 };
    for mode in [m(true, false), m(false, false), m(true, true), m(false, true)] {
        let stored: &[(usize, usize)] = if mode.large { &p.stored } else { &[] };
        let seps = if mode.large { vec![6_000] } else { Vec::new() };
        let (_, c) = synth::compress_with_coverage(&p.code, &mode, stored, &seps);
        assert!(
            c.literals > 1000 && c.short_matches > 100 && c.long_matches > 10,
            "{mode:?} {c:?}"
        );
        assert!(c.two > 10, "{mode:?} {c:?}");
        assert!(c.high_bits_seen.count_ones() >= 28, "{mode:?} {c:?}");
        if mode.v120 {
            assert!(c.two_high > 0 && c.zero_literals > 0, "{mode:?} {c:?}");
        }
        if mode.large {
            assert_eq!((c.separators, c.stored), (1, 2), "{mode:?} {c:?}");
        }
    }
}

/// Untrusted input: random byte flips and truncations across the stub, the
/// stream and the tables must come back as a result, never a panic.
#[test]
fn mutated_files_never_panic() {
    let mut s = 0x2468_ace1u32;
    let mut rnd = move || {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        s
    };
    for name in ["v1.00 large", "v1.14 XOR-scrambled large extra", "May 1990 beta", "startup patch"] {
        let (_, _, image) = packed(name);
        for round in 0..400 {
            let mut m = image.clone();
            for _ in 0..1 + rnd() % 4 {
                let at = rnd() as usize % m.len();
                m[at] = rnd() as u8;
            }
            if round % 5 == 0 {
                m.truncate(rnd() as usize % m.len());
            }
            let _ = unpack(&m);
            let _ = detect(&m);
        }
    }
    let com = synth::pack_com(&synth::program(5, 3_000, Vec::new(), &[]).code, 0x010e);
    for _ in 0..400 {
        let mut m = com.clone();
        let at = rnd() as usize % m.len();
        m[at] = rnd() as u8;
        let _ = unpack(&m);
    }
}
