// Tests for the `gotypes` harvest (included into `mod tests` in gotypes/mod.rs).
//
// Covered, hermetically (synthetic byte spans, no Go toolchain):
//   - the Go name encoding (flags byte + varint length), including the
//     extra-star strip and malformation refusals;
//   - `_type` record decoding: kind masking, size bound, name resolution;
//   - the moduledata window scan: `types`/`etypes` + `typelinks`
//     cross-validation finds the real triple and refuses a corrupt one;
//   - `name_ok`'s phase-1 vocabulary.
// The end-to-end harvest over a real stripped Go build (factory interning,
// facets, definition rendering) is `kuna-console/tests/verify_gotypes.rs`.

use super::*;

/// Build the synthetic type blob + typelinks + moduledata window used by the
/// scan tests. Returns `(spans, anchor, types, etypes)`.
fn synth() -> (Vec<(u64, Vec<u8>)>, u64, u64, u64) {
    const T: u64 = 0x100000; // types blob
    const L: u64 = 0x200000; // typelinks array
    const M: u64 = 0x300000; // moduledata
    const H: u64 = 0x50000; // pclntab header (content irrelevant here)

    let mut blob = vec![0u8; 0x400];
    // int record @ +0x00: size 8, kind INT, name @ +0x200.
    blob[0..8].copy_from_slice(&8u64.to_le_bytes());
    blob[23] = KIND_INT;
    blob[40..44].copy_from_slice(&0x200i32.to_le_bytes());
    // main.Point record @ +0x60: size 16, named struct, name @ +0x210,
    // pkgPath 0 @ +48, fields slice {T+0x300, 2, 2} @ +56.
    let s = 0x60;
    blob[s..s + 8].copy_from_slice(&16u64.to_le_bytes());
    blob[s + 20] = TFLAG_NAMED;
    blob[s + 23] = KIND_STRUCT;
    blob[s + 40..s + 44].copy_from_slice(&0x210i32.to_le_bytes());
    blob[s + 56..s + 64].copy_from_slice(&(T + 0x300).to_le_bytes());
    blob[s + 64..s + 72].copy_from_slice(&2u64.to_le_bytes());
    blob[s + 72..s + 80].copy_from_slice(&2u64.to_le_bytes());
    // names: [flags, varint len, bytes]
    blob[0x200..0x205].copy_from_slice(&[0, 3, b'i', b'n', b't']);
    blob[0x210] = 0;
    blob[0x211] = 10;
    blob[0x212..0x21c].copy_from_slice(b"main.Point");
    blob[0x230..0x233].copy_from_slice(&[0, 1, b'X']);
    blob[0x238..0x23b].copy_from_slice(&[0, 1, b'Y']);
    // fields array @ +0x300: {name_ptr, typ_ptr, offset} x2
    for (i, (np, off)) in [(T + 0x230, 0u64), (T + 0x238, 8u64)].iter().enumerate() {
        let f = 0x300 + i * 24;
        blob[f..f + 8].copy_from_slice(&np.to_le_bytes());
        blob[f + 8..f + 16].copy_from_slice(&T.to_le_bytes()); // typ = int
        blob[f + 16..f + 24].copy_from_slice(&off.to_le_bytes());
    }

    // typelinks: two valid offsets.
    let mut links = Vec::new();
    links.extend_from_slice(&0x60i32.to_le_bytes());
    links.extend_from_slice(&0i32.to_le_bytes());

    // moduledata: pcHeader*, funcnametab slice, pad, types, etypes, pad,
    // typelinks slice.
    let mut md = Vec::new();
    md.extend_from_slice(&H.to_le_bytes()); // w0 pcHeader*
    md.extend_from_slice(&T.to_le_bytes()); // w1 funcnametab.ptr
    md.extend_from_slice(&4u64.to_le_bytes()); // w2 len
    md.extend_from_slice(&4u64.to_le_bytes()); // w3 cap
    for _ in 0..6 {
        md.extend_from_slice(&0u64.to_le_bytes());
    }
    md.extend_from_slice(&T.to_le_bytes()); // w10 types
    md.extend_from_slice(&(T + 0x400).to_le_bytes()); // w11 etypes
    for _ in 0..2 {
        md.extend_from_slice(&0u64.to_le_bytes());
    }
    md.extend_from_slice(&L.to_le_bytes()); // w14 typelinks.ptr
    md.extend_from_slice(&2u64.to_le_bytes()); // w15 len
    md.extend_from_slice(&2u64.to_le_bytes()); // w16 cap
    for _ in 0..8 {
        md.extend_from_slice(&0u64.to_le_bytes());
    }

    let spans = vec![(T, blob), (L, links), (M, md), (H, vec![0u8; 16])];
    (spans, M, T, T + 0x400)
}

fn img_of(spans: &[(u64, Vec<u8>)]) -> ImageBytes<'_> {
    ImageBytes::from_spans(spans.iter().map(|(a, b)| (*a, b.as_slice())).collect())
}

#[test]
fn go_name_encoding_decodes_and_refuses() {
    let data = [0u8, 3, b'a', b'b', b'c', 0, 0];
    let spans = vec![(0x1000u64, data.to_vec())];
    let img = img_of(&spans);
    assert_eq!(read_go_name(&img, 0x1000).as_deref(), Some("abc"));
    // zero length refused; unmapped refused.
    let zero = vec![0u8, 0];
    let spans = vec![(0x1000u64, zero)];
    let img = img_of(&spans);
    assert_eq!(read_go_name(&img, 0x1000), None);
    assert_eq!(read_go_name(&img, 0x9999), None);
}

#[test]
fn type_records_decode_with_kind_and_name() {
    let (spans, _m, types, etypes) = synth();
    let img = img_of(&spans);
    let int_t = read_type(&img, types, etypes, 0).expect("int record");
    assert_eq!((int_t.kind, int_t.size, int_t.name.as_str()), (KIND_INT, 8, "int"));
    let pt = read_type(&img, types, etypes, 0x60).expect("struct record");
    assert_eq!((pt.kind, pt.size, pt.name.as_str()), (KIND_STRUCT, 16, "main.Point"));
    assert!(pt.tflag & TFLAG_NAMED != 0);
    // Out-of-blob and bad-kind offsets refuse.
    assert!(read_type(&img, types, etypes, 0x3f0).is_none(), "truncated record");
    assert!(read_type(&img, types, etypes, 0x20).is_none(), "kind 0 at padding");
}

#[test]
fn moduledata_window_scan_cross_validates() {
    let (spans, anchor, types, etypes) = synth();
    let img = img_of(&spans);
    let meta = locate_type_meta(&img, anchor).expect("finds the triple");
    assert_eq!((meta.types, meta.etypes), (types, etypes));
    // Negative offsets are filtered at collection; both links here are valid.
    assert_eq!(meta.offsets, vec![0x60, 0]);
}

#[test]
fn moduledata_scan_refuses_a_corrupt_typelinks() {
    let (mut spans, anchor, _t, _e) = synth();
    // Corrupt every typelink entry: offsets far past etypes.
    spans[1].1[0..4].copy_from_slice(&0x7fff_0000i32.to_le_bytes());
    spans[1].1[4..8].copy_from_slice(&0x7fff_0004i32.to_le_bytes());
    let img = img_of(&spans);
    assert!(locate_type_meta(&img, anchor).is_none(), "no validating triple");
}

#[test]
fn phase1_name_vocabulary() {
    assert!(name_ok("main.Point"));
    assert!(name_ok("internal/abi.Type"));
    assert!(!name_ok("int"), "undotted");
    assert!(!name_ok("main.Box[int]"), "generic instantiation");
    assert!(!name_ok("struct { x int }"), "anonymous");
    assert!(!name_ok(""), "empty");
}

/// Build the go1.26+ shape: NO typelinks slice — the records sit consecutively
/// in `[types + 8, types + typedesclen)` and moduledata carries the
/// `types, typedesclen, etypes` triple. Returns `(spans, anchor, types)`.
fn synth_descriptors() -> (Vec<(u64, Vec<u8>)>, u64, u64) {
    const T: u64 = 0x100000;
    const M: u64 = 0x300000;
    const H: u64 = 0x50000;

    let mut blob = vec![0u8; 0x400];
    // The walk starts at types + 8.
    // int record @ +0x08 (plain kind: stride 48).
    let r0 = 0x08;
    blob[r0..r0 + 8].copy_from_slice(&8u64.to_le_bytes());
    blob[r0 + 23] = KIND_INT;
    blob[r0 + 40..r0 + 44].copy_from_slice(&0x200i32.to_le_bytes());
    // main.Point record @ +0x38 (struct: base 80 + 2 fields * 24 = stride 128).
    let s = 0x38;
    blob[s..s + 8].copy_from_slice(&16u64.to_le_bytes());
    blob[s + 20] = TFLAG_NAMED;
    blob[s + 23] = KIND_STRUCT;
    blob[s + 40..s + 44].copy_from_slice(&0x210i32.to_le_bytes());
    blob[s + 56..s + 64].copy_from_slice(&(T + 0x300).to_le_bytes());
    blob[s + 64..s + 72].copy_from_slice(&2u64.to_le_bytes());
    blob[s + 72..s + 80].copy_from_slice(&2u64.to_le_bytes());
    // (the walker treats the fields array as part of the record's stride; park
    // it at +0x300, past typedesclen, as the real linker does with name data)
    // ptr record @ +0xb8 (pointer: stride 56), elem = main.Point.
    let p = 0xb8;
    blob[p..p + 8].copy_from_slice(&8u64.to_le_bytes());
    blob[p + 23] = KIND_POINTER;
    blob[p + 40..p + 44].copy_from_slice(&0x220i32.to_le_bytes());
    blob[p + 48..p + 56].copy_from_slice(&(T + s as u64).to_le_bytes());
    // five more plain records to clear the walk's acceptance floor (>= 8).
    for k in 0..5u64 {
        let r = (0xf0 + k * 48) as usize;
        blob[r..r + 8].copy_from_slice(&8u64.to_le_bytes());
        blob[r + 23] = KIND_UINT;
        blob[r + 40..r + 44].copy_from_slice(&0x200i32.to_le_bytes());
    }
    let typedesclen: u64 = 0xf0 + 5 * 48; // walk ends exactly here
    // names
    blob[0x200..0x205].copy_from_slice(&[0, 3, b'i', b'n', b't']);
    blob[0x210] = 0;
    blob[0x211] = 10;
    blob[0x212..0x21c].copy_from_slice(b"main.Point");
    blob[0x220..0x22d].copy_from_slice(&[0, 11, b'*', b'm', b'a', b'i', b'n', b'.', b'P', b'o', b'i', b'n', b't']);
    blob[0x230..0x233].copy_from_slice(&[0, 1, b'X']);
    blob[0x238..0x23b].copy_from_slice(&[0, 1, b'Y']);
    // fields array @ +0x300
    for (i, (np, off)) in [(T + 0x230, 0u64), (T + 0x238, 8u64)].iter().enumerate() {
        let f = 0x300 + i * 24;
        blob[f..f + 8].copy_from_slice(&np.to_le_bytes());
        blob[f + 8..f + 16].copy_from_slice(&(T + 0x08).to_le_bytes()); // typ = int
        blob[f + 16..f + 24].copy_from_slice(&off.to_le_bytes());
    }

    // moduledata: pcHeader*, funcnametab slice, pad, then the 1.26+ triple.
    let mut md = Vec::new();
    md.extend_from_slice(&H.to_le_bytes());
    md.extend_from_slice(&T.to_le_bytes());
    md.extend_from_slice(&4u64.to_le_bytes());
    md.extend_from_slice(&4u64.to_le_bytes());
    for _ in 0..6 {
        md.extend_from_slice(&0u64.to_le_bytes());
    }
    md.extend_from_slice(&T.to_le_bytes()); // types
    md.extend_from_slice(&typedesclen.to_le_bytes()); // typedesclen
    md.extend_from_slice(&(T + 0x400).to_le_bytes()); // etypes
    for _ in 0..8 {
        md.extend_from_slice(&0u64.to_le_bytes());
    }
    (vec![(T, blob), (M, md), (H, vec![0u8; 16])], M, T)
}

#[test]
fn descriptor_region_walk_enumerates_without_typelinks() {
    let (spans, anchor, types) = synth_descriptors();
    let img = img_of(&spans);
    // The old slice form must NOT claim this image...
    assert!(locate_type_meta(&img, anchor).is_none(), "no typelinks slice exists");
    // ...and the descriptor walk must: 8 records, main.Point among them.
    let meta = locate_type_meta_descriptors(&img, anchor).expect("descriptor triple + walk");
    assert_eq!(meta.types, types);
    assert_eq!(meta.offsets.len(), 8, "int, Point, *Point, five fillers");
    assert!(meta.offsets.contains(&0x38), "main.Point's record offset");
    let t = read_type(&img, meta.types, meta.etypes, 0x38).expect("decodes");
    assert_eq!(t.name, "main.Point");
}

#[test]
fn descriptor_stride_matches_the_abi_math() {
    let (spans, _anchor, types) = synth_descriptors();
    let img = img_of(&spans);
    let etypes = types + 0x400;
    let int_t = read_type(&img, types, etypes, 0x08).expect("int");
    assert_eq!(descriptor_stride(&img, &int_t), Some(48), "plain kind");
    let point = read_type(&img, types, etypes, 0x38).expect("struct");
    assert_eq!(descriptor_stride(&img, &point), Some(80 + 2 * 24), "struct base + fields");
    let ptr = read_type(&img, types, etypes, 0xb8).expect("ptr");
    assert_eq!(descriptor_stride(&img, &ptr), Some(56), "pointer wrapper");
}

#[test]
fn descriptor_walk_stops_conservatively_on_garbage() {
    let (mut spans, anchor, _types) = synth_descriptors();
    // Corrupt the SECOND record's kind byte: the walk must stop below the
    // acceptance floor and the locator must refuse the image whole.
    spans[0].1[0x38 + 23] = 0x1f; // invalid kind (> UnsafePointer after mask)
    let img = img_of(&spans);
    assert!(locate_type_meta_descriptors(&img, anchor).is_none());
}
