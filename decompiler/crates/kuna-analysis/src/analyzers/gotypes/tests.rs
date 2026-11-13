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

fn img_of(spans: &[(u64, Vec<u8>)]) -> GoImage<'_> {
    GoImage::from_spans(spans.iter().map(|(a, b)| (*a, b.as_slice())).collect())
}

#[test]
fn type_headers_and_slices_use_32_bit_words() {
    let mut bytes = vec![0; 128];
    bytes[0..4].copy_from_slice(&4u32.to_le_bytes());
    bytes[15] = KIND_INT;
    bytes[24..28].copy_from_slice(&64i32.to_le_bytes());
    bytes[64..69].copy_from_slice(b"\0\x03int");
    bytes[80..84].copy_from_slice(&0x1000u32.to_le_bytes());
    bytes[84..88].copy_from_slice(&2u32.to_le_bytes());
    bytes[88..92].copy_from_slice(&2u32.to_le_bytes());
    let mut img = GoImage::from_spans(vec![(0x1000, &bytes)]);
    img.word_size = 4;
    let t = read_type(&img, 0x1000, 0x1080, 0).unwrap();
    assert_eq!((t.kind, t.size, t.name.as_str()), (KIND_INT, 4, "int"));
    assert_eq!(plausible_slice(&img, 0x1050, 2), Some((0x1000, 2)));
    assert!(read_type(&img, 0x1000, 0x101f, 0).is_none());
    assert!(descriptor_stride(&img, &t).is_none(), "newer 32-bit descriptor regions are not guessed");
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
fn struct_offsets_follow_the_toolchain_layout() {
    for (version, encoded, expected_size) in [
        (Some((1, 18)), 17u64, 16),
        (Some((1, 19)), 8, 16),
        (Some((1, 27)), 8, 16),
        (None, 8, 16),
        (None, 17, 0),
        (Some((1, 18)), 41, 0),
    ] {
        let (mut spans, anchor, _, _) = synth();
        spans[0].1[0x328..0x330].copy_from_slice(&encoded.to_le_bytes());
        let img = img_of(&spans);
        let meta = locate_type_meta(&img, anchor).unwrap();
        let tf = kuna_decomp::dtype::TypeFactoryImpl::new();
        tf.set_default_alignment_map();
        tf.set_max_basetype_size(8);
        tf.setup_sizes(Some(8), 8, 8);
        tf.cache_core_types().unwrap();
        intern_named_structs(&tf, &img, &meta, version);
        let pt = tf.find_by_name("main.Point").unwrap().unwrap();
        assert_eq!(pt.get_size(), expected_size, "{version:?}, encoded={encoded}");
        if expected_size != 0 {
            for (i, (name, offset)) in [("X", 0), ("Y", 8)].into_iter().enumerate() {
                let field = pt.get_field(i as i32).unwrap();
                assert_eq!((field.name.as_str(), field.offset), (name, offset));
            }
        }
    }
}

#[test]
fn nested_value_fields_complete_even_without_their_own_typelink() {
    for (cyclic, bad_extent) in [(false, false), (true, false), (false, true)] {
        let (mut spans, anchor, types, _) = synth();
        let blob = &mut spans[0].1;
        let child = 0xb0;
        blob[child..child + 8].copy_from_slice(&8u64.to_le_bytes());
        blob[child + 20] = TFLAG_NAMED;
        blob[child + 23] = KIND_STRUCT;
        blob[child + 40..child + 44].copy_from_slice(&0x260i32.to_le_bytes());
        blob[child + 56..child + 64].copy_from_slice(&(types + 0x360).to_le_bytes());
        blob[child + 64..child + 72].copy_from_slice(&1u64.to_le_bytes());
        blob[child + 72..child + 80].copy_from_slice(&1u64.to_le_bytes());
        blob[0x260..0x268].copy_from_slice(b"\0\x06main.Z");
        blob[0x320..0x328].copy_from_slice(&(types + child as u64).to_le_bytes());
        blob[0x360..0x368].copy_from_slice(&(types + 0x230).to_le_bytes());
        let child_field = if cyclic { types + 0x60 } else { types };
        blob[0x368..0x370].copy_from_slice(&child_field.to_le_bytes());
        if bad_extent {
            blob[0x370..0x378].copy_from_slice(&8u64.to_le_bytes());
        }
        let img = img_of(&spans);
        let meta = locate_type_meta(&img, anchor).unwrap();
        assert!(!meta.offsets.contains(&(child as u32)));
        let tf = kuna_decomp::dtype::TypeFactoryImpl::new();
        tf.set_default_alignment_map();
        tf.set_max_basetype_size(8);
        tf.setup_sizes(Some(8), 8, 8);
        tf.cache_core_types().unwrap();
        intern_named_structs(&tf, &img, &meta, Some((1, 22)));
        let pt = tf.find_by_name("main.Point").unwrap().unwrap();
        if cyclic || bad_extent {
            assert_eq!(pt.get_size(), 0, "an invalid value dependency keeps the parent opaque");
        } else {
            let field = pt.get_field(1).expect("the nested value field survives");
            assert_eq!((field.name.as_str(), field.offset), ("Y", 8));
            assert_eq!(field.field_type.get_name(), "main.Z");
            assert_eq!(field.field_type.get_size(), 8);
            assert_eq!(field.field_type.get_field(0).unwrap().name, "X");
        }
    }
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

#[test]
fn pointer_chains_stop_at_cycles_missing_records_and_depth_limit() {
    for width in [4usize, 8] {
        for (length, target, depth, leaf) in [
            (1, Some(0), 1, type_metatype::TYPE_VOID),
            (2, Some(0), 2, type_metatype::TYPE_VOID),
            (3, Some(1), 3, type_metatype::TYPE_VOID),
            (1, Some(8), 1, type_metatype::TYPE_VOID),
            (2, None, 2, type_metatype::TYPE_UINT),
            (130, None, 64, type_metatype::TYPE_VOID),
        ] {
            let types = 0x100000u64;
            let stride = 96;
            let name = (length + 1) * stride + 16;
            let mut bytes = vec![0u8; (length + 2) * stride];
            bytes[name..name + 8].copy_from_slice(b"\0\x06main.P");
            for index in 0..=length {
                let off = index * stride;
                let size = if index == length { 4u64 } else { width as u64 };
                bytes[off..off + width].copy_from_slice(&size.to_le_bytes()[..width]);
                bytes[off + 2 * width + 7] = if index == length { KIND_UINT32 } else { KIND_POINTER };
                bytes[off + 4 * width + 8..off + 4 * width + 12].copy_from_slice(&(name as i32).to_le_bytes());
                let next = if index + 1 == length { target.unwrap_or(index + 1) } else { index + 1 };
                let ptr = types + (next * stride) as u64;
                let elem = off + 4 * width + 16;
                bytes[elem..elem + width].copy_from_slice(&ptr.to_le_bytes()[..width]);
            }
            let meta = TypeMeta { types, etypes: types + bytes.len() as u64, offsets: vec![] };
            let mut image = GoImage::from_spans(vec![(types, &bytes)]);
            image.word_size = width as u64;
            let tf = kuna_decomp::dtype::TypeFactoryImpl::new();
            tf.set_default_alignment_map();
            tf.set_max_basetype_size(8);
            tf.setup_sizes(Some(width as i32), width as i32, width as i32);
            tf.cache_core_types().unwrap();
            let mut ty = field_type(&tf, &image, &meta, types).unwrap();
            for _ in 0..depth {
                assert_eq!(ty.get_size(), width as i32);
                ty = ty.get_ptr_to().expect("retains each proven pointer layer");
            }
            assert_eq!(ty.get_metatype(), leaf, "width {width}, length {length}, target {target:?}");
            if leaf == type_metatype::TYPE_UINT { assert_eq!(ty.get_size(), 4); }
        }
    }
}

#[test]
fn forward_struct_pointers_observe_only_valid_completed_layouts() {
    for bad_extent in [false, true] {
        let (mut spans, anchor, types, _) = synth();
        let blob = &mut spans[0].1;
        let pointer = 0xb0;
        blob[pointer..pointer + 8].copy_from_slice(&8u64.to_le_bytes());
        blob[pointer + 23] = KIND_POINTER;
        blob[pointer + 40..pointer + 44].copy_from_slice(&0x250i32.to_le_bytes());
        blob[pointer + 48..pointer + 56].copy_from_slice(&(types + 0x60).to_le_bytes());
        blob[0x250..0x25d].copy_from_slice(b"\0\x0b*main.Point");
        blob[0x320..0x328].copy_from_slice(&(types + pointer as u64).to_le_bytes());
        if bad_extent {
            blob[0x328..0x330].copy_from_slice(&16u64.to_le_bytes());
        }
        let img = img_of(&spans);
        let meta = locate_type_meta(&img, anchor).unwrap();
        let tf = kuna_decomp::dtype::TypeFactoryImpl::new();
        tf.set_default_alignment_map();
        tf.set_max_basetype_size(8);
        tf.setup_sizes(Some(8), 8, 8);
        tf.cache_core_types().unwrap();
        let forward = field_type(&tf, &img, &meta, types + pointer as u64).unwrap();
        assert_eq!(forward.get_ptr_to().unwrap().get_size(), 0);
        intern_named_structs(&tf, &img, &meta, Some((1, 22)));
        let pointee = forward.get_ptr_to().unwrap();
        if bad_extent {
            assert!(pointee.is_incomplete());
            assert_eq!(pointee.get_size(), 0);
        } else {
            assert_eq!(pointee.get_size(), 16);
            assert_eq!(pointee.get_field(0).unwrap().name, "X");
            let next = pointee.get_field(1).unwrap().field_type.get_ptr_to().unwrap();
            assert!(Rc::ptr_eq(&pointee, &next));
        }
    }
}

#[test]
fn global_name_vocabulary_demands_printable_spaceless() {
    assert!(kuna_typeglobals::global_name_ok("main.Point"));
    assert!(kuna_typeglobals::global_name_ok("*main.Point"), "pointer spelling kept");
    assert!(kuna_typeglobals::global_name_ok("[]uint8"), "unnamed composite kept");
    assert!(kuna_typeglobals::global_name_ok("map[string]int"));
    assert!(!kuna_typeglobals::global_name_ok("struct { x int }"), "interior space");
    assert!(!kuna_typeglobals::global_name_ok("chan int"), "interior space");
    assert!(!kuna_typeglobals::global_name_ok(""), "empty");
    assert!(!kuna_typeglobals::global_name_ok("a\u{00e9}b"), "non-ASCII byte");
}

#[test]
fn descriptor_globals_name_and_type_every_printable_record() {
    let (mut spans, anchor, types, _) = synth();
    {
        let blob = &mut spans[0].1;
        // Pointer record @ +0xc0: `*main.Point`, elem = the struct @ +0x60.
        let p = 0xc0;
        blob[p..p + 8].copy_from_slice(&8u64.to_le_bytes());
        blob[p + 23] = KIND_POINTER;
        blob[p + 40..p + 44].copy_from_slice(&0x250i32.to_le_bytes());
        blob[p + 48..p + 56].copy_from_slice(&(types + 0x60).to_le_bytes());
        blob[0x250..0x25d].copy_from_slice(b"\0\x0b*main.Point");
        // Spacey record @ +0x120: `chan int` — decodes but cannot print.
        let c = 0x120;
        blob[c..c + 8].copy_from_slice(&8u64.to_le_bytes());
        blob[c + 23] = KIND_CHAN;
        blob[c + 40..c + 44].copy_from_slice(&0x260i32.to_le_bytes());
        blob[0x260..0x26a].copy_from_slice(b"\0\x08chan int");
    }
    {
        // Typelinks grow to four entries: int, struct, pointer, chan — the
        // struct is now reached twice (its own link and the pointer's elem),
        // which is the dedup leg.
        let links = &mut spans[1].1;
        links.extend_from_slice(&0xc0i32.to_le_bytes());
        links.extend_from_slice(&0x120i32.to_le_bytes());
        let md = &mut spans[2].1;
        md[120..128].copy_from_slice(&4u64.to_le_bytes());
        md[128..136].copy_from_slice(&4u64.to_le_bytes());
    }
    let img = img_of(&spans);
    let meta = locate_type_meta(&img, anchor).expect("triple still validates");
    let tf = kuna_decomp::dtype::TypeFactoryImpl::new();
    tf.set_default_alignment_map();
    tf.set_max_basetype_size(8);
    tf.setup_sizes(Some(8), 8, 8);
    tf.cache_core_types().unwrap();
    let facts = kuna_typeglobals::collect(&tf, &img, &meta);
    let named: Vec<(u64, &str)> = facts.iter().map(|f| (f.addr, f.name.as_str())).collect();
    // Address order, deduplicated, the pointer POINTEE present exactly once
    // (its own typelink and the pointer walk both reach it), the spacey
    // record dropped.
    assert_eq!(
        named,
        vec![
            (types, "type:int"),
            (types + 0x60, "type:main.Point"),
            (types + 0xc0, "type:*main.Point"),
        ]
    );
    // Every fact carries the completed `go.typedesc` shell: the abi.Type base
    // record, 48 bytes on a 64-bit image, decode-offset field positions.
    for f in &facts {
        assert_eq!(f.type_.get_name(), "go.typedesc");
        assert_eq!(f.type_.get_size(), 48);
    }
    let desc = tf.find_by_name("go.typedesc").unwrap().unwrap();
    for (i, (name, offset)) in [
        ("Size_", 0),
        ("PtrBytes", 8),
        ("Hash", 16),
        ("TFlag", 20),
        ("Align_", 21),
        ("FieldAlign_", 22),
        ("Kind_", 23),
        ("Equal", 24),
        ("GCData", 32),
        ("Str", 40),
        ("PtrToThis", 44),
    ]
    .into_iter()
    .enumerate()
    {
        let field = desc.get_field(i as i32).unwrap();
        assert_eq!((field.name.as_str(), field.offset), (name, offset));
    }
}

// --- itab consumer (kuna_itabs) ---------------------------------------------

/// Build a types blob with a named concrete struct (`main.Point` @ +0x60) and
/// a two-method interface (`fmt.Stringer` @ +0x100), plus one itab record at
/// `I` binding them. Returns (spans, types, etypes, itab_vma, inter_vma,
/// concrete_vma). The itab's `Fun[0]` is non-zero so its `Size()` includes the
/// method tail.
fn synth_itab_blob() -> (Vec<(u64, Vec<u8>)>, u64, u64, u64, u64, u64) {
    const T: u64 = 0x100000;
    const I: u64 = 0x240000; // itab record
    let mut blob = vec![0u8; 0x400];
    // concrete main.Point @ +0x60: named struct, size 16.
    let c = 0x60usize;
    blob[c..c + 8].copy_from_slice(&16u64.to_le_bytes());
    blob[c + 20] = TFLAG_NAMED;
    blob[c + 23] = KIND_STRUCT;
    blob[c + 40..c + 44].copy_from_slice(&0x210i32.to_le_bytes());
    blob[0x210] = 0;
    blob[0x211] = 10;
    blob[0x212..0x21c].copy_from_slice(b"main.Point");
    // interface fmt.Stringer @ +0x100: kind INTERFACE, named, Methods len 2 at
    // +0x100 + type_base(48) + 2*word(16) = +0x150.
    let n = 0x100usize;
    blob[n..n + 8].copy_from_slice(&0u64.to_le_bytes());
    blob[n + 20] = TFLAG_NAMED;
    blob[n + 23] = KIND_INTERFACE;
    blob[n + 40..n + 44].copy_from_slice(&0x220i32.to_le_bytes());
    blob[0x220] = 0;
    blob[0x221] = 12;
    blob[0x222..0x22e].copy_from_slice(b"fmt.Stringer");
    // Methods slice sits after Type base (48) + PkgPath Name ptr (8): ptr @
    // inter+56, len @ inter+64, cap @ inter+72 (blob 0x138/0x140/0x148).
    blob[0x138..0x140].copy_from_slice(&0u64.to_le_bytes()); // Methods.ptr
    blob[0x140..0x148].copy_from_slice(&2u64.to_le_bytes()); // Methods.len = 2
    blob[0x148..0x150].copy_from_slice(&2u64.to_le_bytes()); // cap
    // itab record @ I: Inter=T+0x100, Type=T+0x60, Hash, Fun[0]!=0.
    let mut itab = vec![0u8; 0x40];
    itab[0..8].copy_from_slice(&(T + 0x100).to_le_bytes());
    itab[8..16].copy_from_slice(&(T + 0x60).to_le_bytes());
    itab[16..20].copy_from_slice(&0xdeadu32.to_le_bytes());
    itab[24..32].copy_from_slice(&0x401000u64.to_le_bytes()); // Fun[0] != 0
    let spans = vec![(T, blob), (I, itab)];
    (spans, T, T + 0x400, I, T + 0x100, T + 0x60)
}

#[test]
fn itab_record_decodes_to_its_two_type_names() {
    let (spans, types, etypes, itab, _inter, _concrete) = synth_itab_blob();
    let img = img_of(&spans);
    let (c, i) = super::kuna_itabs::read_itab(&img, types, etypes, itab).expect("itab decodes");
    assert_eq!((c.name.as_str(), i.name.as_str()), ("main.Point", "fmt.Stringer"));
    assert_eq!(i.kind, KIND_INTERFACE);
    // Inter not an interface descriptor -> refuse (swap the two pointers).
    let mut bad = spans.clone();
    bad[1].1[0..8].copy_from_slice(&(types + 0x60).to_le_bytes()); // Inter = struct
    assert!(super::kuna_itabs::read_itab(&img_of(&bad), types, etypes, itab).is_none());
    // A pointer outside the blob -> refuse.
    let mut oob = spans.clone();
    oob[1].1[0..8].copy_from_slice(&0xffff_0000u64.to_le_bytes());
    assert!(super::kuna_itabs::read_itab(&img_of(&oob), types, etypes, itab).is_none());
}

#[test]
fn itab_size_covers_method_tail_and_placeholder() {
    let (spans, _t, _e, itab, inter, _c) = synth_itab_blob();
    let img = img_of(&spans);
    // 2-method interface, Fun[0]!=0: header(32) + (2-1)*8 = 40.
    assert_eq!(super::kuna_itabs::itab_size(&img, itab, inter), Some(40));
    // Fun[0]==0 placeholder: header only (32).
    let mut ph = spans.clone();
    ph[1].1[24..32].copy_from_slice(&0u64.to_le_bytes());
    assert_eq!(super::kuna_itabs::itab_size(&img_of(&ph), itab, inter), Some(32));
    assert_eq!(super::kuna_itabs::interface_method_count(&img, inter), Some(2));
}

#[test]
fn locate_itablinks_slice_follows_pointers() {
    // go<=1.25 form: a moduledata window holding an `itablinks` slice of
    // *itab pointers; `collect`-style location finds it by validation.
    let (mut spans, types, etypes, itab, _i, _c) = synth_itab_blob();
    const L: u64 = 0x250000; // itablinks array (one *itab)
    const M: u64 = 0x260000; // moduledata window
    spans.push((L, itab.to_le_bytes().to_vec()));
    let mut md = Vec::new();
    md.extend_from_slice(&L.to_le_bytes()); // slice.ptr
    md.extend_from_slice(&1u64.to_le_bytes()); // len
    md.extend_from_slice(&1u64.to_le_bytes()); // cap
    spans.push((M, md));
    let img = img_of(&spans);
    let found = super::kuna_itabs::locate_itablinks_slice(&img, M, types, etypes).expect("slice");
    assert_eq!(found, vec![itab]);
}

#[test]
fn locate_itab_region_walks_the_go126_contiguous_region() {
    // go1.26+ form: two consecutive itab records walked by Size(), located via
    // an (itaboffset, itabsize) pair in the moduledata window.
    let (spans0, types, etypes, _itab0, inter, concrete) = synth_itab_blob();
    const R: u64 = 0x270000; // contiguous itab region
    // Two itabs back to back: first 40 bytes (2-method, Fun!=0), second 32
    // (placeholder, Fun==0). Total 72.
    let mut region = vec![0u8; 72];
    region[0..8].copy_from_slice(&inter.to_le_bytes());
    region[8..16].copy_from_slice(&concrete.to_le_bytes());
    region[24..32].copy_from_slice(&0x401000u64.to_le_bytes()); // Fun[0]!=0 -> size 40
    region[40..48].copy_from_slice(&inter.to_le_bytes());
    region[48..56].copy_from_slice(&concrete.to_le_bytes());
    // second Fun[0]==0 -> size 32
    let mut spans = spans0;
    let itaboffset = R - types;
    spans.push((R, region));
    const M: u64 = 0x280000;
    let mut md = Vec::new();
    md.extend_from_slice(&itaboffset.to_le_bytes()); // itaboffset (from types)
    md.extend_from_slice(&72u64.to_le_bytes()); // itabsize
    spans.push((M, md));
    let img = img_of(&spans);
    let found = super::kuna_itabs::locate_itab_region(&img, M, types, etypes).expect("region");
    assert_eq!(found, vec![R, R + 40]);
    // A wrong itabsize that does not land on a record boundary is refused.
    let mut bad = spans.clone();
    let mi = bad.iter().position(|(a, _)| *a == M).unwrap();
    bad[mi].1[8..16].copy_from_slice(&71u64.to_le_bytes());
    assert!(super::kuna_itabs::locate_itab_region(&img_of(&bad), M, types, etypes).is_none());
}

#[test]
fn collect_emits_the_linker_itab_symbol() {
    let (mut spans, types, etypes, itab, _i, _c) = synth_itab_blob();
    const L: u64 = 0x250000;
    const M: u64 = 0x260000;
    spans.push((L, itab.to_le_bytes().to_vec()));
    let mut md = Vec::new();
    md.extend_from_slice(&L.to_le_bytes());
    md.extend_from_slice(&1u64.to_le_bytes());
    md.extend_from_slice(&1u64.to_le_bytes());
    spans.push((M, md));
    let img = img_of(&spans);
    let meta = TypeMeta { types, etypes, offsets: vec![] };
    let facts = super::kuna_itabs::collect(&img, &meta, M, &std::collections::HashSet::new());
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].addr, itab);
    assert_eq!(facts[0].name, "go:itab.main.Point,fmt.Stringer");
    assert_eq!(facts[0].kind, crate::pass::SymKind::Data);
}
