use super::*;

/// One segment or section of a synthesized image: its type, and its contents.
struct Part {
    kind: u32,
    data: Vec<u8>,
}

/// Lay out an ELF header, `segments` as program headers, `sections` as section
/// headers, and every part's contents after them.
fn image(wide: bool, big: bool, osabi: u8, segments: &[Part], sections: &[Part]) -> Vec<u8> {
    let (ehsize, phentsize, shentsize) = if wide { (64, 56, 64) } else { (52, 32, 40) };
    let phoff = ehsize;
    let shoff = phoff + phentsize * segments.len();
    let mut data_at = shoff + shentsize * sections.len();
    let mut out = vec![0u8; data_at];
    let put = |out: &mut Vec<u8>, at: usize, v: u64, n: usize| {
        let bytes = if big {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        };
        let b = if big { &bytes[8 - n..] } else { &bytes[..n] };
        out[at..at + n].copy_from_slice(b);
    };
    let word = if wide { 8 } else { 4 };
    out[0..4].copy_from_slice(b"\x7fELF");
    out[4] = if wide { 2 } else { 1 };
    out[5] = if big { 2 } else { 1 };
    out[6] = 1;
    out[7] = osabi;
    put(&mut out, 24 + word, phoff as u64, word);
    put(&mut out, 24 + 2 * word, shoff as u64, word);
    let rest = 24 + 3 * word + 4;
    put(&mut out, rest, ehsize as u64, 2);
    put(&mut out, rest + 2, phentsize as u64, 2);
    put(&mut out, rest + 4, segments.len() as u64, 2);
    put(&mut out, rest + 6, shentsize as u64, 2);
    put(&mut out, rest + 8, sections.len() as u64, 2);
    let mut payloads = Vec::new();
    for (i, p) in segments.iter().enumerate() {
        let ph = phoff + i * phentsize;
        put(&mut out, ph, p.kind as u64, 4);
        if wide {
            put(&mut out, ph + 8, data_at as u64, 8);
            put(&mut out, ph + 32, p.data.len() as u64, 8);
            put(&mut out, ph + 48, 4, 8);
        } else {
            put(&mut out, ph + 4, data_at as u64, 4);
            put(&mut out, ph + 16, p.data.len() as u64, 4);
            put(&mut out, ph + 28, 4, 4);
        }
        payloads.extend_from_slice(&p.data);
        data_at += p.data.len();
    }
    for (i, p) in sections.iter().enumerate() {
        let sh = shoff + i * shentsize;
        put(&mut out, sh + 4, p.kind as u64, 4);
        if wide {
            put(&mut out, sh + 24, data_at as u64, 8);
            put(&mut out, sh + 32, p.data.len() as u64, 8);
            put(&mut out, sh + 48, 4, 8);
        } else {
            put(&mut out, sh + 16, data_at as u64, 4);
            put(&mut out, sh + 20, p.data.len() as u64, 4);
            put(&mut out, sh + 32, 4, 4);
        }
        payloads.extend_from_slice(&p.data);
        data_at += p.data.len();
    }
    out.extend_from_slice(&payloads);
    out
}

/// A note area holding one note per `(owner, type)`, padded to 4 bytes.
fn notes(big: bool, entries: &[(&str, u32)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (owner, kind) in entries {
        let mut name = owner.as_bytes().to_vec();
        name.push(0);
        let desc = [0u8; 16];
        for v in [name.len() as u32, desc.len() as u32, *kind] {
            out.extend_from_slice(&if big {
                v.to_be_bytes()
            } else {
                v.to_le_bytes()
            });
        }
        out.extend_from_slice(&name);
        out.resize(out.len().next_multiple_of(4), 0);
        out.extend_from_slice(&desc);
    }
    out
}

/// A dynamic table of `(tag, value)` entries closed by `DT_NULL`.
fn dynamic(wide: bool, entries: &[u64]) -> Vec<u8> {
    let mut out = Vec::new();
    for &tag in entries.iter().chain(&[0]) {
        for v in [tag, 0] {
            if wide {
                out.extend_from_slice(&v.to_le_bytes());
            } else {
                out.extend_from_slice(&(v as u32).to_le_bytes());
            }
        }
    }
    out
}

/// Firmware carries no interpreter, no named shared object, no ABI note and no
/// system `EI_OSABI`, and is not a user-space image.
#[test]
fn bare_image_is_not_userland() {
    assert!(!is_os_userland_image(&image(false, false, 0, &[], &[])));
    let build_id = Part {
        kind: PT_NOTE,
        data: notes(false, &[("GNU", 3)]),
    };
    assert!(!is_os_userland_image(&image(
        false,
        false,
        0,
        &[build_id],
        &[]
    )));
    let relocations_only = Part {
        kind: PT_DYNAMIC,
        data: dynamic(false, &[7, 8, 9]),
    };
    assert!(!is_os_userland_image(&image(
        false,
        false,
        0,
        &[relocations_only],
        &[]
    )));
    for osabi in [0, 64, 97, 255] {
        assert!(
            !is_os_userland_image(&image(false, false, osabi, &[], &[])),
            "EI_OSABI {osabi}"
        );
    }
}

/// Each marker on its own makes the image a user-space one.
#[test]
fn any_marker_makes_userland() {
    for osabi in OS_ABIS {
        assert!(
            is_os_userland_image(&image(false, false, osabi, &[], &[])),
            "EI_OSABI {osabi}"
        );
    }
    let interp = Part {
        kind: PT_INTERP,
        data: b"/lib/ld-linux.so.3\0".to_vec(),
    };
    assert!(is_os_userland_image(&image(
        false,
        false,
        0,
        &[interp],
        &[]
    )));
    for tag in [DT_NEEDED, DT_SONAME] {
        let dynamic = Part {
            kind: PT_DYNAMIC,
            data: dynamic(true, &[7, tag]),
        };
        assert!(
            is_os_userland_image(&image(true, false, 0, &[dynamic], &[])),
            "tag {tag}"
        );
    }
    for owner in OS_NOTE_OWNERS {
        let owner = std::str::from_utf8(owner).unwrap();
        let note = Part {
            kind: PT_NOTE,
            data: notes(false, &[("GNU", 3), (owner, 1)]),
        };
        assert!(
            is_os_userland_image(&image(false, false, 0, &[note], &[])),
            "{owner}"
        );
    }
}

/// A relocatable object has no program headers: its ABI note is a section.
#[test]
fn note_section_counts_without_segments() {
    let note = Part {
        kind: SHT_NOTE,
        data: notes(true, &[("GNU", 1)]),
    };
    assert!(is_os_userland_image(&image(true, true, 0, &[], &[note])));
    let other = Part {
        kind: 1,
        data: notes(true, &[("GNU", 1)]),
    };
    assert!(!is_os_userland_image(&image(true, true, 0, &[], &[other])));
}

/// A truncated or foreign file is not an image at all.
#[test]
fn malformed_input_is_not_userland() {
    let interp = Part {
        kind: PT_INTERP,
        data: b"/lib/ld.so\0".to_vec(),
    };
    let whole = image(false, false, 0, &[interp], &[]);
    assert!(is_os_userland_image(&whole));
    for cut in [0, 4, 30, 52, 60] {
        assert!(!is_os_userland_image(&whole[..cut]), "cut at {cut}");
    }
    assert!(!is_os_userland_image(b"MZ\x90\0"));
    let mut huge = whole.clone();
    huge[28..32].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(!is_os_userland_image(&huge));
}
