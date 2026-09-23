//! Unit tests for the `structmerge` option surface and the union rule.

use super::*;
use crate::p5_types::kuna_structsynth::ledger;

const ON: StructMergeMode = StructMergeMode::Siblings;
const OFF: StructMergeMode = StructMergeMode::Off;

#[test]
fn option_parses_both_values_and_rejects_anything_else() {
    assert_eq!(OptionStructMerge.apply("off").unwrap().0, StructMergeMode::Off);
    assert_eq!(OptionStructMerge.apply("siblings").unwrap().0, StructMergeMode::Siblings);
    assert!(OptionStructMerge.apply("on").is_err());
    assert!(OptionStructMerge.apply("all").is_err());
    assert!(!StructMergeMode::Off.fires() && StructMergeMode::Siblings.fires());
    assert_eq!(StructMergeMode::default(), StructMergeMode::Off);
}

/// A factory with the core types a real architecture mints.
fn core_factory() -> crate::dtype::TypeFactoryImpl {
    let f = crate::dtype::TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    for (name, size, meta, chartp) in [
        ("undefined", 1, type_metatype::TYPE_UNKNOWN, false),
        ("undefined8", 8, type_metatype::TYPE_UNKNOWN, false),
        ("char", 1, type_metatype::TYPE_INT, true),
        ("int4", 4, type_metatype::TYPE_INT, false),
        ("uint4", 4, type_metatype::TYPE_UINT, false),
        ("int8", 8, type_metatype::TYPE_INT, false),
    ] {
        f.set_core_type(name, size, meta, chartp).unwrap();
    }
    f.cache_core_types().unwrap();
    f
}

/// A pointer to `char`, the one field type that carries a pointee.
fn charptr(f: &crate::dtype::TypeFactoryImpl) -> Rc<Datatype> {
    f.get_type_pointer(8, f.get_base(1, type_metatype::TYPE_INT).unwrap(), 1).unwrap()
}

/// The claimed offsets of a completed structure, filler dropped.
fn claimed(ct: &Datatype) -> Vec<int4> {
    ledger::layout_of(ct).unwrap().fields.iter().map(|k| k.offset).collect()
}

/// `undefined1[n]` filler at `off`, the member a hole becomes.
fn filler(f: &crate::dtype::TypeFactoryImpl, ident: int4, off: int4, n: int4) -> TypeField {
    let byte = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    TypeField::new(ident, off, format!("field_0x{off:x}"), f.get_type_array(n, byte).unwrap())
}

/// Two readers of one record that measured overlapping-but-different parts of
/// it: `{0: char *, 8: long, 0x18: long}` and `{0: char *, 8: long, 0x10: int}`.
/// Neither contains the other, so the ledger alone gives each a name carrying
/// three fields; the union carries all four and supersedes the first.
#[test]
fn two_readers_of_one_record_are_answered_by_the_union_of_their_claims() {
    let build = |f: &crate::dtype::TypeFactoryImpl, tail: (int4, bool)| {
        let (int4t, int8t, cp) = (
            f.get_base(4, type_metatype::TYPE_INT).unwrap(),
            f.get_base(8, type_metatype::TYPE_INT).unwrap(),
            charptr(f),
        );
        let (off, wide) = tail;
        let mut out = vec![
            TypeField::new(0, 0, "field_0x0", cp),
            TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        ];
        if off > 0x10 {
            out.push(filler(f, 2, 0x10, off - 0x10));
        }
        out.push(TypeField::new(
            3,
            off,
            format!("field_0x{off:x}"),
            if wide { int8t } else { Rc::clone(&int4t) },
        ));
        if !wide {
            out.push(filler(f, 4, off + 4, 4));
        }
        out
    };

    let f = core_factory();
    let a = ledger::lookup_or_mint(&f, build(&f, (0x18, true)), 0x20, &[], &[], OFF).unwrap();
    let b = ledger::lookup_or_mint(&f, build(&f, (0x10, false)), 0x18, &[], &[], OFF).unwrap();
    assert_eq!(claimed(&a), vec![0, 8, 0x18]);
    assert_eq!(claimed(&b), vec![0, 8, 0x10], "off, each reader keeps only its own claims");
    assert_ne!(a.get_name(), b.get_name());

    let g = core_factory();
    let ga = ledger::lookup_or_mint(&g, build(&g, (0x18, true)), 0x20, &[], &[], ON).unwrap();
    let union = ledger::lookup_or_mint(&g, build(&g, (0x10, false)), 0x18, &[], &[], ON).unwrap();
    assert_eq!(claimed(&union), vec![0, 8, 0x10, 0x18], "the union carries both readers' claims");
    assert_eq!(union.get_size(), 0x20);
    assert_ne!(union.get_name(), ga.get_name(), "a held structure is never widened in place");
    assert_eq!(
        ledger::superseded_names(&g),
        vec![ga.get_name().to_string()],
        "the first reader's record is superseded, so the sweep moves it onto the union"
    );
}

/// A field the two readers disagree about is two different records' fields, and
/// a claim of one over bytes the other claims at a different offset is the same
/// answer. Neither merges.
#[test]
fn a_contradicted_claim_refuses_the_union() {
    for (off, width) in [(0x10, 8), (0x14, 4)] {
        let f = core_factory();
        let (int4t, int8t, cp) = (
            f.get_base(4, type_metatype::TYPE_INT).unwrap(),
            f.get_base(8, type_metatype::TYPE_INT).unwrap(),
            charptr(&f),
        );
        // The held record: a four-byte field at 0x10, then nothing.
        let held = vec![
            TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
            TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
            TypeField::new(2, 0x10, "field_0x10", Rc::clone(&int4t)),
            filler(&f, 3, 0x14, 4),
        ];
        let a = ledger::lookup_or_mint(&f, held, 0x18, &[], &[], ON).unwrap();
        assert_eq!(claimed(&a), vec![0, 8, 0x10]);
        // The second reader: eight bytes at 0x10 (the same offset, a different
        // field), or four bytes at 0x14 (inside the bytes the held field covers
        // once it is eight wide) -- either way the two contradict each other.
        let mut second = vec![
            TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
            TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        ];
        if off > 0x10 {
            second.push(TypeField::new(2, 0x10, "field_0x10", Rc::clone(&int8t)));
        }
        second.push(TypeField::new(
            3,
            off,
            format!("field_0x{off:x}"),
            if width == 8 { Rc::clone(&int8t) } else { Rc::clone(&int4t) },
        ));
        if width == 4 {
            second.push(filler(&f, 4, off + 4, 4));
        }
        let b = ledger::lookup_or_mint(&f, second, off + 8, &[], &[], ON).unwrap();
        assert_ne!(b.get_name(), a.get_name());
        assert!(!claimed(&b).contains(&0x14) || off == 0x14, "each keeps what it measured");
        assert!(claimed(&b).len() <= 4);
    }
}

/// One shared claim is what unrelated records have: a `char *` at offset 0 is
/// the first word of half the records in a program. The union is only minted
/// where the agreement is itself evidence.
#[test]
fn a_single_shared_claim_is_not_evidence_of_one_record() {
    let f = core_factory();
    let (int4t, int8t, cp) =
        (f.get_base(4, type_metatype::TYPE_INT).unwrap(),
         f.get_base(8, type_metatype::TYPE_INT).unwrap(),
         charptr(&f));
    let first = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        filler(&f, 1, 8, 8),
        TypeField::new(2, 0x10, "field_0x10", Rc::clone(&int4t)),
        filler(&f, 3, 0x14, 4),
    ];
    let a = ledger::lookup_or_mint(&f, first, 0x18, &[], &[], ON).unwrap();
    let second = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        filler(&f, 2, 0x10, 8),
        TypeField::new(3, 0x18, "field_0x18", Rc::clone(&int8t)),
    ];
    let b = ledger::lookup_or_mint(&f, second, 0x20, &[], &[], ON).unwrap();
    assert_ne!(b.get_name(), a.get_name());
    assert_eq!(claimed(&b), vec![0, 8, 0x18], "only offset 0 agreed, so nothing is merged");
}

/// A reader is never declared to hold more than twice the fields it measured:
/// the union is held to the containment rule's own growth bounds against each
/// side separately.
#[test]
fn the_union_may_not_outgrow_either_reader() {
    let f = core_factory();
    let (int8t, cp) =
        (f.get_base(8, type_metatype::TYPE_INT).unwrap(), charptr(&f));
    let mut wide = Vec::new();
    for i in 0..8 {
        let off = i * 8;
        let ty = if i == 0 { Rc::clone(&cp) } else { Rc::clone(&int8t) };
        wide.push(TypeField::new(i, off, format!("field_0x{off:x}"), ty));
    }
    let a = ledger::lookup_or_mint(&f, wide, 0x40, &[], &[], ON).unwrap();
    assert_eq!(claimed(&a).len(), 8);
    // Two claims of its own plus one more: the union would be nine fields, more
    // than twice what this reader measured.
    let small = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        filler(&f, 2, 0x10, 0x30),
        TypeField::new(3, 0x40, "field_0x40", Rc::clone(&int8t)),
    ];
    let b = ledger::lookup_or_mint(&f, small, 0x48, &[], &[], ON).unwrap();
    assert_ne!(b.get_name(), a.get_name());
    assert_eq!(claimed(&b), vec![0, 8, 0x40]);
}

/// The reader dereferenced four bytes at 0x10 without claiming a field for
/// them; the held structure claims an `int4` there. A union that puts a member
/// boundary inside that access would print the load through a different member
/// than the reader's own layout does, which is what lets it move past an
/// overlapping store; it is refused, and the reader keeps its own claims.
///
/// The same two layouts with no unclaimed bytes merge, so the veto is what the
/// two outcomes differ by.
#[test]
fn a_union_may_not_split_bytes_the_reader_accessed_without_claiming() {
    let held = |f: &crate::dtype::TypeFactoryImpl| {
        vec![
            TypeField::new(0, 0, "field_0x0", charptr(f)),
            TypeField::new(1, 8, "field_0x8", f.get_base(8, type_metatype::TYPE_INT).unwrap()),
            TypeField::new(2, 0x10, "field_0x10", f.get_base(4, type_metatype::TYPE_INT).unwrap()),
            filler(f, 3, 0x14, 4),
        ]
    };
    // The reader claims nothing at 0x10: its own member there is the eight bytes
    // of filler between `field_0x8` and `field_0x18`.
    let reader = |f: &crate::dtype::TypeFactoryImpl| {
        vec![
            TypeField::new(0, 0, "field_0x0", charptr(f)),
            TypeField::new(1, 8, "field_0x8", f.get_base(8, type_metatype::TYPE_INT).unwrap()),
            filler(f, 2, 0x10, 8),
            TypeField::new(3, 0x18, "field_0x18", f.get_base(8, type_metatype::TYPE_INT).unwrap()),
        ]
    };

    let g = core_factory();
    let ga = ledger::lookup_or_mint(&g, held(&g), 0x18, &[], &[], ON).unwrap();
    let union = ledger::lookup_or_mint(&g, reader(&g), 0x20, &[], &[], ON).unwrap();
    assert_eq!(claimed(&ga), vec![0, 8, 0x10]);
    assert_eq!(
        claimed(&union),
        vec![0, 8, 0x10, 0x18],
        "with nothing unclaimed the two merge, and the union claims the held `int4` at 0x10"
    );
    assert_eq!(ledger::superseded_names(&g), vec![ga.get_name().to_string()]);

    let f = core_factory();
    let a = ledger::lookup_or_mint(&f, held(&f), 0x18, &[], &[], ON).unwrap();
    let b = ledger::lookup_or_mint(&f, reader(&f), 0x20, &[(0x10, 4)], &[], ON).unwrap();
    assert_ne!(b.get_name(), a.get_name());
    assert_eq!(
        claimed(&b),
        vec![0, 8, 0x18],
        "the union would split the access at 0x10, so the reader keeps its own claims"
    );
    assert!(
        ledger::superseded_names(&f).is_empty(),
        "nothing was merged, so the held structure still answers for its own reader"
    );
}

/// `off` is the whole rule: every lookup answers exactly as it did before the
/// option existed.
#[test]
fn off_mints_a_record_per_reader() {
    let f = core_factory();
    let (int4t, int8t, cp) =
        (f.get_base(4, type_metatype::TYPE_INT).unwrap(),
         f.get_base(8, type_metatype::TYPE_INT).unwrap(),
         charptr(&f));
    let first = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        filler(&f, 1, 8, 8),
        TypeField::new(2, 0x10, "field_0x10", Rc::clone(&int4t)),
        filler(&f, 3, 0x14, 4),
    ];
    let second = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
    ];
    let a = ledger::lookup_or_mint(&f, first, 0x18, &[], &[], OFF).unwrap();
    let b = ledger::lookup_or_mint(&f, second, 0x10, &[], &[], OFF).unwrap();
    assert_ne!(a.get_name(), b.get_name());
    assert_eq!(claimed(&a), vec![0, 0x10]);
    assert_eq!(claimed(&b), vec![0, 8]);
    assert!(ledger::superseded_names(&f).is_empty());
}

/// The limit, pinned: agreement is a shape, not an identity. `struct Job {char
/// *name; long n; long deadline; char *tag;}` and `struct Conn {char *host; long
/// n; int port; long unused;}` are different records, but a reader of each that
/// skips one member measures the SAME two claim sets as the two readers of one
/// record above -- `{0: char *, 8: long, 0x18: char *}` and `{0: char *, 8:
/// long, 0x10: int}` -- so the union is minted here too, and each reader ends up
/// declaring a field its own object does not have. Two shared claims one of
/// which is a pointer is where the floor sits, and 19 of the 63 merges measured
/// over sixteen real builds fire there;
/// `docs/features/structmerge/default-on-evaluation.md`
/// prices the class (189 of 1,105 added fields are not DWARF fields), and it is
/// why the option ships `off`.
#[test]
fn two_records_that_begin_alike_are_fused_at_the_agreement_floor() {
    let f = core_factory();
    let (int4t, int8t, cp) = (
        f.get_base(4, type_metatype::TYPE_INT).unwrap(),
        f.get_base(8, type_metatype::TYPE_INT).unwrap(),
        charptr(&f),
    );
    // A reader of `Job` that never touches `deadline`.
    let job = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        filler(&f, 2, 0x10, 8),
        TypeField::new(3, 0x18, "field_0x18", Rc::clone(&cp)),
    ];
    // A reader of `Conn` that never touches `unused`.
    let conn = vec![
        TypeField::new(0, 0, "field_0x0", Rc::clone(&cp)),
        TypeField::new(1, 8, "field_0x8", Rc::clone(&int8t)),
        TypeField::new(2, 0x10, "field_0x10", Rc::clone(&int4t)),
        filler(&f, 3, 0x14, 4),
    ];
    let a = ledger::lookup_or_mint(&f, job, 0x20, &[], &[], ON).unwrap();
    let union = ledger::lookup_or_mint(&f, conn, 0x18, &[], &[], ON).unwrap();
    assert_eq!(
        claimed(&union),
        vec![0, 8, 0x10, 0x18],
        "the two share `char *` at 0 and `long` at 8, which clears the floor"
    );
    assert_eq!(
        ledger::superseded_names(&f),
        vec![a.get_name().to_string()],
        "the Job reader is moved onto a record carrying Conn's `port` as well"
    );
}
