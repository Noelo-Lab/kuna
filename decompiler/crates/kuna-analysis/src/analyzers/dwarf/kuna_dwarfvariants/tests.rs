//! Unit tests for the DWARF variant-part importer.
//!
//! Every public entry point is exercised DIRECTLY (`read_variant_part`,
//! `variant_part`, `intern_variant_aggregate`), and every refusal in the module
//! header has a test that exercises the REFUSAL, not only the happy path.
//!
//! The DIE trees are forged rather than read out of a fixture on purpose: a
//! refusal has to be provable for shapes no compiler in the corpus emits
//! (a nested variant part, two default variants, a repeated discriminant), and a
//! forged tree is also the only way to put two SAME-NAMED, DIFFERENT-SIZED
//! payload structs in one type factory. The shapes rustc does emit are
//! transcribed from `readelf --debug-dump=info` of the committed
//! `dwarfvariants_x86_64` fixture and pinned end to end by
//! `kuna-console/tests/verify_dwarfvariants.rs`.
//!
//! The NAMING rule (which variant names may reach the emitted type at all) has
//! its refusals here too, not only its keeps: `result_overlays_so_neither_variant_is_named`,
//! `tree_style_overlay_names_no_variant_but_keeps_the_uncontested_field`,
//! `default_variant_is_suppressed_when_it_overlays_another`,
//! `builds_multi_field_variant` and `builds_five_variants` are the refusals;
//! `option_keeps_its_only_payload_variant_name`,
//! `disjoint_payload_variants_keep_both_names` and
//! `agreeing_field_names_survive_even_when_the_facet_does_not` are the keeps; and
//! `seals_when_every_suppressed_facet_name_is_taken` is the post-intern refusal
//! the offset-derived facet name introduces.

use super::*;
use kuna_decomp::dtype::TypeFactoryImpl;

/// A configured [`TypeFactory`] for the mapper unit tests (same recipe as
/// `super::super::tests::factory`).
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.setup_sizes(Some(8), 8, 4);
    let _ = f.cache_core_types();
    f
}

/// Builder for a forged DIE tree, offsets assigned in insertion order.
struct Tree {
    dies: BTreeMap<usize, DieSnap>,
    next: usize,
}

impl Tree {
    fn new() -> Self {
        Tree { dies: BTreeMap::new(), next: 1 }
    }

    fn add(&mut self, parent: Option<usize>, mut d: DieSnap) -> usize {
        let off = self.next;
        self.next += 1;
        d.parent = parent;
        if let Some(p) = parent {
            if let Some(pd) = self.dies.get_mut(&p) {
                pd.children.push(off);
            }
        }
        self.dies.insert(off, d);
        off
    }

    /// A `DW_TAG_base_type` of `size` bytes, unsigned.
    fn uint(&mut self, size: u64) -> usize {
        let mut d = DieSnap::new(gimli::DW_TAG_base_type, 1);
        d.name = format!("u{}", size * 8);
        d.byte_size = Some(size);
        d.encoding = Some(gimli::DW_ATE_unsigned);
        self.add(None, d)
    }

    /// A pointer to `to`.
    fn ptr(&mut self, to: usize) -> usize {
        let mut d = DieSnap::new(gimli::DW_TAG_pointer_type, 1);
        d.type_ref = Some(to);
        d.byte_size = Some(8);
        self.add(None, d)
    }

    /// A named `DW_TAG_structure_type` of `size` bytes.
    fn strukt(&mut self, parent: Option<usize>, name: &str, size: u64) -> usize {
        let mut d = DieSnap::new(gimli::DW_TAG_structure_type, 1);
        d.name = name.into();
        d.byte_size = Some(size);
        self.add(parent, d)
    }

    /// A named `DW_TAG_union_type` of `size` bytes.
    fn union(&mut self, parent: Option<usize>, name: &str, size: u64) -> usize {
        let mut d = DieSnap::new(gimli::DW_TAG_union_type, 1);
        d.name = name.into();
        d.byte_size = Some(size);
        self.add(parent, d)
    }

    /// A `DW_TAG_member` at `off` of type `ty`.
    fn member(&mut self, parent: usize, name: &str, ty: usize, off: i64) -> usize {
        let mut d = DieSnap::new(gimli::DW_TAG_member, 1);
        d.name = name.into();
        d.type_ref = Some(ty);
        d.data_member_location = Some(off);
        self.add(Some(parent), d)
    }

    /// A `DW_TAG_variant_part` under `parent`, with its artificial discriminant
    /// member of type `tag_ty` at `tag_off`.
    fn variant_part(&mut self, parent: usize, tag_ty: usize, tag_off: i64) -> usize {
        let vp = self.add(Some(parent), DieSnap::new(gimli::DW_TAG_variant_part, 1));
        let mut tag = DieSnap::new(gimli::DW_TAG_member, 1);
        tag.type_ref = Some(tag_ty);
        tag.data_member_location = Some(tag_off);
        let toff = self.add(Some(vp), tag);
        self.dies.get_mut(&vp).unwrap().discr_ref = Some(toff);
        vp
    }

    /// A `DW_TAG_variant` under `vp` with discriminant `discr` (`None` = the
    /// default variant), whose member `name` refers to the payload struct
    /// `payload`.
    fn variant(&mut self, vp: usize, discr: Option<i64>, name: &str, payload: usize) -> usize {
        let mut v = DieSnap::new(gimli::DW_TAG_variant, 1);
        v.discr_value = discr;
        let voff = self.add(Some(vp), v);
        self.member(voff, name, payload, 0);
        voff
    }
}

/// `Result<u32,u32>` exactly as `rustc 1.90 -C debuginfo=2` emits it: an 8-byte
/// struct, a `variant_part` whose `DW_AT_discr` is an artificial u32 at 0, and
/// two variants at 0/1 whose payload structs are 8 bytes with `__0` at 4.
fn result_u32(t: &mut Tree) -> usize {
    let u32t = t.uint(4);
    let e = t.strukt(None, "Result<u32, u32>", 8);
    let vp = t.variant_part(e, u32t, 0);
    let ok = t.strukt(Some(e), "Ok", 8);
    t.member(ok, "__0", u32t, 4);
    let err = t.strukt(Some(e), "Err", 8);
    t.member(err, "__0", u32t, 4);
    t.variant(vp, Some(0), "Ok", ok);
    t.variant(vp, Some(1), "Err", err);
    e
}

// ---------------------------------------------------------------------------
// read_variant_part / variant_part -- the pure DWARF read
// ---------------------------------------------------------------------------

/// The Result shape reads back exactly: tag u32 at 0, `Ok`=0 and `Err`=1, each
/// with one field `__0` at ENUM offset 4.
#[test]
fn reads_result_variant_part() {
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    let raw = read_variant_part(&t.dies[&e], &t.dies).expect("Result must read");
    assert_eq!(raw.tag_offset, 0);
    assert_eq!(raw.variants.len(), 2);
    assert_eq!(raw.variants[0].name, "Ok");
    assert_eq!(raw.variants[0].discr, Some(0));
    assert_eq!(raw.variants[0].fields.len(), 1);
    assert_eq!(raw.variants[0].fields[0].0, "__0");
    assert_eq!(raw.variants[0].fields[0].1, 4);
    assert_eq!(raw.variants[1].name, "Err");
    assert_eq!(raw.variants[1].discr, Some(1));
}

/// A `DW_TAG_variant` with NO `DW_AT_discr_value` is the DEFAULT variant (the
/// niche encoding) and must survive the read as `discr: None`, not be dropped.
#[test]
fn reads_niche_default_variant() {
    let mut t = Tree::new();
    let u64t = t.uint(8);
    let refu = t.ptr(u64t);
    let e = t.strukt(None, "Option<&u32>", 8);
    let vp = t.variant_part(e, u64t, 0);
    let none = t.strukt(Some(e), "None", 8);
    let some = t.strukt(Some(e), "Some", 8);
    t.member(some, "__0", refu, 0);
    t.variant(vp, Some(0), "None", none);
    t.variant(vp, None, "Some", some);

    let raw = read_variant_part(&t.dies[&e], &t.dies).expect("niche Option must read");
    assert_eq!(raw.variants.len(), 2);
    assert_eq!(raw.variants[0].discr, Some(0));
    assert_eq!(raw.variants[1].discr, None, "the default variant keeps its absence");
    assert_eq!(raw.variants[1].fields[0].1, 0, "the payload starts where the tag does");
}

/// An ordinary C aggregate has no `DW_TAG_variant_part`, so both entry points
/// hand it straight back to the caller.
#[test]
fn plain_struct_is_not_a_variant_part() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let s = t.strukt(None, "Pair", 8);
    t.member(s, "lo", u32t, 0);
    t.member(s, "hi", u32t, 4);
    assert!(variant_part(&t.dies[&s], &t.dies).is_none());
    assert!(read_variant_part(&t.dies[&s], &t.dies).is_none());
}

/// Two variant parts in one aggregate is a shape no producer emits; refuse it
/// rather than pick one.
#[test]
fn refuses_two_variant_parts() {
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    let u32t = t.uint(4);
    t.variant_part(e, u32t, 0);
    assert!(variant_part(&t.dies[&e], &t.dies).is_none());
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// No `DW_AT_discr` means DWARF never said which member is the tag. That is the
/// `core::convert::Infallible` / single-variant shape, and it is refused.
#[test]
fn refuses_missing_discr() {
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    let vpoff = *t.dies[&e]
        .children
        .iter()
        .find(|c| t.dies[c].tag == gimli::DW_TAG_variant_part)
        .unwrap();
    t.dies.get_mut(&vpoff).unwrap().discr_ref = None;
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// A `DW_AT_discr` pointing at something that is not a `DW_TAG_member`.
#[test]
fn refuses_discr_that_is_not_a_member() {
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    let u32t = t.uint(4);
    let vpoff = *t.dies[&e]
        .children
        .iter()
        .find(|c| t.dies[c].tag == gimli::DW_TAG_variant_part)
        .unwrap();
    t.dies.get_mut(&vpoff).unwrap().discr_ref = Some(u32t);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// Two variants with no discriminant value: DWARF admits at most one default.
#[test]
fn refuses_two_default_variants() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "TwoDefaults", 8);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 8);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 8);
    t.member(b, "__0", u32t, 4);
    t.variant(vp, None, "A", a);
    t.variant(vp, None, "B", b);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// Two variants claiming the same discriminant value.
#[test]
fn refuses_repeated_discr_value() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "Dup", 8);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 8);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 8);
    t.member(b, "__0", u32t, 4);
    t.variant(vp, Some(1), "A", a);
    t.variant(vp, Some(1), "B", b);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// Two variants with the same NAME would collapse into one union facet.
#[test]
fn refuses_repeated_variant_name() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "DupName", 8);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 8);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 8);
    t.member(b, "__0", u32t, 4);
    t.variant(vp, Some(0), "Same", a);
    t.variant(vp, Some(1), "Same", b);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// A variant with two member children, or with none, is not the one-member shape
/// this module reads.
#[test]
fn refuses_variant_with_wrong_member_count() {
    for extra in [0usize, 2usize] {
        let mut t = Tree::new();
        let u32t = t.uint(4);
        let e = t.strukt(None, "Odd", 8);
        let vp = t.variant_part(e, u32t, 0);
        let a = t.strukt(Some(e), "A", 8);
        t.member(a, "__0", u32t, 4);
        let mut v = DieSnap::new(gimli::DW_TAG_variant, 1);
        v.discr_value = Some(0);
        let voff = t.add(Some(vp), v);
        for i in 0..extra {
            t.member(voff, &format!("A{i}"), a, 0);
        }
        assert!(
            read_variant_part(&t.dies[&e], &t.dies).is_none(),
            "a variant with {extra} members must be refused"
        );
    }
}

/// A NESTED `DW_TAG_variant_part` inside a payload struct: the payload's own
/// fields would then be discriminant-selected, which the single-level overlay
/// cannot describe. Refused (rustc 1.90 does not emit it -- see `variant_fields`).
#[test]
fn refuses_nested_variant_part() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "OuterNest", 16);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 16);
    t.member(a, "__0", u32t, 4);
    // A's payload gets its own variant part.
    t.variant_part(a, u32t, 8);
    let b = t.strukt(Some(e), "B", 16);
    t.member(b, "__0", u32t, 4);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

/// A variant whose payload is not a structure type at all.
#[test]
fn refuses_non_struct_payload() {
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "BadPayload", 8);
    let vp = t.variant_part(e, u32t, 0);
    t.variant(vp, Some(0), "A", u32t);
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_none());
}

// ---------------------------------------------------------------------------
// intern_variant_aggregate -- the built type
// ---------------------------------------------------------------------------

/// Drive the importer over `die` with a default (`typedepth on`) walk.
fn intern(types: &TypeFactoryImpl, t: &Tree, die: usize) -> Option<Rc<Datatype>> {
    let mut walk = TypeWalk::with_gate(true);
    intern_variant_aggregate(
        types,
        &t.dies[&die],
        &t.dies,
        None,
        "anon_struct",
        &mut walk,
        1,
        false,
    )
}

/// Field `(name, offset, size)` triples of a struct/union type.
fn fields(t: &Rc<Datatype>) -> Vec<(String, int4, int4)> {
    (0..t.num_depend())
        .filter_map(|i| t.get_field(i))
        .map(|f| (f.name.clone(), f.offset, f.field_type.get_size()))
        .collect()
}

/// `Result<u32,u32>` becomes `struct { u32 tag @0; union payload @4 }` — and the
/// union's facets are NOT named `Ok` and `Err`. Both variants place a field at
/// offset 4, the union member selects itself by offset, and nothing consults the
/// discriminant, so either name would be a coin flip. The offset spelling goes in
/// instead. The shared `__0` survives: both variants spell that range the same
/// way, so it claims nothing about which one is live.
#[test]
fn result_overlays_so_neither_variant_is_named() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    let built = intern(&types, &t, e).expect("Result must build");

    assert_eq!(built.get_name(), "Result<u32, u32>");
    assert_eq!(built.get_size(), 8);
    assert_eq!(built.get_metatype(), type_metatype::TYPE_STRUCT);
    let f = fields(&built);
    assert_eq!(f.len(), 2, "tag + payload: {f:?}");
    assert_eq!(f[0], ("tag".to_string(), 0, 4));
    assert_eq!(f[1].0, "payload");
    assert_eq!(f[1].1, 4);
    assert_eq!(f[1].2, 4, "the overlay is the enum minus the tag");

    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(payload.get_metatype(), type_metatype::TYPE_UNION);
    let uf = fields(&payload);
    assert_eq!(
        uf.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["field_0x4", "field_0x4"],
        "two variants at one offset name neither: {uf:?}"
    );
    assert!(uf.iter().all(|x| x.1 == 0), "a union facet is an overlay at 0: {uf:?}");

    // The facet's own field is re-based off the payload offset: DWARF put `__0`
    // at enum offset 4, the overlay starts at 4, so inside the facet it is 0.
    let ok = payload.get_field(0).unwrap().field_type.clone();
    assert_eq!(fields(&ok), vec![("__0".to_string(), 0, 4)]);
    assert_eq!(ok.get_name(), "Result<u32, u32>::field_0x4");
    // The two facets describe the same bytes, so they share one interned struct.
    let err = payload.get_field(1).unwrap().field_type.clone();
    assert_eq!(err.get_name(), "Result<u32, u32>::field_0x4");

    // Nothing named `Ok` or `Err` was minted at all -- a cast in the emitted C
    // prints the FACET TYPE's name, so suppressing only the member name would
    // still leak the variant.
    for n in ["Result<u32, u32>::Ok", "Result<u32, u32>::Err"] {
        assert!(types.find_by_name(n).unwrap().is_none(), "{n} must not exist");
    }
    // The source names are not lost -- they are on the side table.
    let l = types.kuna_variant_layout("Result<u32, u32>").unwrap();
    assert_eq!(l.facet_for_discr(0).unwrap().name, "Ok");
    assert_eq!(l.facet_for_discr(0).unwrap().label, "field_0x4");
    assert!(!l.names_any_variant());
}

/// The side table records what the type cannot: which value selects which
/// variant, and where the tag is.
#[test]
fn records_the_layout_side_table() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    intern(&types, &t, e).unwrap();

    let l = types.kuna_variant_layout("Result<u32, u32>").expect("layout recorded");
    assert_eq!(l.size, 8);
    assert_eq!((l.tag_offset, l.tag_size), (0, 4));
    assert_eq!(l.payload_offset, 4);
    assert!(!l.niche);
    assert_eq!(l.union_type, "Result<u32, u32>::payload");
    assert_eq!(l.facet_for_discr(0).unwrap().name, "Ok");
    assert_eq!(l.facet_for_discr(1).unwrap().name, "Err");
    assert!(l.facet_for_discr(2).is_none(), "no default variant here");
    // Field offsets in the table are ABSOLUTE within the enum, not re-based.
    assert_eq!(l.facet_named("Err").unwrap().fields, vec![("__0".to_string(), 4)]);
    assert_eq!(types.kuna_variant_layouts().len(), 1);
}

/// `Option<u32>`: `None` is a real, named, FIELDLESS facet -- it keeps its name
/// and simply has no members.
#[test]
fn builds_option_with_a_fieldless_variant() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "Option<u32>", 8);
    let vp = t.variant_part(e, u32t, 0);
    let none = t.strukt(Some(e), "None", 8);
    let some = t.strukt(Some(e), "Some", 8);
    t.member(some, "__0", u32t, 4);
    t.variant(vp, Some(0), "None", none);
    t.variant(vp, Some(1), "Some", some);

    let built = intern(&types, &t, e).expect("Option must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    let uf = fields(&payload);
    assert_eq!(
        uf.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["Some"],
        "a fieldless variant overlays nothing, so it gets no union member: {uf:?}"
    );
    let somet = payload.get_field(0).unwrap().field_type.clone();
    assert_eq!(fields(&somet), vec![("__0".to_string(), 0, 4)]);

    // `None` is not lost -- it is on the side table, with its discriminant, which
    // is where a `match` renderer reads it.
    let l = types.kuna_variant_layout("Option<u32>").unwrap();
    let none = l.facet_named("None").unwrap();
    assert_eq!(none.discr, Some(0));
    assert!(none.fields.is_empty());
    assert!(none.payload_type.is_empty(), "no union member was minted for it");
    assert_eq!(l.facet_named("Some").unwrap().fields, vec![("__0".to_string(), 4)]);
    assert_eq!(l.facet_named("Some").unwrap().payload_type, "Option<u32>::Some");
}

/// A niche-encoded `Option<&u32>`: the discriminant IS the payload's first word,
/// so there is no byte range that is only the tag. The recovered struct then has
/// only the overlay, and `niche` on the side table is what says so.
#[test]
fn builds_niche_option_without_a_tag_field() {
    let types = factory();
    let mut t = Tree::new();
    let u64t = t.uint(8);
    let refu = t.ptr(u64t);
    let e = t.strukt(None, "Option<&u32>", 8);
    let vp = t.variant_part(e, u64t, 0);
    let none = t.strukt(Some(e), "None", 8);
    let some = t.strukt(Some(e), "Some", 8);
    t.member(some, "__0", refu, 0);
    t.variant(vp, Some(0), "None", none);
    t.variant(vp, None, "Some", some);

    let built = intern(&types, &t, e).expect("niche Option must build");
    assert_eq!(
        built.get_metatype(),
        type_metatype::TYPE_UNION,
        "the overlay IS the type when no byte range is only the tag"
    );
    assert_eq!(built.get_size(), 8);
    assert_eq!(
        fields(&built),
        vec![("Some".to_string(), 0, 8)],
        "the fieldless `None` overlays nothing"
    );
    let some = built.get_field(0).unwrap().field_type.clone();
    assert_eq!(fields(&some), vec![("__0".to_string(), 0, 8)]);

    let l = types.kuna_variant_layout("Option<&u32>").unwrap();
    assert!(l.niche, "the overlap must be recorded");
    assert_eq!((l.tag_offset, l.tag_size), (0, 8));
    assert_eq!(l.payload_offset, 0);
    assert_eq!(l.union_type, "Option<&u32>", "the union is the whole type");
    assert_eq!(l.facet_for_discr(0).unwrap().name, "None");
    assert_eq!(
        l.facet_for_discr(0x5555_5555).unwrap().name,
        "Some",
        "every unclaimed value is the default variant"
    );
}

/// Three variants, and one of them fieldless: nothing here is capped at two.
#[test]
fn builds_three_variants() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let e = t.strukt(None, "Three", 16);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 16);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 16);
    t.member(b, "__0", u64t, 8);
    let c = t.strukt(Some(e), "C", 16);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    t.variant(vp, Some(2), "C", c);

    let built = intern(&types, &t, e).expect("Three must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(
        fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["A", "B"],
        "the fieldless `C` overlays nothing; it is on the side table instead"
    );
    let l = types.kuna_variant_layout("Three").unwrap();
    assert_eq!(l.variants.len(), 3);
    assert_eq!(l.facet_for_discr(2).unwrap().name, "C");
    assert!(l.facet_named("C").unwrap().payload_type.is_empty());
    // The overlay starts at 4 (A's field), so B's u64 at enum offset 8 is at 4
    // inside the facet.
    assert_eq!(l.payload_offset, 4);
    let bt = payload.get_field(1).unwrap().field_type.clone();
    assert_eq!(fields(&bt), vec![("__0".to_string(), 4, 8)]);
}

/// A variant with MULTIPLE fields keeps both, at their re-based offsets — and
/// the naming rule is applied PER FIELD, not per variant: `P.a` at [4,8) is the
/// only thing any variant puts there, so it keeps its name, while `P.b` and
/// `Q.__0` collide over offset 8 under different spellings and both become the
/// offset form.
#[test]
fn builds_multi_field_variant() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let e = t.strukt(None, "Multi", 16);
    let vp = t.variant_part(e, u32t, 0);
    let p = t.strukt(Some(e), "P", 16);
    t.member(p, "a", u32t, 4);
    t.member(p, "b", u32t, 8);
    let q = t.strukt(Some(e), "Q", 16);
    t.member(q, "__0", u64t, 8);
    t.variant(vp, Some(0), "P", p);
    t.variant(vp, Some(1), "Q", q);

    let built = intern(&types, &t, e).expect("Multi must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(
        fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["field_0x4", "field_0x8"],
        "P and Q share offset 8, so neither is named"
    );
    let pt = payload.get_field(0).unwrap().field_type.clone();
    assert_eq!(
        fields(&pt),
        vec![("a".to_string(), 0, 4), ("field_0x8".to_string(), 4, 4)],
        "`a` is P's alone; `b` collides with Q's `__0`"
    );
    let l = types.kuna_variant_layout("Multi").unwrap();
    // The side table keeps the SOURCE names and offsets regardless of the label.
    assert_eq!(
        l.facet_named("P").unwrap().fields,
        vec![("a".to_string(), 4), ("b".to_string(), 8)]
    );
    assert_eq!(l.facet_named("P").unwrap().label, "field_0x4");
    assert_eq!(l.facet_named("Q").unwrap().label, "field_0x8");
    assert!(!l.names_any_variant());
}

/// `enum List { Cons(u32, *const List), Nil }` -- a payload that points back at
/// the enclosing enum. The assertion is that the call RETURNS and the pointer
/// still names the type; a hang or a stack overflow is the failure.
#[test]
fn recursive_enum_terminates() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "List", 16);
    let selfptr = t.ptr(e);
    let vp = t.variant_part(e, u32t, 0);
    let cons = t.strukt(Some(e), "Cons", 16);
    t.member(cons, "__0", u32t, 4);
    t.member(cons, "__1", selfptr, 8);
    let nil = t.strukt(Some(e), "Nil", 16);
    t.variant(vp, Some(0), "Cons", cons);
    t.variant(vp, Some(1), "Nil", nil);

    let built = intern(&types, &t, e).expect("List must build");
    assert_eq!(built.get_size(), 16);
    let payload = built.get_field(1).unwrap().field_type.clone();
    let consty = payload.get_field(0).unwrap().field_type.clone();
    let f = fields(&consty);
    assert_eq!(f.len(), 2, "{f:?}");
    assert_eq!(f[1].0, "__1");
    let next = consty.get_field(1).unwrap().field_type.clone();
    assert_eq!(next.get_metatype(), type_metatype::TYPE_PTR);
    assert_eq!(
        next.get_depend(0).map(|d| d.get_name().to_string()),
        Some("List".to_string()),
        "the recursive pointer still names the enum"
    );
}

/// Two enums whose variants are SAME-NAMED and DIFFERENT-SIZED. Interning the
/// second under the first's facet name would make the factory refuse the
/// redefinition ("Trying to alter definition of type") and silently degrade the
/// whole aggregate; parent qualification plus the size-suffix step keeps both.
#[test]
fn same_named_different_sized_variants_both_survive() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);

    // A: 8 bytes, Some payload 4 bytes.
    let ea = t.strukt(None, "Opt", 8);
    let vpa = t.variant_part(ea, u32t, 0);
    let na = t.strukt(Some(ea), "None", 8);
    let sa = t.strukt(Some(ea), "Some", 8);
    t.member(sa, "__0", u32t, 4);
    t.variant(vpa, Some(0), "None", na);
    t.variant(vpa, Some(1), "Some", sa);

    // B: the SAME enum name, 16 bytes, Some payload 8 bytes.
    let eb = t.strukt(None, "Opt", 16);
    let vpb = t.variant_part(eb, u32t, 0);
    let nb = t.strukt(Some(eb), "None", 16);
    let sb = t.strukt(Some(eb), "Some", 16);
    t.member(sb, "__0", u64t, 8);
    t.variant(vpb, Some(0), "None", nb);
    t.variant(vpb, Some(1), "Some", sb);

    let a = intern(&types, &t, ea).expect("first Opt must build");
    let b = intern(&types, &t, eb).expect("second Opt must build");
    assert_eq!(a.get_name(), "Opt");
    assert_eq!(b.get_name(), "Opt_16", "the second steps to a size-suffixed name");
    assert_eq!(a.get_size(), 8);
    assert_eq!(b.get_size(), 16);

    let pa = a.get_field(1).unwrap().field_type.clone();
    let pb = b.get_field(1).unwrap().field_type.clone();
    assert_eq!(pa.get_field(0).unwrap().name, "Some");
    assert_eq!(pb.get_field(0).unwrap().name, "Some");
    assert_eq!(pa.get_field(0).unwrap().field_type.get_size(), 4);
    assert_eq!(pb.get_field(0).unwrap().field_type.get_size(), 8);
    assert_ne!(
        pa.get_field(0).unwrap().field_type.get_name(),
        pb.get_field(0).unwrap().field_type.get_name(),
        "the two `Some` facets must not share one interned type"
    );

    let la = types.kuna_variant_layout("Opt").unwrap();
    let lb = types.kuna_variant_layout("Opt_16").unwrap();
    assert_eq!(la.facet_named("Some").unwrap().fields, vec![("__0".to_string(), 4)]);
    assert_eq!(lb.facet_named("Some").unwrap().fields, vec![("__0".to_string(), 8)]);
}

/// A variant part where NO variant has any field is a C-like enum wearing the
/// wrong tag; there is no overlay to synthesize, so it is refused and left to
/// the ordinary aggregate path.
#[test]
fn refuses_variant_part_with_no_payload_anywhere() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "AllUnit", 4);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 4);
    let b = t.strukt(Some(e), "B", 4);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    assert!(intern(&types, &t, e).is_none());
    assert!(types.kuna_variant_layout("AllUnit").is_none());
}

/// A discriminant whose type is a POINTER is not a discriminant this module will
/// match `DW_AT_discr_value` against.
#[test]
fn refuses_non_integer_discriminant() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let p = t.ptr(u32t);
    let e = t.strukt(None, "PtrTag", 16);
    let vp = t.variant_part(e, p, 0);
    let a = t.strukt(Some(e), "A", 16);
    t.member(a, "__0", u32t, 8);
    let b = t.strukt(Some(e), "B", 16);
    t.member(b, "__0", u32t, 8);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    assert!(intern(&types, &t, e).is_none());
}

/// A zero-width or absent `DW_AT_byte_size` gives nothing to lay the overlay out
/// in.
#[test]
fn refuses_sizeless_enum() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    t.dies.get_mut(&e).unwrap().byte_size = None;
    assert!(intern(&types, &t, e).is_none());
    t.dies.get_mut(&e).unwrap().byte_size = Some(0);
    assert!(intern(&types, &t, e).is_none());
}

/// A `DW_AT_declaration` forward reference carries no layout.
#[test]
fn refuses_declaration_only() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    t.dies.get_mut(&e).unwrap().declaration = true;
    assert!(intern(&types, &t, e).is_none());
}

/// Every facet's fields were skipped (here: each variant's only member has a
/// `void` type), so there is no overlay left and a zero-member union would
/// describe nothing.
#[test]
fn refuses_when_every_facet_field_is_unbuildable() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "AllVoid", 8);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 8);
    // A member with no `DW_AT_type` is `void`, which is skipped as a field.
    let mut m = DieSnap::new(gimli::DW_TAG_member, 1);
    m.name = "__0".into();
    m.data_member_location = Some(4);
    t.add(Some(a), m);
    let b = t.strukt(Some(e), "B", 8);
    let mut m2 = DieSnap::new(gimli::DW_TAG_member, 1);
    m2.name = "__0".into();
    m2.data_member_location = Some(4);
    t.add(Some(b), m2);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    // The read still succeeds -- the fields are there in DWARF -- and the BUILD
    // is what refuses. The refusal comes AFTER the shell was interned, so what
    // comes back is the shell SEALED at the enum's own width with no fields (the
    // `dwarfstructs` answer), never a zero-size incomplete type.
    assert!(read_variant_part(&t.dies[&e], &t.dies).is_some());
    let built = intern(&types, &t, e).expect("the sealed shell is the answer");
    assert_eq!(built.get_size(), 8, "sealed at DW_AT_byte_size, not 0");
    assert!(fields(&built).is_empty());
    assert!(!built.is_incomplete());
    assert!(types.kuna_variant_layout("AllVoid").is_none(), "no layout is claimed");
}

/// The same seal for a member that would run PAST the enum's byte size: the
/// layout is refused, the name still carries the width the ABI needs.
#[test]
fn a_member_past_the_end_seals_instead_of_leaving_a_zero_size_shell() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let e = t.strukt(None, "Overrun2", 8);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 8);
    t.member(a, "__0", u64t, 4); // 4 + 8 > 8
    let b = t.strukt(Some(e), "B", 8);
    t.member(b, "__0", u32t, 4);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    let built = intern(&types, &t, e).expect("the sealed shell is the answer");
    assert_eq!(built.get_size(), 8);
    assert!(fields(&built).is_empty());
    assert!(types.kuna_variant_layout("Overrun2").is_none());
}

/// FIVE variants, three of them carrying payloads: nothing in the walk is capped
/// at two or three. `A` owns [4,8) alone and keeps its name; `B` (u64 at 8) and
/// `C` (u32 at 8) collide, so both are spelled by offset — and because their
/// layouts DIFFER they must not be merged into one interned facet, or the ABI
/// classifier's common refinement of the union would lose a shape.
#[test]
fn builds_five_variants() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let e = t.strukt(None, "Five", 16);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 16);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 16);
    t.member(b, "__0", u64t, 8);
    let c = t.strukt(Some(e), "C", 16);
    t.member(c, "__0", u32t, 8);
    let d = t.strukt(Some(e), "D", 16);
    let f = t.strukt(Some(e), "F", 16);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);
    t.variant(vp, Some(2), "C", c);
    t.variant(vp, Some(3), "D", d);
    t.variant(vp, Some(4), "F", f);

    let built = intern(&types, &t, e).expect("Five must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(
        fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["A", "field_0x8", "field_0x8"],
        "the two fieldless variants overlay nothing; B and C collide at 8"
    );
    // Different shapes at the same offset get DISTINCT interned facets.
    let bt = payload.get_field(1).unwrap().field_type.clone();
    let ct = payload.get_field(2).unwrap().field_type.clone();
    assert_eq!(bt.get_name(), "Five::field_0x8");
    assert_eq!(ct.get_name(), "Five::field_0x8_1");
    // Both are called `__0`, but over DIFFERENT extents ([8,16) vs [8,12)), so
    // the spellings do not agree and the offset form wins in both.
    assert_eq!(fields(&bt), vec![("field_0x8".to_string(), 4, 8)]);
    assert_eq!(fields(&ct), vec![("field_0x8".to_string(), 4, 4)]);

    let l = types.kuna_variant_layout("Five").unwrap();
    assert_eq!(l.variants.len(), 5, "all five reach the side table");
    for (v, n) in [(0u64, "A"), (1, "B"), (2, "C"), (3, "D"), (4, "F")] {
        assert_eq!(l.facet_for_discr(v).unwrap().name, n);
    }
    assert!(l.facet_for_discr(5).is_none(), "no default variant here");
    assert!(l.facet_named("D").unwrap().payload_type.is_empty());
    assert_eq!(l.facet_named("A").unwrap().label, "A");
    assert!(l.names_any_variant(), "`A` owns its bytes outright");
}

/// One GENERIC enum instantiated at two different types. rustc gives the two
/// instantiations distinct enum names but names both payload structs the bare
/// `Some`, so the facet names are the collision risk -- and parent qualification
/// is what keeps them apart.
#[test]
fn a_generic_enum_at_two_instantiations_keeps_both() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);

    let small = t.strukt(None, "Option<u32>", 8);
    let vps = t.variant_part(small, u32t, 0);
    let ns = t.strukt(Some(small), "None", 8);
    let ss = t.strukt(Some(small), "Some", 8);
    t.member(ss, "__0", u32t, 4);
    t.variant(vps, Some(0), "None", ns);
    t.variant(vps, Some(1), "Some", ss);

    let big = t.strukt(None, "Option<u64>", 16);
    let vpb = t.variant_part(big, u32t, 0);
    let nb = t.strukt(Some(big), "None", 16);
    let sb = t.strukt(Some(big), "Some", 16);
    t.member(sb, "__0", u64t, 8);
    t.variant(vpb, Some(0), "None", nb);
    t.variant(vpb, Some(1), "Some", sb);

    let a = intern(&types, &t, small).expect("Option<u32> must build");
    let b = intern(&types, &t, big).expect("Option<u64> must build");
    assert_eq!((a.get_name(), a.get_size()), ("Option<u32>", 8));
    assert_eq!((b.get_name(), b.get_size()), ("Option<u64>", 16));

    let fa = a.get_field(1).unwrap().field_type.get_field(0).unwrap().field_type.clone();
    let fb = b.get_field(1).unwrap().field_type.get_field(0).unwrap().field_type.clone();
    assert_eq!(fa.get_name(), "Option<u32>::Some");
    assert_eq!(fb.get_name(), "Option<u64>::Some");
    assert_eq!(fields(&fa), vec![("__0".to_string(), 0, 4)]);
    assert_eq!(fields(&fb), vec![("__0".to_string(), 0, 8)]);
    assert_eq!(types.kuna_variant_layouts().len(), 2);
}

/// A niche `Option<Box<T>>` is the same shape as `Option<&T>` -- the null pointer
/// IS the discriminant -- and must recover as the overlay alone, with `Some` the
/// DEFAULT variant that every non-null value selects.
#[test]
fn builds_niche_option_box() {
    let types = factory();
    let mut t = Tree::new();
    let u64t = t.uint(8);
    let boxed = t.ptr(u64t);
    let e = t.strukt(None, "Option<Box<u64>>", 8);
    let vp = t.variant_part(e, u64t, 0);
    let none = t.strukt(Some(e), "None", 8);
    let some = t.strukt(Some(e), "Some", 8);
    t.member(some, "__0", boxed, 0);
    t.variant(vp, Some(0), "None", none);
    t.variant(vp, None, "Some", some);

    let built = intern(&types, &t, e).expect("niche Option<Box> must build");
    assert_eq!(built.get_metatype(), type_metatype::TYPE_UNION);
    assert_eq!(fields(&built), vec![("Some".to_string(), 0, 8)]);
    let l = types.kuna_variant_layout("Option<Box<u64>>").unwrap();
    assert!(l.niche);
    assert_eq!(l.facet_named("Some").unwrap().discr, None, "Some is the default variant");
    assert_eq!(l.facet_for_discr(0).unwrap().name, "None");
    assert_eq!(l.facet_for_discr(0x4000).unwrap().name, "Some");
}

/// A C `union` carries `DW_TAG_member` children and NO `DW_TAG_variant_part`, so
/// it never reaches this arm's layout at all. (`mod.rs` also refuses to enter the
/// arm for a union DIE, because the tagged encoding is a struct shape; this is
/// the belt to that braces.)
#[test]
fn refuses_a_c_union() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let u = t.union(None, "value", 8);
    t.member(u, "i", u32t, 0);
    t.member(u, "l", u64t, 0);
    assert!(variant_part(&t.dies[&u], &t.dies).is_none());
    assert!(read_variant_part(&t.dies[&u], &t.dies).is_none());
    assert!(intern(&types, &t, u).is_none());
    assert!(types.kuna_variant_layouts().is_empty());
}

/// A C++ `std::variant` is a tagged representation too, but libstdc++ spells it
/// as ordinary members (`_M_u`, `_M_index`) -- there is no `DW_AT_discr` saying
/// which one is the tag, so this arm must not claim it. `dwarfstructs` keeps
/// giving it its real members.
#[test]
fn refuses_a_cpp_std_variant() {
    let types = factory();
    let mut t = Tree::new();
    let u8t = t.uint(1);
    let u64t = t.uint(8);
    let v = t.strukt(None, "std::variant<int, long>", 16);
    t.member(v, "_M_u", u64t, 0);
    t.member(v, "_M_index", u8t, 8);
    assert!(variant_part(&t.dies[&v], &t.dies).is_none());
    assert!(intern(&types, &t, v).is_none());
    assert!(types.kuna_variant_layouts().is_empty());
}

/// A plain `gcc -g` struct -- members, no variant part -- is refused at the very
/// first step and left entirely to `dwarfstructs`. Asserted through
/// `intern_variant_aggregate` (not only `variant_part`) because that is the
/// entry point `mod.rs` actually calls.
#[test]
fn refuses_a_plain_gcc_struct() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let s = t.strukt(None, "point", 8);
    t.member(s, "x", u32t, 0);
    t.member(s, "y", u32t, 4);
    assert!(intern(&types, &t, s).is_none());
    assert!(types.find_by_name("point").unwrap().is_none(), "nothing was interned");
    assert!(types.kuna_variant_layouts().is_empty());
}

/// The last post-intern refusal: every candidate name for the overlay union is
/// held by something that is not a union, so the payload union cannot be minted.
/// The shell is already in the factory under the enum's own name, so the answer
/// is the SEALED shell -- the enum's `DW_AT_byte_size` with no fields, exactly
/// what `dwarfstructs` gives -- and never a zero-size type that would degrade to
/// `void` downstream.
#[test]
fn seals_when_the_overlay_union_name_is_exhausted() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    // `resolve_name` tries base, base_<size>, base_<size>_2, base_<size>_3
    // (MAX_NAME_ATTEMPTS = 4); the payload overlay is 4 bytes wide.
    for n in [
        "Result<u32, u32>::payload",
        "Result<u32, u32>::payload_4",
        "Result<u32, u32>::payload_4_2",
        "Result<u32, u32>::payload_4_3",
    ] {
        let s = types.get_type_struct(n).expect("blocker interns");
        types
            .set_fields_struct_raw(&s, Vec::new(), Vec::new(), 4, 1, 0)
            .expect("blocker completes");
    }

    let built = intern(&types, &t, e).expect("the sealed shell is the answer");
    assert_eq!(built.get_name(), "Result<u32, u32>");
    assert_eq!(built.get_size(), 8, "sealed at DW_AT_byte_size, not 0");
    assert!(!built.is_incomplete(), "a zero-size incomplete type is the defect");
    assert!(fields(&built).is_empty());
    assert!(
        types.kuna_variant_layout("Result<u32, u32>").is_none(),
        "no layout may be claimed for a shape that was not installed"
    );
}

/// The gate is the module's own env var, and it is what `mod.rs` consults before
/// entering this arm at all.
#[test]
fn gate_defaults_on_and_honours_off() {
    std::env::remove_var(kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV);
    assert!(enabled());
    kuna_decomp::kuna_dwarfvariants::set_dwarfvariants_env(false);
    assert!(!enabled());
    kuna_decomp::kuna_dwarfvariants::set_dwarfvariants_env(true);
    assert!(enabled());
    std::env::remove_var(kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV);
}

// ---------------------------------------------------------------------------
// The naming rule -- both halves, and both answers
// ---------------------------------------------------------------------------

/// The KEPT half, stated on its own: `Option<T>` has exactly ONE
/// payload-carrying variant, so no other variant claims the payload's bytes and
/// `Some` is forced. This is the case the suppression must not take away.
#[test]
fn option_keeps_its_only_payload_variant_name() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let e = t.strukt(None, "Option<u32>", 8);
    let vp = t.variant_part(e, u32t, 0);
    let none = t.strukt(Some(e), "None", 8);
    let some = t.strukt(Some(e), "Some", 8);
    t.member(some, "__0", u32t, 4);
    t.variant(vp, Some(0), "None", none);
    t.variant(vp, Some(1), "Some", some);

    let built = intern(&types, &t, e).expect("Option must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), vec!["Some"]);
    let somet = payload.get_field(0).unwrap().field_type.clone();
    assert_eq!(somet.get_name(), "Option<u32>::Some", "the facet TYPE keeps it too");
    let l = types.kuna_variant_layout("Option<u32>").unwrap();
    assert_eq!(l.facet_named("Some").unwrap().label, "Some");
    assert!(l.names_any_variant());
}

/// A niche `enum Tree { Leaf(i64), Node(Box<Tree>, Box<Tree>) }` — the shape the
/// PR that introduced this module documented as a corner case and this test
/// pins as the rule. `Leaf.__0` and `Node.__0` are both at offset 8, so neither
/// facet is named; `Node.__1` at offset 16 is claimed by nobody else, so IT
/// keeps its name. No path into the type spells `Leaf` or `Node`.
#[test]
fn tree_style_overlay_names_no_variant_but_keeps_the_uncontested_field() {
    let types = factory();
    let mut t = Tree::new();
    let u64t = t.uint(8);
    let e = t.strukt(None, "Tree", 24);
    let boxed = t.ptr(e);
    let vp = t.variant_part(e, u64t, 0);
    let leaf = t.strukt(Some(e), "Leaf", 24);
    t.member(leaf, "__0", u64t, 8);
    let node = t.strukt(Some(e), "Node", 24);
    t.member(node, "__0", boxed, 8);
    t.member(node, "__1", boxed, 16);
    t.variant(vp, Some(0), "Leaf", leaf);
    t.variant(vp, Some(1), "Node", node);

    let built = intern(&types, &t, e).expect("Tree must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(
        fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["field_0x8", "field_0x8"]
    );
    let leaft = payload.get_field(0).unwrap().field_type.clone();
    let nodet = payload.get_field(1).unwrap().field_type.clone();
    assert_eq!(leaft.get_name(), "Tree::field_0x8");
    assert_eq!(nodet.get_name(), "Tree::field_0x8_1", "different shapes stay apart");
    assert_eq!(fields(&leaft), vec![("__0".to_string(), 0, 8)]);
    assert_eq!(
        fields(&nodet),
        vec![("__0".to_string(), 0, 8), ("__1".to_string(), 8, 8)],
        "`__1` at offset 16 is Node's alone, so it keeps its name"
    );
    for n in ["Tree::Leaf", "Tree::Node"] {
        assert!(types.find_by_name(n).unwrap().is_none(), "{n} must not exist");
    }
    let l = types.kuna_variant_layout("Tree").unwrap();
    assert!(!l.names_any_variant());
    assert_eq!(l.facet_for_discr(1).unwrap().name, "Node", "the source name survives here");
}

/// The DEFAULT (niche) variant is not a special case for the rule: with two
/// payload-carrying variants overlaying one another, the one DWARF gave no
/// `DW_AT_discr_value` is suppressed exactly like the other.
#[test]
fn default_variant_is_suppressed_when_it_overlays_another() {
    let types = factory();
    let mut t = Tree::new();
    let u64t = t.uint(8);
    let ptr = t.ptr(u64t);
    let e = t.strukt(None, "Either", 16);
    let vp = t.variant_part(e, u64t, 0);
    let a = t.strukt(Some(e), "Ptr", 16);
    t.member(a, "__0", ptr, 8);
    let b = t.strukt(Some(e), "Num", 16);
    t.member(b, "__0", u64t, 8);
    t.variant(vp, Some(0), "Num", b);
    t.variant(vp, None, "Ptr", a);

    let built = intern(&types, &t, e).expect("Either must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert!(
        fields(&payload).iter().all(|f| f.0 == "field_0x8"),
        "the default variant is named no more freely than the tagged one: {:?}",
        fields(&payload)
    );
    let l = types.kuna_variant_layout("Either").unwrap();
    assert_eq!(l.facet_for_discr(0xdead).unwrap().name, "Ptr", "still the default on the table");
    assert!(!l.names_any_variant());
}

/// Two payload variants that do NOT overlap keep both names, and the emitted
/// path is then forced by the offset: only `A` has a field in [4,8) and only `B`
/// has one in [8,16), so neither label can be the wrong one.
#[test]
fn disjoint_payload_variants_keep_both_names() {
    let types = factory();
    let mut t = Tree::new();
    let u32t = t.uint(4);
    let u64t = t.uint(8);
    let e = t.strukt(None, "Split", 16);
    let vp = t.variant_part(e, u32t, 0);
    let a = t.strukt(Some(e), "A", 16);
    t.member(a, "__0", u32t, 4);
    let b = t.strukt(Some(e), "B", 16);
    t.member(b, "__0", u64t, 8);
    t.variant(vp, Some(0), "A", a);
    t.variant(vp, Some(1), "B", b);

    let built = intern(&types, &t, e).expect("Split must build");
    let payload = built.get_field(1).unwrap().field_type.clone();
    assert_eq!(
        fields(&payload).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        vec!["A", "B"]
    );
    let l = types.kuna_variant_layout("Split").unwrap();
    assert_eq!(l.facet_named("A").unwrap().label, "A");
    assert_eq!(l.facet_named("B").unwrap().label, "B");
    assert!(l.names_any_variant());
}

/// A field two variants place at the SAME extent under the SAME spelling keeps
/// that spelling — rustc names every tuple payload `__0`, so `Result`'s two
/// `__0`s agree and the name claims nothing about which variant is live. The
/// facet around it is still suppressed.
#[test]
fn agreeing_field_names_survive_even_when_the_facet_does_not() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    intern(&types, &t, e).expect("Result must build");
    let facet = types
        .find_by_name("Result<u32, u32>::field_0x4")
        .unwrap()
        .expect("the suppressed facet is interned under the offset name");
    assert_eq!(fields(&facet), vec![("__0".to_string(), 0, 4)]);
}

/// The suppressed-facet name is derived from an OFFSET, so two variants of one
/// enum can want the same one and `open_facet` steps over a same-named complete
/// type whose layout differs. When every candidate it will try is taken by a
/// different shape it gives up — and because the enum's shell is already interned
/// by then, the answer must be the SEALED shell (byte size, no fields), never a
/// zero-size type that degrades to `void`.
#[test]
fn seals_when_every_suppressed_facet_name_is_taken() {
    let types = factory();
    let mut t = Tree::new();
    let e = result_u32(&mut t);
    // The facet wants `Result<u32, u32>::field_0x4` (payload width 4) and will
    // try `_1` .. `_7` after it. Hold every one with a COMPLETE 4-byte struct of
    // a different shape, so `same_layout` rejects each.
    for n in 0..8 {
        let name = if n == 0 {
            "Result<u32, u32>::field_0x4".to_string()
        } else {
            format!("Result<u32, u32>::field_0x4_{n}")
        };
        let s = types.get_type_struct(&name).expect("blocker interns");
        let u32t = types.get_base(4, type_metatype::TYPE_UINT).expect("u32");
        types
            .set_fields_struct_raw(
                &s,
                vec![TypeField::new(0, 0, "zz".to_string(), u32t)],
                Vec::new(),
                4,
                4,
                0,
            )
            .expect("blocker completes");
    }

    let built = intern(&types, &t, e).expect("the sealed shell is the answer");
    assert_eq!(built.get_name(), "Result<u32, u32>");
    assert_eq!(built.get_size(), 8, "sealed at DW_AT_byte_size, not 0");
    assert!(!built.is_incomplete(), "a zero-size incomplete type is the defect");
    assert!(fields(&built).is_empty());
    assert!(
        types.kuna_variant_layout("Result<u32, u32>").is_none(),
        "no layout may be claimed for a shape that was not installed"
    );
}
