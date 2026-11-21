//! [`scan`] over the ARM and stripped Thumb `ehcatch.cc` builds.

use super::*;

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn prel31_sign_extends_bit_30() {
    assert_eq!(prel31(0x0000_0010, 0x1000), 0x1010);
    assert_eq!(prel31(0x7fff_fff0, 0x1000), 0xff0);
    assert_eq!(prel31(0xffff_fff0, 0x1000), 0xff0);
}

/// Three functions and the section-end `CANTUNWIND` entry, which is not a
/// function but bounds the last one. Nothing in the image is Thumb.
#[test]
fn arm_table_gives_a32_starts_and_the_two_lsdas() {
    let bytes = fixture("ehcatch_arm.so");
    let file = object::File::parse(&*bytes).expect("parse");
    let entries = scan(&file);
    let starts: Vec<_> = entries.iter().map(|e| (e.start, e.end, e.thumb)).collect();
    assert_eq!(
        starts,
        [
            (0x103b0, Some(0x103d8), Some(false)),
            (0x103d8, Some(0x1041c), Some(false)),
            (0x1041c, Some(0x10430), Some(false)),
        ]
    );
    assert!(entries[0].lsda.is_some() && entries[1].lsda.is_some());
    assert_eq!(entries[2].lsda, None);
    assert!(thumb_paints(&file).is_empty());
}

/// The table's relocations were made against the section symbol, so no start
/// carries the Thumb bit. `0x103f6` is not a multiple of four; the other two
/// follow the exported Thumb symbols, the image naming no A32 function.
#[test]
fn stripped_thumb_table_is_painted_thumb_per_function() {
    let bytes = fixture("ehcatch_thumb_stripped.so");
    let file = object::File::parse(&*bytes).expect("parse");
    let paints: Vec<_> = thumb_paints(&file).iter().map(|p| (p.addr, p.end, p.value)).collect();
    assert_eq!(
        paints,
        [(0x103b0, Some(0x103c8), 1), (0x103c8, Some(0x103f6), 1), (0x103f6, Some(0x10402), 1)]
    );
    assert_eq!(starts(&file).collect::<Vec<_>>(), [0x103b0, 0x103c8, 0x103f6]);
}

#[test]
fn a_non_arm_image_has_no_entries() {
    let bytes = fixture("ehcatch_x86_64.so");
    assert!(scan(&object::File::parse(&*bytes).expect("parse")).is_empty());
}
